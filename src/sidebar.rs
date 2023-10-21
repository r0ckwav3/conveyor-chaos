use ggez::{
    glam,
    graphics::{self, Image},
    input::mouse::MouseButton,
    Context, GameResult
};

use crate::level::Holding;
use crate::tile::{Tile, TileType};
use crate::block::BlockObject;
use crate::constants::*;
use crate::helpers::*;

pub struct Sidebar{
    pos: graphics::Rect, // I'm secretly going to render everything in here
    tilesize: f32,
    spacing_x: f32,
    spacing_y: f32,
    margin_x: f32,
    margin_y: f32,
    scroll_y: f32,
    tiles: Vec<Tile>,
    blockobjects: Vec<BlockObject>,
    rows: Vec<Box<dyn SidebarRow>>
}

pub trait SidebarRow{
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Image>;
    fn get_height(&mut self) -> GameResult<f32>;
    fn get_held(&mut self, x: f32, y: f32) -> GameResult<Holding>;
}

struct SidebarRowTile{
    tiles: Vec<Tile>,
    tilesize: f32,
    padding: f32 // on the side of each tile
}

// for now I'm going to have each BORow only contain one BlockObject
struct SidebarRowBO{
    blockobject: BlockObject,
    tilesize: f32
}

impl Sidebar{
    pub fn new(pos: graphics::Rect, bos: &Vec<BlockObject>) -> GameResult<Sidebar>{
        let tiles = TILETYPES.iter().map(|tt: &TileType| {
            Tile::new(*tt, BoardPos{x:0, y:0})
        }).collect();

        let mut new = Sidebar{
            pos,
            tilesize: SIDEBAR_TILESIZE,
            spacing_x: SIDEBAR_SPACING_X,
            spacing_y: SIDEBAR_SPACING_Y,
            margin_x: SIDEBAR_MARGING_X,
            margin_y: SIDEBAR_MARGIN_Y,
            scroll_y: 0.0,
            tiles,
            blockobjects: bos.clone(),
            rows: Vec::new()
        };

        new.init_rows()?;
        Ok(new)
    }

    pub fn set_bos(&mut self, bos: &Vec<BlockObject>) -> GameResult{
        self.blockobjects = bos.clone();
        self.init_rows()?;
        Ok(())
    }

    fn init_rows(&mut self) -> GameResult{
        let width = self.pos.w - self.margin_x*2.0;

        // tile rows
        let tiles_per_row = ((width+self.spacing_x)/(self.tilesize + self.spacing_x)).floor() as usize;
        if tiles_per_row == 0{
            // TODO: do something useful here like scaling down the tiles
            panic!("sidebar is too small to fit tiles, fix this with an actual fix and not a panic")
        }

        let mut i = 0;
        while i<self.tiles.len(){
            let mut temp_srt = SidebarRowTile::new(self.tilesize);
            for _ in 0..tiles_per_row{
                if i<self.tiles.len(){
                    temp_srt.tiles.push(self.tiles[i].clone());
                    i += 1;
                }
            }
            let len = temp_srt.tiles.len();
            temp_srt.padding = (width - ((len as f32) * self.tilesize)) / ((len*2) as f32);

            self.rows.push(Box::new(temp_srt));
        }

        // blockobject rows
        for bo in self.blockobjects.iter_mut(){
            let botl = bo.get_top_left()?;
            let bobr = bo.get_bottom_right()?;
            let bowidth = 1 + bobr.x - botl.x;
            // if the block object won't fit, use a smaller tile size
            let tilesize;
            if self.tilesize <= width/(bowidth as f32){
                tilesize = self.tilesize;
            }else{
                tilesize = width/(bowidth as f32);
            }

            let temp_srbo = SidebarRowBO::new(tilesize, bo.clone());
            self.rows.push(Box::new(temp_srbo));
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, out_canvas: &mut graphics::Canvas) -> GameResult {
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            self.pos.w as u32,
            self.pos.h as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), SIDEBAR_BG_COLOR);

