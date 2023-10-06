use ggez::{
    event,
    glam::vec2,
    graphics::{self, Color, Mesh, DrawMode, Canvas},
    Context, GameResult,
};

use crate::tile::Tile;
use crate::block::BlockObject;

const TILE_SIZE: f32 = 100.0;
const GRID_THICKNESS: f32 = 0.1;
const BG_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
const EMPTY_COLOR: Color = Color::new(0.3, 0.3, 0.3, 1.0);

const ANIMATION_DURATION: f32 = 1.0;

enum BoardMode {
    Building,
    Processing,
    Animating
}

pub struct Board {
    canvas: BoardCanvas,
    state: BoardState
}

struct BoardCanvas {
    pos: graphics::Rect, // where to render it on the screen
    tile_size: f32,
    grid_thickness: f32, // % of tile size
    bg_color: Color,
    empty_color: Color,
    mouse_down: bool,
    offset_x: f32, // the top left corner of the screen should show what's at (offset_x, offset_y)
    offset_y: f32
}

struct BoardState {
    mode: BoardMode,
    animation_duration: f32,
    animation_timer: f32,
    tiles: Vec<Tile>,
    block_objects: Vec<BlockObject>,
}

impl Board{
    pub fn new(screenpos: graphics::Rect) -> GameResult<Board> {
        Ok(Board{
            canvas: BoardCanvas::new(screenpos)?,
            state: BoardState::new()?
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.canvas.offset_x += 2.0;
        // self.canvas.offset_y += 1.0;
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, out_canvas: &mut Canvas) -> GameResult {
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            self.canvas.pos.w as u32,
            self.canvas.pos.h as u32,
            1
        );

        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), self.canvas.bg_color);
        let empty_tile_mesh = Mesh::new_rounded_rectangle(
            ctx,
            DrawMode::fill(),
            graphics::Rect {
                x: self.canvas.tile_size * self.canvas.grid_thickness/2.0,
                y: self.canvas.tile_size * self.canvas.grid_thickness/2.0,
                w: self.canvas.tile_size * (1.0-self.canvas.grid_thickness),
                h: self.canvas.tile_size * (1.0-self.canvas.grid_thickness),
            },
            self.canvas.tile_size*self.canvas.grid_thickness,
            self.canvas.empty_color
        )?;
        // let mut instance_array = graphics::InstanceArray::new(ctx, )

        let tilex_min = (self.canvas.offset_x/self.canvas.tile_size).floor() as i32;
        let tilex_max = ((self.canvas.offset_x+self.canvas.pos.w)/self.canvas.tile_size).ceil() as i32;
        let tiley_min = (self.canvas.offset_y/self.canvas.tile_size).floor() as i32;
        let tiley_max = ((self.canvas.offset_y+self.canvas.pos.h)/self.canvas.tile_size).ceil() as i32;

        for tiley in tiley_min..tiley_max {
            for tilex in tilex_min..tilex_max {
                image_canvas.draw(
                    &empty_tile_mesh,
                    vec2(
                        tilex as f32 * self.canvas.tile_size - self.canvas.offset_x,
                        tiley as f32 * self.canvas.tile_size - self.canvas.offset_y
                    )
                );
            }
        }
        image_canvas.finish(ctx)?;

        out_canvas.draw(&image, vec2(self.canvas.pos.x, self.canvas.pos.y));
        Ok(())
    }
}

impl BoardCanvas{
    fn new(screenpos: graphics::Rect) -> GameResult<BoardCanvas> {
        Ok(BoardCanvas{
            pos: screenpos,
            tile_size: TILE_SIZE,
            grid_thickness: GRID_THICKNESS,
            bg_color: BG_COLOR,
            empty_color: EMPTY_COLOR,
            mouse_down: false,
            offset_x: 0.0,
            offset_y: 0.0
        })
    }
}


impl BoardState{
    fn new() -> GameResult<BoardState> {
        Ok(BoardState{
            mode: BoardMode::Building,
            animation_duration: ANIMATION_DURATION,
            animation_timer: 0.0,
            tiles: Vec::new(),
            block_objects: Vec::new(),
        })
    }
}
