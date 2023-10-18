use std::time::Duration;

use ggez::{
    glam,
    event,
    graphics,
    input::{mouse::MouseButton, keyboard::{KeyInput, KeyMods}, keyboard::KeyCode},
    Context, GameResult,
};

use crate::board::Board;
use crate::tile::{Tile, TileType};
use crate::block::{BlockObjectMode, BlockObject, Block};
use crate::sidebar::Sidebar;
use crate::helpers::*;
use crate::constants::*;

pub struct MainState {
    click_time: Duration,
    board: Board,
    sidebar: Sidebar,
    held: Holding
}

pub enum Holding {
    BlockObject{blockobject: BlockObject},
    Tile{tile: Tile},
    None
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // TEMPCODE REMOVE EVENTUALLY
        // Probably read this in from a file eventually
        // let mut blockobjects = vec![
        //     BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0}), Block::new(BoardPos{x: 0, y: 1})], BlockObjectMode::Input),
        //     BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0})], BlockObjectMode::Output),
        //     BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0})], BlockObjectMode::Output)
        // ];
        // blockobjects[0].set_id(1);
        // blockobjects[1].set_id(2);
        // blockobjects[2].set_id(3);

        let mut blockobjects = vec![
            BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0})], BlockObjectMode::Input),
            BlockObject::from_blocklist(vec![
                Block::new(BoardPos{x: 0, y: 0}),
                Block::new(BoardPos{x: 1, y: 0}),
                Block::new(BoardPos{x: 1, y: 1}),
                Block::new(BoardPos{x: 2, y: 1}),
                Block::new(BoardPos{x: 1, y: 2})], BlockObjectMode::Output),
        ];
        blockobjects[0].set_id(1);
        blockobjects[1].set_id(2);

        Ok(MainState {
            board: Board::new(BOARD_POS),
            sidebar: Sidebar::new(SIDEBAR_POS, &blockobjects)?,
            click_time: Duration::ZERO,
            held: Holding::None
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.board.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::new(1.0, 0.0, 1.0, 1.0));

        self.board.draw(ctx, &mut canvas)?;
        self.sidebar.draw(ctx, &mut canvas)?;

        // draw what the player is holding

        match &mut self.held{
            Holding::Tile { tile } => {
                let im = TileType::get_image(ctx, tile.get_type(), HELD_TILESIZE, HELD_TILESIZE*GRID_THICKNESS/2.0)?;
                let mut drawparam: graphics::DrawParam = glam::vec2(
                    ctx.mouse.position().x - HELD_TILESIZE/2.0,
                    ctx.mouse.position().y - HELD_TILESIZE/2.0
                ).into();
                drawparam = drawparam.rotation(tile.get_dir().to_rot());
                drawparam = rot_fix(&mut drawparam, HELD_TILESIZE, HELD_TILESIZE)?;
                canvas.draw(
                    &mult_alpha(ctx, im, HELD_OBJECT_ALPHA)?,
                    drawparam
                );
            },
            Holding::BlockObject { blockobject } => {
                let im = blockobject.draw(ctx, HELD_TILESIZE)?;
                canvas.draw(
                    &mult_alpha(ctx, im, HELD_OBJECT_ALPHA)?,
                    glam::vec2(
                        ctx.mouse.position().x - HELD_TILESIZE/2.0,
                        ctx.mouse.position().y - HELD_TILESIZE/2.0
                    )
                );
            },
            Holding::None => ()
        };

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        if button == MouseButton::Left{
            self.click_time = ctx.time.time_since_start();
        }

        self.board.mouse_button_down_event(ctx,button,x,y)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        if button == MouseButton::Left{
            let time_since_click = ctx.time.time_since_start() - self.click_time;
            // println!("thing {}", time_since_click.as_millis());
            if time_since_click < CLICK_TIME_THRESHOLD{
                self.board.mouse_click_event(ctx,button,x,y,&mut self.held)?;
                self.sidebar.mouse_click_event(ctx,button,x,y,&mut self.held)?;
            }
        }

        self.board.mouse_button_up_event(ctx,button,x,y)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> GameResult{
        self.board.mouse_motion_event(ctx,x,y,dx,dy)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        } else if input.keycode == Some(KeyCode::R) {
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
        }
        self.board.key_down_event(ctx, input, repeated)?;
        Ok(())
    }
}
