use ggez::{
    glam,
    graphics,
    GameError,
    Context, GameResult
};

use crate::helpers::*;
use crate::constants::*;

#[derive(Copy, Clone)]
pub struct Block {
    pos: BoardPos
}

// we only use id during the building phase, to ensure only one of each input or output is in the build
// the id -1 is allowed to have multiple copies
pub struct BlockObject{
    blocks: Vec<Block>,
    image_cache: Option<graphics::Image>,
    top_left: Option<BoardPos>,
    bottom_right: Option<BoardPos>,
    mode: BlockObjectMode,
    id: i32
}

#[derive(Copy, Clone)]
pub enum BlockObjectMode{
    Input,
    Output,
    Processing
}

impl BlockObject{
    pub fn new() -> BlockObject{
        BlockObject{
            blocks: Vec::new(),
            image_cache: None,
            top_left: None,
            bottom_right: None,
            mode: BlockObjectMode::Processing,
            id: -1
        }
    }

    pub fn from_blocklist(blocks: Vec<Block>, mode: BlockObjectMode) -> BlockObject{
        BlockObject{
            blocks,
            image_cache: None,
            top_left: None,
            bottom_right: None,
            mode,
            id: -1
        }
    }

    // pub fn merge(a:BlockObject, b:BlockObject) -> BlockObject{}

    pub fn shift(&mut self, dx: i32, dy: i32){
        for block in self.blocks.iter_mut(){
            block.shift(dx, dy);
        }
        let _ = self.generate_bounds();
    }

    pub fn draw(&mut self, ctx: &mut Context, tilesize: f32) -> GameResult<graphics::Image>{
        if let Some(image) = self.image_cache.clone(){
            Ok(image)
        }else{
            self.generate_image(ctx, tilesize)?;
            let image = self.image_cache.clone().expect("Failed to cache image");
            Ok(image)
        }
    }

    pub fn get_top_left(&mut self) -> GameResult<BoardPos>{
        if let Some(pos) = self.top_left{
            Ok(pos)
        }else{
            self.generate_bounds()?;
            let pos = self.top_left.expect("Failed to cache bounds");
            Ok(pos)
        }
    }

    pub fn get_bottom_right(&mut self) -> GameResult<BoardPos>{
        if let Some(pos) = self.bottom_right{
            Ok(pos)
        }else{
            self.generate_bounds()?;
            let pos = self.bottom_right.expect("Failed to cache bounds");
            Ok(pos)
        }
    }

    pub fn get_id(&self) -> i32{
        return self.id;
    }

    pub fn set_id(&mut self, id: i32){
        return self.id = id;
    }

    fn generate_bounds(&mut self) -> GameResult{
        if self.blocks.len() == 0{
            return Err(GameError::RenderError("Cannot render blockobject with no blocks".to_string()));
        }
        // find the bounds
        let mut xmin = self.blocks[0].pos.x;
        let mut ymin = self.blocks[0].pos.y;
        let mut xmax = self.blocks[0].pos.x;
        let mut ymax = self.blocks[0].pos.y;
        for block in self.blocks.iter(){
            if block.pos.x < xmin{
                xmin = block.pos.x;
            }
            if block.pos.y < ymin{
                ymin = block.pos.y;
            }
            if block.pos.x > xmax{
                xmax = block.pos.x;
            }
            if block.pos.y > ymax{
                ymax = block.pos.y;
            }
        }

        self.top_left = Some(BoardPos{x:xmin, y:ymin});
        self.bottom_right = Some(BoardPos{x:xmax, y:ymax});
        Ok(())
    }

    // most of the time draw will do this automatically, but you can call it manually if you want,
    fn generate_image(&mut self, ctx: &mut Context, tilesize: f32) -> GameResult{
        if self.blocks.len() == 0{
            return Err(GameError::RenderError("Cannot render blockobject with no blocks".to_string()));
        }
        if let None = self.top_left{
            self.generate_bounds()?;
        }

        let br = self.bottom_right.expect("Failed to cache bounds");
        let tl = self.top_left.expect("Failed to cache bounds");

        let grid_w = 1 + br.x - tl.x;
        let grid_h = 1 + br.y - tl.y;
        let canvas_w = tilesize*grid_w as f32;
        let canvas_h = tilesize*grid_h as f32;

        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            canvas_w.ceil() as u32,
            canvas_h.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);
        // find the locations of all blocks
        let mut block_grid: Vec<Vec<bool>> = Vec::new();
        block_grid.resize(grid_h as usize, Vec::new());
        for i in 0..block_grid.len(){
            block_grid[i].resize(grid_w as usize, false);
        }
        for block in self.blocks.iter(){
            block_grid[(block.pos.y-tl.y) as usize][(block.pos.x-tl.x) as usize] = true;
        }

