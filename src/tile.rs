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

#[derive(Clone, Copy)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum TileType{
    Empty,
    PushTile
}

impl Tile{
    pub fn new(tiletype: TileType, x: i32, y: i32) -> Tile{
        Tile{
            tiletype,
            dir: Direction::Right,
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

    pub fn get_x(&self) -> i32{
        self.x
    }

    pub fn get_y(&self) -> i32{
        self.y
    }

    pub fn get_type(&self) -> TileType{
        self.tiletype
    }

    pub fn get_dir(&self) -> Direction{
        self.dir
    }

    pub fn pos_eq(&self, other: &Tile) -> bool{
        self.x==other.x && self.y == other.y
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

    // convert to a radian counterclockwise rotation
    pub fn to_rot(&self) -> f32{
        let pi = std::f32::consts::PI;
        match self{
            Direction::Right => 0.0,
            Direction::Down  => pi*0.5,
            Direction::Left  => pi,
            Direction::Up    => pi*1.5
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
        // draw the sepecific tile types
        if tiletype == TileType::PushTile{
            image_canvas.draw(
                &graphics::Mesh::new_polyline(
                    ctx,
                    graphics::DrawMode::fill(),
                    &[
                        glam::vec2(tilesize*0.3,tilesize*0.2),
                        glam::vec2(tilesize*0.4,tilesize*0.2),
                        glam::vec2(tilesize*0.8,tilesize*0.5),
                        glam::vec2(tilesize*0.4,tilesize*0.8),
                        glam::vec2(tilesize*0.3,tilesize*0.8),
                        glam::vec2(tilesize*0.7,tilesize*0.5)
                    ],
                    graphics::Color::new(1.0,0.0,1.0,1.0)
                )?,
                graphics::DrawParam::default()
            );
        }

        image_canvas.finish(ctx)?;

        Ok(image)
    }
}
