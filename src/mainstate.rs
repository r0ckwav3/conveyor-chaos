use ggez::{
    event,
    graphics,
    input::{mouse::MouseButton, keyboard::KeyInput, keyboard::KeyCode},
    Context, GameResult,
};

use crate::board::Board;
use crate::block::{BlockObjectIO, BlockObject, Block};
use crate::sidebar::Sidebar;
use crate::helpers::*;
use crate::constants::*;

pub struct MainState {
    board: Board,
    sidebar: Sidebar
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // TEMPCODE REMOVE EVENTUALLY
        let blockobjects = vec![
            BlockObjectIO{blockobject: BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0}), Block::new(BoardPos{x: 0, y: 1})]), input: true},
            BlockObjectIO{blockobject: BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0})]), input: false},
            BlockObjectIO{blockobject: BlockObject::from_blocklist(vec![Block::new(BoardPos{x: 0, y: 0})]), input: false}
        ];
        Ok(MainState {
            board: Board::new(BOARD_POS),
            sidebar: Sidebar::new(SIDEBAR_POS, &blockobjects)?
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
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        self.board.key_down_event(ctx, input, repeated)?;
        Ok(())
    }
}
