use crate::helpers::*;

use ggez::{
    glam,
    graphics::{Image, DrawParam, Canvas},
    Context, GameResult
};

use crate::constants::*;
use crate::asset_cache;

pub struct Tile {
    tiletype: TileType,
    dir: Direction,
    pos: BoardPos,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum TileType{
    Empty,
    PushTile
}

impl Tile{
    pub fn new(tiletype: TileType, pos: BoardPos) -> Tile{
        Tile{
            tiletype,
            dir: Direction::Right,
            pos
        }
    }

    pub fn new_directional(tiletype: TileType, pos: BoardPos, dir: Direction) -> Tile{
        Tile{
            tiletype,
            dir,
            pos
        }
    }

    pub fn get_pos(&self) -> BoardPos{
        self.pos
    }

    pub fn get_x(&self) -> i32{
        self.pos.x
    }

    pub fn get_y(&self) -> i32{
        self.pos.y
    }

    pub fn get_type(&self) -> TileType{
        self.tiletype
    }

    pub fn get_dir(&self) -> Direction{
        self.dir
    }

    pub fn pos_eq(&self, other: &Tile) -> bool{
        self.pos.x==other.pos.x && self.pos.y == other.pos.y
    }

    pub fn rotate_cw(&mut self){
        self.dir = self.dir.clockwise();
    }

    pub fn rotate_ccw(&mut self){
        self.dir = self.dir.counterclockwise();
    }

    pub fn set_dir(&mut self, dir: Direction){
        self.dir = dir;
    }
}

impl TileType{
    pub fn get_image(ctx: &mut Context, tiletype: TileType, tilesize: f32, border_width: f32) -> GameResult<Image>{
        let image_name = match tiletype{
            TileType::Empty => "empty_tile",
            TileType::PushTile => "push_tile"
        };
        let base_image = asset_cache::get_image(ctx, image_name.to_string())?;

        let color_format = ctx.gfx.surface_format();
        let result_image = Image::new_canvas_image(
            ctx, color_format,
            tilesize.ceil() as u32,
            tilesize.ceil() as u32,
            1
        );
        let mut image_canvas = Canvas::from_image(ctx, result_image.clone(), TRANSPARENT_COLOR);

        image_canvas.draw(
            &base_image,
            DrawParam::default().scale(glam::vec2(
                tilesize / base_image.width() as f32,
                tilesize / base_image.height() as f32
            ))
        );

        image_canvas.finish(ctx)?;

        Ok(result_image)
    }
}
