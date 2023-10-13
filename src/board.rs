use std::collections::HashMap;
use std::time::Duration;

use ggez::GameError;
use ggez::{
    glam,
    graphics,
    input::mouse::MouseButton,
    input::keyboard::{KeyInput, KeyCode, KeyMods},
    Context, GameResult,
};

use crate::tile::{Tile, TileType};
use crate::block::{Block, BlockObject};
use crate::constants::*;
use crate::helpers::*;

enum BoardMode {
    Building,
    Processing,
    Animating
}

pub struct Board {
    mouse_down: bool,
    click_time: Duration,
    canvas: BoardCanvas,
    state: BoardState
}

struct BoardCanvas {
    pos: graphics::Rect, // where to render it on the screen
    tile_size: f32,
    grid_thickness: f32, // % of tile size
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
            mouse_down: false,
            click_time: Duration::ZERO,
            canvas: BoardCanvas::new(screenpos)?,
            state: BoardState::new()?
        };
        // TEST CODE PLEASE REMOVE
        let test_bo = BlockObject::from_blocklist(vec![
            Block::new(BoardPos{x: 0, y: 0}),
            Block::new(BoardPos{x: 1, y: 0}),
            Block::new(BoardPos{x: 2, y: 0}),
            Block::new(BoardPos{x: 3, y: 0}),
            Block::new(BoardPos{x: 3, y: 1}),
            Block::new(BoardPos{x: 3, y: 2}),
            Block::new(BoardPos{x: 3, y: 3}),
            Block::new(BoardPos{x: 2, y: 3}),
            Block::new(BoardPos{x: 1, y: 3}),
            Block::new(BoardPos{x: 1, y: 2}),
            Block::new(BoardPos{x: 0, y: 2}),
            Block::new(BoardPos{x: 0, y: 1}),
            Block::new(BoardPos{x: 0, y: -1}),
        ]);
        board.state.block_objects.push(test_bo);

        Ok(board)
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, out_canvas: &mut graphics::Canvas) -> GameResult {
        let color_format = ctx.gfx.surface_format();
        let image = graphics::Image::new_canvas_image(
            ctx, color_format,
            self.canvas.pos.w as u32,
            self.canvas.pos.h as u32,
            1
        );
        let mut image_canvas = graphics::Canvas::from_image(ctx, image.clone(), BG_COLOR);

        // empty tiles
        // TODO: don't render empty tiles under filled tiles
        let empty_tile_image = TileType::get_image(
            ctx,
            TileType::Empty,
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

        // filled tiles
        let mut ia_map = HashMap::new();
        for tiletype in TILETYPES{
            let empty_tile_image = TileType::get_image(
                ctx,
                TileType::PushTile,
                self.canvas.tile_size,
                self.canvas.tile_size * self.canvas.grid_thickness/2.0
            )?;

            let empty_tile_ia = Box::new(graphics::InstanceArray::new(ctx, empty_tile_image));
            ia_map.insert(tiletype, empty_tile_ia);
        }
        for tile in self.state.tiles.iter(){
            if tile.get_x() >= tilex_min && tile.get_x() <= tilex_max &&
                tile.get_y() >= tiley_min && tile.get_y() <= tiley_max{
                let screenpos: graphics::DrawParam = glam::vec2(
                    tile.get_x() as f32 * self.canvas.tile_size - self.canvas.offset_x,
                    tile.get_y() as f32 * self.canvas.tile_size - self.canvas.offset_y
                ).into();

                let mut draw_param = screenpos.rotation(tile.get_dir().to_rot());
                draw_param = rot_fix(&mut draw_param,self.canvas.tile_size, self.canvas.tile_size)?;

                if let Some(temp_ia) = ia_map.get_mut(&tile.get_type()){
                    temp_ia.push(draw_param);
                }else{
                    return Err(GameError::CustomError(format!("Failed to find InstanceArray for tiletype {:?}", tile.get_type())))
                }
            }
        }
        for tiletype in TILETYPES{
            if let Some(temp_ia) = ia_map.get(&tiletype){
                image_canvas.draw(temp_ia.as_ref(), graphics::DrawParam::default());
            }else{
                return Err(GameError::CustomError(format!("Failed to find InstanceArray for tiletype {:?}", tiletype)))
            }
        }

        // blocks
        for block_object in self.state.block_objects.iter_mut(){
            let bo_image = block_object.draw(ctx, self.canvas.tile_size)?;
            let bo_pos = block_object.get_top_left()?;
            let screenpos: graphics::DrawParam = glam::vec2(
                bo_pos.x as f32 * self.canvas.tile_size - self.canvas.offset_x,
                bo_pos.y as f32 * self.canvas.tile_size - self.canvas.offset_y
            ).into();
            image_canvas.draw(&bo_image, screenpos);
        }


        image_canvas.finish(ctx)?;

        out_canvas.draw(&image, glam::vec2(self.canvas.pos.x, self.canvas.pos.y));
        Ok(())
    }

    pub fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult{
        if self.canvas.pos.contains(glam::vec2(x, y)) && button == MouseButton::Left{
            self.mouse_down = true;
            self.click_time = ctx.time.time_since_start();
        }
        Ok(())
    }

    pub fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) -> GameResult{
        if button == MouseButton::Left{
            self.mouse_down = false;
            let time_since_click = ctx.time.time_since_start() - self.click_time;
            // println!("thing {}", time_since_click.as_millis());
            if time_since_click < CLICK_TIME_THRESHOLD{
                self.state.toggle_tile(self.canvas.screen_pos_to_tile(x, y))
            }
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
            self.canvas.offset_x -= dx;
            self.canvas.offset_y -= dy;
        }
        Ok(())
    }

    pub fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        if input.keycode == Some(KeyCode::R) {
            let mouse_pos = ctx.mouse.position();
            let tile_pos = self.canvas.screen_pos_to_tile(mouse_pos.x, mouse_pos.y);
            if input.mods.contains(KeyMods::SHIFT){
                self.state.rotate_tile_ccw(tile_pos);
            }else{
                self.state.rotate_tile(tile_pos);
            }
        }
        Ok(())
    }
}