        let mut curr_y = self.margin_y;
        for row in self.rows.iter_mut(){
            let row_image = row.draw(ctx)?;

            image_canvas.draw(
                &row_image,
                glam::vec2(self.margin_x, curr_y-self.scroll_y)
            );

            curr_y += row.get_height()? + self.spacing_y;
        }

        image_canvas.finish(ctx)?;

        out_canvas.draw(&image, glam::vec2(self.pos.x, self.pos.y));
        Ok(())
    }

    pub fn mouse_click_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
        held: &mut Holding
    ) -> GameResult{
        if self.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            // find the relevant row
            let mut curr_y = self.margin_y;
            let mut chosen_row = None;
            for row in self.rows.iter_mut(){
                if y > curr_y && y < curr_y + row.get_height()?{
                    chosen_row = Some(row);
                    break;
                }
                curr_y += row.get_height()? + self.spacing_y;
            }
            *held = match chosen_row{
                Some(row) => row.get_held(x-self.margin_x, y-curr_y)?,
                None => Holding::None
            }
        }
        Ok(())
    }
}

impl SidebarRowTile{
    fn new(tilesize: f32) -> SidebarRowTile{
        SidebarRowTile{
            tiles: Vec::new(),
            tilesize,
            padding: 0.0
        }
    }
}

impl SidebarRow for SidebarRowTile{
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Image> {
        let color_format = ctx.gfx.surface_format();
        let width = (self.tilesize + 2.0*self.padding) * self.tiles.len() as f32;
        let height = self.tilesize;

        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            width.ceil() as u32,
            height.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);

        for (i, tile) in self.tiles.iter().enumerate(){
            let xpos = self.padding + (self.tilesize + (2.0 * self.padding)) * i as f32;
            image_canvas.draw(
                &tile.draw(ctx, self.tilesize)?,
                glam::vec2(xpos, 0.0)
            );
        }

        image_canvas.finish(ctx)?;
        Ok(image)
    }

    fn get_height(&mut self) -> GameResult<f32>{
        Ok(self.tilesize)
    }

    fn get_held(&mut self, x: f32, _y: f32) -> GameResult<Holding>{
        for (i, tile) in self.tiles.iter().enumerate(){
            let xpos = self.padding + (self.tilesize + (2.0 * self.padding)) * i as f32;
            if x >= xpos && x <= xpos + self.tilesize{
                return Ok(Holding::Tile { tile: tile.clone() })
            }
        }
        Ok(Holding::None)
    }
}

impl SidebarRowBO{
    fn new(tilesize:f32, blockobject: BlockObject) -> SidebarRowBO{
        SidebarRowBO{
            blockobject,
            tilesize,
        }
    }
}

impl SidebarRow for SidebarRowBO{
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Image> {
        let color_format = ctx.gfx.surface_format();
        let botl = self.blockobject.get_top_left()?;
        let bobr = self.blockobject.get_bottom_right()?;
        let bowidth = 1 + bobr.x - botl.x;
        let boheight = 1 + bobr.y - botl.y;

        let width = self.tilesize * bowidth as f32;
        let height = self.tilesize * boheight as f32;

        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            width.ceil() as u32,
            height.ceil() as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), TRANSPARENT_COLOR);

        image_canvas.draw(
            &self.blockobject.draw(ctx, self.tilesize)?,
            glam::vec2(0.0, 0.0)
        );

        image_canvas.finish(ctx)?;
        Ok(image)
    }

    fn get_height(&mut self) -> GameResult<f32>{
        let botl = self.blockobject.get_top_left()?;
        let bobr = self.blockobject.get_bottom_right()?;
        let boheight = 1 + bobr.y - botl.y;

        Ok(self.tilesize * boheight as f32)
    }

    fn get_held(&mut self, x: f32, _y: f32) -> GameResult<Holding>{
        let botl = self.blockobject.get_top_left()?;
        let bobr = self.blockobject.get_bottom_right()?;
        let bowidth = 1 + bobr.x - botl.x;

        let width = self.tilesize * bowidth as f32;

        if x<width {
            Ok(Holding::BlockObject { blockobject: self.blockobject.clone() })
        }else{
            Ok(Holding::None)
        }
    }
}
