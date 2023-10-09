use ggez::{
    glam,
    graphics,
    Context, GameResult
};

use crate::constants::*;

pub struct Tile {
    tiletype: TileType,
    dir: Direction,
    x: i32,
    y: i32
}

pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

pub enum TileType{
    PushTile
}

impl Tile{
    pub fn new(tiletype: TileType, x: i32, y: i32) -> Tile{
        Tile{
            tiletype,
            dir: Direction::Up,
            x,
            y
        }
    }

    pub fn new_directional(tiletype: TileType, x: i32, y: i32, dir: Direction) -> Tile{
        Tile{
            tiletype,
            dir,
            x,
            y
        }
    }

    pub fn pos_eq(&self, other: &Tile) -> bool{
        self.x==other.x && self.y == other.y
    }

    pub fn get_x(&self) -> i32{
        self.x
    }

    pub fn get_y(&self) -> i32{
        self.y
    }

    pub fn rotate(&mut self){
        self.dir = self.dir.clockwise();
    }
}

impl Direction {
    fn clockwise(&self) -> Direction{
        match self{
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Up    => Direction::Right,
        }
    }

    // like usual, pointing right is the default
    // due to the right/down coordinate system, these are not the normal matricies
    fn to_mat2(&self) -> glam::Mat2{
        match self{
            Direction::Right => glam::Mat2::from_cols_array(&[ 1.0, 0.0, 0.0, 1.0]),
            Direction::Down  => glam::Mat2::from_cols_array(&[ 0.0,-1.0, 1.0, 0.0]), // want <1,0> -> <0,1>
            Direction::Left  => glam::Mat2::from_cols_array(&[-1.0, 0.0, 0.0,-1.0]),
            Direction::Up    => glam::Mat2::from_cols_array(&[ 0.0, 1.0,-1.0, 0.0]),
        }
    }
}

impl TileType{
    pub fn get_image(ctx: &mut Context, tiletype: TileType, tilesize: f32, border_width: f32) -> GameResult<graphics::Image>{
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            tilesize.ceil() as u32,
            tilesize.ceil() as u32,
            1
        );

        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), graphics::Color::from_rgba(0,0,0,0));
        // make the bg rounded rectangle
        image_canvas.draw(
            &graphics::Mesh::new_rounded_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect {
                    x: border_width,
                    y: border_width,
                    w: tilesize - (2.0 * border_width),
                    h: tilesize - (2.0 * border_width),
                },
                border_width*2.0,
                TILE_BG_COLOR
            )?,
            graphics::DrawParam::default()
        );
        image_canvas.finish(ctx)?;

        Ok(image)
    }
}