impl BoardCanvas{
    fn new(screenpos: graphics::Rect) -> GameResult<BoardCanvas> {
        Ok(BoardCanvas{
            pos: screenpos,
            tile_size: TILE_SIZE,
            grid_thickness: GRID_THICKNESS,
            offset_x: 0.0,
            offset_y: 0.0
        })
    }

    fn screen_pos_to_tile(&self, x: f32, y: f32) -> BoardPos{
        let true_x = x + self.offset_x - self.pos.x;
        let true_y = y + self.offset_y - self.pos.y;
        BoardPos{
            x: (true_x/self.tile_size).floor() as i32,
            y: (true_y/self.tile_size).floor() as i32
        }
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

    // find the index of the tile at a position
    // returns None if there is no tile
    fn find_tile(&mut self, pos: BoardPos) -> Option<usize>{
        let mut found_index : Option<usize> = None;
        for (i, tile) in self.tiles.iter().enumerate(){
            if tile.get_x() == pos.x && tile.get_y() == pos.y{
                found_index = Some(i);
            }
        }

        found_index
    }

    fn place_tile(&mut self, tiletype: TileType, pos: BoardPos){
        let newtile = Tile::new(tiletype, pos);
        let to_remove: Option<usize> = self.find_tile(pos);

        if let Some(i) = to_remove{
            self.tiles[i] = newtile;
        }else{
            self.tiles.push(newtile);
        }
    }

    fn rotate_tile(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles[i].rotate();
        }
    }

    fn rotate_tile_ccw(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles[i].rotate_ccw();
        }
    }

    // returns false if the tile is not removed (typically because there is no tile at x,y)
    fn remove_tile(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles.remove(i);
        }
    }

    // cycles between push and empty tiles
    fn toggle_tile(&mut self, pos: BoardPos){
        if let Some(i) = self.find_tile(pos){
            self.tiles.remove(i);
        }else{
            self.tiles.push(Tile::new(TileType::PushTile,pos));
        }
    }
}
