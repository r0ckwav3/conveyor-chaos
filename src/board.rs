use ggez::{
    glam,
    graphics::{self, Color, Mesh, DrawMode, Canvas},
    input::mouse::MouseButton,
    Context, GameResult,
};

use crate::tile::{Tile, TileType};
use crate::block::BlockObject;
use crate::constants::*;

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
        let mut board = Board{
            canvas: BoardCanvas::new(screenpos)?,
            state: BoardState::new()?
        };
        // TESTCODE PLEASE REMOVE
        board.state.place_tile(TileType::PushTile, 0, 0);
        board.state.place_tile(TileType::PushTile, 0, 1);
        board.state.rotate_tile(0,1);
        board.state.place_tile(TileType::PushTile, 1, 0);
        board.state.rotate_tile(1,0);
        board.state.rotate_tile(1,0);
        board.state.place_tile(TileType::PushTile, 1, 1);
        board.state.rotate_tile(1,1);
        board.state.rotate_tile(1,1);
        board.state.rotate_tile(1,1);

        Ok(board)
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
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
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), BG_COLOR);

        let empty_tile_image = TileType::get_image(
            ctx,
            TileType::PushTile,
            self.canvas.tile_size,
            self.canvas.tile_size * self.canvas.grid_thickness/2.0
        )?;

        let mut empty_tile_ia = graphics::InstanceArray::new(ctx, empty_tile_image);

        let tilex_min = (self.canvas.offset_x/self.canvas.tile_size).floor() as i32;
        let tilex_max = ((self.canvas.offset_x+self.canvas.pos.w)/self.canvas.tile_size).ceil() as i32;
        let tiley_min = (self.canvas.offset_y/self.canvas.tile_size).floor() as i32;
        let tiley_max = ((self.canvas.offset_y+self.canvas.pos.h)/self.canvas.tile_size).ceil() as i32;

        for tiley in tiley_min..tiley_max {
            for tilex in tilex_min..tilex_max {
                empty_tile_ia.push(
                    glam::vec2(
                        tilex as f32 * self.canvas.tile_size - self.canvas.offset_x,
                        tiley as f32 * self.canvas.tile_size - self.canvas.offset_y
                    ).into()
                );
            }
        }
        image_canvas.draw(&empty_tile_ia, graphics::DrawParam::default());
        image_canvas.finish(ctx)?;

        out_canvas.draw(&image, glam::vec2(self.canvas.pos.x, self.canvas.pos.y));
        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        self.canvas.mouse_button_down_event(ctx,button,x,y)?;
        Ok(())
    }

    pub fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        self.canvas.mouse_button_up_event(ctx,button,x,y)?;
        Ok(())
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> GameResult{
        self.canvas.mouse_motion_event(ctx,x,y,dx,dy)?;
        Ok(())
    }
}

impl BoardCanvas{
    fn new(screenpos: graphics::Rect) -> GameResult<BoardCanvas> {
        Ok(BoardCanvas{
            pos: screenpos,
            tile_size: TILE_SIZE,
            grid_thickness: GRID_THICKNESS,
            mouse_down: false,
            offset_x: 0.0,
            offset_y: 0.0
        })
    }

    pub fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult{
        if self.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            self.mouse_down = true;
        }
        Ok(())
    }

    pub fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32
    ) -> GameResult{
        if button == MouseButton::Left{
            self.mouse_down = false;
        }
        Ok(())
    }

    pub fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32
    ) -> GameResult{
        if self.mouse_down{
            // println!("{}, {}", dx,dy);
            self.offset_x -= dx;
            self.offset_y -= dy;
        }
        Ok(())
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

    // returns whether something got removed
    fn place_tile(&mut self, tiletype: TileType, x: i32, y: i32) -> bool{
        let mut to_remove: Option<usize> = None;
        let newtile = Tile::new(tiletype,x,y);
        for (i, tile) in self.tiles.iter().enumerate(){
            if newtile.pos_eq(tile){
                to_remove = Some(i);
            }
        }

        if let Some(i) = to_remove{
            self.tiles[i] = newtile;
        }else{
            self.tiles.push(newtile);
        }

        to_remove.is_some()
    }

    // returns false if the rotation failed (typically because there is no tile at x,y)
    fn rotate_tile(&mut self, x: i32, y: i32) -> bool{
        let mut to_rotate: Option<usize> = None;
        for (i, tile) in self.tiles.iter().enumerate(){
            if tile.get_x() == x && tile.get_y() == y{
                to_rotate = Some(i);
            }
        }

        if let Some(i) = to_rotate{
            self.tiles[i].rotate();
            true
        }else{
            false
        }
    }
}
