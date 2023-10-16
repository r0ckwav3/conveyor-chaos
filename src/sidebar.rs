use std::time::Duration;

use ggez::{
    glam,
    graphics::{self, Image},
    input::mouse::MouseButton,
    Context, GameResult, GameError
};

use crate::tile::TileType;
use crate::block::BlockObject;
use crate::constants::*;

pub struct Sidebar{
    pos: graphics::Rect, // I'm secretly going to render everything in here
    tilesize: f32,
    spacing_x: f32,
    spacing_y: f32,
    margin_x: f32,
    margin_y: f32,
    scroll_y: f32,
    tiles: Vec<TileType>,
    blockobjects: Vec<BlockObject>,
    rows: Vec<SidebarRow>
}

enum SidebarRow{
    TileRow{row: SidebarRowTile},
    BORow{row: SidebarRowBO}
}

struct SidebarRowTile{
    tiles: Vec<TileType>,
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
        let mut new = Sidebar{
            pos,
            tilesize: SIDEBAR_TILESIZE,
            spacing_x: SIDEBAR_SPACING_X,
            spacing_y: SIDEBAR_SPACING_Y,
            margin_x: SIDEBAR_MARGING_X,
            margin_y: SIDEBAR_MARGIN_Y,
            scroll_y: 0.0,
            // tiles: Vec::from(TILETYPES),
            tiles: vec![TileType::PushTile; 5],
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

            self.rows.push(SidebarRow::TileRow { row: temp_srt });
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
            self.rows.push(SidebarRow::BORow { row: temp_srbo });
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
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult{
        if self.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            println!("sidebar click at {},{}", x, y);
        }
        Ok(())
    }

}

// I could also implement this as a trait, which honestly seems like it's more idiomatic
// TODO: maybe do that instead
impl SidebarRow{
    fn draw(&mut self, ctx: &mut Context) -> GameResult<Image> {
        match self{
            SidebarRow::TileRow{row: subrow} => subrow.draw(ctx),
            SidebarRow::BORow{row: subrow} => subrow.draw(ctx)
        }
    }

    fn get_height(&mut self) -> GameResult<f32>{
        match self{
            SidebarRow::TileRow{row: subrow} => subrow.get_height(),
            SidebarRow::BORow{row: subrow} => subrow.get_height()
        }
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

    fn draw(&self, ctx: &mut Context) -> GameResult<Image> {
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
                &TileType::get_image(ctx, *tile, self.tilesize, 0.0)?,
                glam::vec2(xpos, 0.0)
            );
        }

        image_canvas.finish(ctx)?;
        Ok(image)
    }

    fn get_height(&mut self) -> GameResult<f32>{
        Ok(self.tilesize)
    }
}

impl SidebarRowBO{
    fn new(tilesize:f32, blockobject: BlockObject) -> SidebarRowBO{
        SidebarRowBO{
            blockobject,
            tilesize,
        }
    }

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
}
