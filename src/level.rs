use std::path::Path;
use std::io;
use std::fs;

use ggez::{
    glam,
    event,
    graphics,
    input::{mouse::MouseButton, keyboard::{KeyInput, KeyMods}, keyboard::KeyCode},
    Context, GameResult, GameError
};

use crate::board::Board;
use crate::tile::Tile;
use crate::block::{BlockObjectMode, BlockObject, Block};
use crate::sidebar::Sidebar;
use crate::helpers::*;
use crate::constants::*;
use crate::mainstate::SceneState;

pub struct LevelState {
    board: Board,
    sidebar: Sidebar,
    held: Holding,
    mode: LevelMode
}

pub enum LevelMode {
    Building,
    Running
}

pub enum Holding {
    BlockObject{blockobject: BlockObject},
    Tile{tile: Tile},
    None
}

impl LevelState {
    pub fn new(_ctx: &mut Context) -> GameResult<LevelState> {
        let blockobjects = Self::load_level("Testlevel1")?;

        Ok(LevelState {
            board: Board::new(BOARD_POS),
            sidebar: Sidebar::new(SIDEBAR_POS, &blockobjects)?,
            held: Holding::None,
            mode: LevelMode::Building
        })
    }

    pub fn load_level(level_name: &str) -> GameResult<Vec<BlockObject>>{
        let level_path = Path::new("levels").join(level_name).with_extension("json");
        let level_string = fs::read_to_string(level_path)
            .map_err(|e: io::Error| GameError::ResourceLoadError(format!("Failed to load level data: {}", e)))?;

        let level_json: Vec<SerializedBlockObject> = serde_json::from_str(&level_string[..])
            .map_err(|e: serde_json::Error| GameError::ResourceLoadError(format!("Failed to parse level data into json: {}", e)))?;

        let mut id_counter = 1;
        let mut out: Vec<BlockObject> = Vec::new();
        for sbo in level_json.iter(){
            let mut blocks: Vec<Block> = Vec::new();
            let mode = if sbo.input {BlockObjectMode::Input} else {BlockObjectMode::Output};
            for pos in sbo.blocks.iter(){
                blocks.push(Block::new(*pos))
            }
            let mut bo = BlockObject::from_blocklist(blocks, mode);
            bo.start_counter = sbo.counter;
            bo.id = id_counter;
            id_counter += 1;
            out.push(bo);
        }

        Ok(out)
    }
}

impl SceneState for LevelState {
    fn mouse_click_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        if let LevelMode::Building = self.mode{
            self.board.mouse_click_event(ctx,button,x,y,&mut self.held)?;
            self.sidebar.mouse_click_event(ctx,button,x,y,&mut self.held)?;
        }
        Ok(())
    }
}

impl event::EventHandler for LevelState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let sim_result = self.board.update(ctx, &self.mode);
        match sim_result{
            Err(sim_err) => {
                println!("{}: {:?}", sim_err.message, sim_err.relevant_locations);
                self.mode = LevelMode::Building; // TODO: add Error state where the player can see the error message
                self.board.process_end()?;
            }
            Ok(true) => {
                println!("You Win!!!!!!");
                self.mode = LevelMode::Building; // TODO: add win state where the player can see the stats or something
                self.board.process_end()?;
            }
            _default => ()
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::new(1.0, 0.0, 1.0, 1.0));

        self.board.draw(ctx, &mut canvas, &self.mode)?;
        self.sidebar.draw(ctx, &mut canvas, &self.mode)?;

        // draw what the player is holding

        let held_image = match &mut self.held{
            Holding::Tile { tile } => Some(tile.draw(ctx, HELD_TILESIZE)?),
            Holding::BlockObject { blockobject } => Some(blockobject.draw(ctx, HELD_TILESIZE)?),
            Holding::None => None
        };
        if let Some(im) = held_image{
            canvas.draw(
                &mult_alpha(ctx, im, HELD_OBJECT_ALPHA)?,
                glam::vec2(
                    ctx.mouse.position().x - HELD_TILESIZE/2.0,
                    ctx.mouse.position().y - HELD_TILESIZE/2.0
                )
            );
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        self.board.mouse_button_down_event(ctx,button,x,y)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        self.board.mouse_button_up_event(ctx,button,x,y)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> GameResult{
        self.board.mouse_motion_event(ctx,x,y,dx,dy)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if input.keycode == Some(KeyCode::R) {
            if input.mods.contains(KeyMods::SHIFT){
                match &mut self.held{
                    Holding::BlockObject { blockobject } => blockobject.rotate_ccw(BoardPos{x:0,y:0}),
                    Holding::Tile { tile } => tile.rotate_ccw(),
                    _other => ()
                }
            }else{
                match &mut self.held{
                    Holding::BlockObject { blockobject } => blockobject.rotate_cw(BoardPos{x:0,y:0}),
                    Holding::Tile { tile } => tile.rotate_cw(),
                    _other => ()
                }
            }
        }else if input.keycode == Some(KeyCode::Return){
            match self.mode{
                LevelMode::Building => {
                    // everything has been placed
                    if self.sidebar.num_blockobjects() == self.board.num_blockobjects(){
                        self.process_start()?;
                        self.mode = LevelMode::Running;
                    }else{
                        // TODO: display this to the player somehow
                        // could use the error state if I want
                        println!("Please place all inputs and outputs first")
                    }
                }
                LevelMode::Running => {
                    self.process_end()?;
                    self.mode = LevelMode::Building;
                }
            }
        }

        if let LevelMode::Building = self.mode{
            self.board.key_down_event(ctx, input, repeated)?;
        }
        Ok(())
    }
}

impl LevelState{
    fn process_start(&mut self) -> GameResult{
        self.held = Holding::None;

        self.board.process_start()?;
        Ok(())
    }

    fn process_end(&mut self) -> GameResult{
        self.board.process_end()?;
        Ok(())
    }
}