        // this does a bit more computation than strictly neccesary
        for x in 0..grid_w{
            for y in 0..grid_h{
                // we're reusing a lot of data between each loop here
                let mut nhood = [[false; 3]; 3];
                for x2 in x-1..x+2{
                    for y2 in y-1..y+2{
                        if x2 < 0 || x2 >= grid_w || y2 < 0 || y2 >= grid_h{
                            continue
                        }
                        nhood[(y2-y+1) as usize][(x2-x+1) as usize] = block_grid[y2 as usize][x2 as usize];
                    }
                }
                // println!("nhood of {}, {}: {:?}", x, y, nhood);
                let block_image = match self.mode{
                    BlockObjectMode::Output => Block::draw_output(ctx, tilesize, nhood),
                    _default => Block::draw(ctx, tilesize, nhood),
                }?;
                image_canvas.draw(
                    &block_image,
                    glam::vec2(x as f32 * tilesize, y as f32 * tilesize)
                )
            }
        }

        image_canvas.finish(ctx)?;

        self.image_cache = Some(image);
        Ok(())
    }

    // since we need a context and tilesize to generate the image, we just clear it until we draw again
    // if you only change the position (e.g. shift) you can use generate_bounds immediately
    fn reset_cache(&mut self){
        self.top_left = None;
        self.bottom_right = None;
        self.image_cache = None;
    }

    pub fn has_overlap(&mut self, other: &mut Self) -> bool{
        let zeropos = BoardPos{x:0,y:0};
        let stl = self.get_top_left().unwrap_or(zeropos);
        let sbr = self.get_bottom_right().unwrap_or(zeropos);
        let otl = other.get_top_left().unwrap_or(zeropos);
        let obr = other.get_bottom_right().unwrap_or(zeropos);
        let xoverlap = sbr.x >= otl.x || obr.x >= stl.x;
        let yoverlap = sbr.y >= otl.y || obr.y >= stl.y;

        // try to avoid this since it's o(n^2)
        if xoverlap && yoverlap{
            for sblock in self.blocks.iter(){
                for oblock in other.blocks.iter(){
                    if sblock.pos == oblock.pos{
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn rotate_cw(&mut self, around: BoardPos){
        for block in self.blocks.iter_mut(){
            block.rotate_cw(around);
        }
        self.reset_cache();
    }

    pub fn rotate_ccw(&mut self, around: BoardPos){
        for block in self.blocks.iter_mut(){
            block.rotate_ccw(around);
        }
        self.reset_cache();
    }
}

impl Clone for BlockObject{
    fn clone(&self) -> BlockObject{
        return BlockObject{
            blocks: self.blocks.clone(),
            image_cache: None,
            top_left: None,
            bottom_right: None,
            mode: self.mode,
            id: self.id
        }
    }
}

impl Block{
    pub fn new(pos: BoardPos) -> Block{
        Block{
            pos
        }
    }

    pub fn shift(&mut self, dx: i32, dy: i32){
        self.pos.x += dx;
        self.pos.y += dy;
    }

    pub fn rotate_ccw(&mut self, around: BoardPos){
        // x_rel = (y_rel), y_rel = -(x_rel)
        // x_rel = x-x_around, y_rel = y-y_around
        // x-x_around = (y-y_around), y-y_around = -(x-x_around)
        // x = y-y_around+x_around, y = -x+x_around + y_around
        self.pos = BoardPos{
            x: self.pos.y - around.y + around.x,
            y: around.x - self.pos.x + around.y,
        }
    }

    pub fn rotate_cw(&mut self, around: BoardPos){
        self.pos = BoardPos{
            x: around.y - self.pos.y + around.x,
            y: self.pos.x - around.x + around.y,
        }
    }

    pub fn draw(ctx: &mut Context, tilesize: f32, nhood: [[bool; 3]; 3]) -> GameResult<graphics::Image>{
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            tilesize.ceil() as u32,
            tilesize.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);

        // TODO, we generate these exact meshes a bunch of times per frame
        // I wonder if we can cache them
        let base_mesh = graphics::Mesh::new_rounded_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize, tilesize),
            tilesize*BLOCK_ROUNDNESS,
            BLOCK_COLOR
        )?;

        let clear_base_mesh = graphics::Mesh::new_rounded_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize, tilesize),
            tilesize*BLOCK_ROUNDNESS,
            TRANSPARENT_COLOR
        )?;

        let corner_mesh = graphics::Mesh::new_rectangle(
            ctx, graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, tilesize*BLOCK_ROUNDNESS, tilesize*BLOCK_ROUNDNESS),
            BLOCK_COLOR
        )?;

        let corner_offset = tilesize*(1.0-BLOCK_ROUNDNESS);

        if nhood[1][1]{ // if the center tile exists
            // draw the base
            image_canvas.draw(&base_mesh, graphics::DrawParam::default());
            // corners
            if nhood[0][0] || nhood[1][0] || nhood[0][1]{ // top left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, 0.0));
            }
            if nhood[0][2] || nhood[0][1] || nhood[1][2]{ // top right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, 0.0));
            }
            if nhood[2][2] || nhood[1][2] || nhood[2][1]{ // bottom right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, corner_offset));
            }
            if nhood[2][0] || nhood[1][0] || nhood[2][1]{ // bottom left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, corner_offset));
            }
        }else{ // center tile does not exist
            // corners
            if nhood[1][0] && nhood[0][1]{ // top left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, 0.0));
            }
            if nhood[0][1] && nhood[1][2]{ // top right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, 0.0));
            }
            if nhood[1][2] && nhood[2][1]{ // bottom right
                image_canvas.draw(&corner_mesh, glam::vec2(corner_offset, corner_offset));
            }
            if nhood[1][0] && nhood[2][1]{ // bottom left
                image_canvas.draw(&corner_mesh, glam::vec2(0.0, corner_offset));
            }
            // subtract the base
            image_canvas.set_blend_mode(graphics::BlendMode::MULTIPLY);
            image_canvas.draw(&clear_base_mesh, graphics::DrawParam::default());
        }

        image_canvas.finish(ctx)?;

        Ok(image)
    }

    pub fn draw_output(ctx: &mut Context, tilesize: f32, nhood: [[bool; 3]; 3]) -> GameResult<graphics::Image>{
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            tilesize.ceil() as u32,
            tilesize.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);

        if nhood[1][1]{ // if the center tile exists
            let base_mesh = graphics::Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, tilesize, tilesize),
                OUTPUT_BLOCK_COLOR
            )?;

            // top edge
            let edge_mesh = graphics::Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, tilesize, OUTPUT_OUTLINE_WIDTH),
                OUTPUT_OUTLINE_COLOR
            )?;

            let corner_mesh = graphics::Mesh::new_rectangle(
                ctx, graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, OUTPUT_OUTLINE_WIDTH, OUTPUT_OUTLINE_WIDTH),
                OUTPUT_OUTLINE_COLOR
            )?;
            let corner_offset = tilesize - OUTPUT_OUTLINE_WIDTH;

            let pi = std::f32::consts::PI;

            // draw the base
            image_canvas.draw(&base_mesh, graphics::DrawParam::default());

            // sides
            if !nhood[0][1]{ // up
                image_canvas.draw(&edge_mesh, graphics::DrawParam::new().dest(glam::vec2(0.0, 0.0)).rotation(0.0 * pi));
            }
            if !nhood[1][2]{ // right
                image_canvas.draw(&edge_mesh, graphics::DrawParam::new().dest(glam::vec2(tilesize, 0.0)).rotation(0.5 * pi));
            }
            if !nhood[2][1]{ // down
                image_canvas.draw(&edge_mesh, graphics::DrawParam::new().dest(glam::vec2(tilesize, tilesize)).rotation(1.0 * pi));
            }
            if !nhood[1][0]{ // left
                image_canvas.draw(&edge_mesh, graphics::DrawParam::new().dest(glam::vec2(0.0, tilesize)).rotation(1.5 * pi));
            }
            // corners
            if !nhood[0][0]{
                image_canvas.draw(&corner_mesh, graphics::DrawParam::new().dest(glam::vec2(0.0, 0.0)));
            }
            if !nhood[0][2]{
                image_canvas.draw(&corner_mesh, graphics::DrawParam::new().dest(glam::vec2(corner_offset, 0.0)));
            }
            if !nhood[2][0]{
                image_canvas.draw(&corner_mesh, graphics::DrawParam::new().dest(glam::vec2(0.0, corner_offset)));
            }
            if !nhood[2][2]{
                image_canvas.draw(&corner_mesh, graphics::DrawParam::new().dest(glam::vec2(corner_offset, corner_offset)));
            }
        }
        image_canvas.finish(ctx)?;

        Ok(image)
    }
}
