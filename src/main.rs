//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam,
    graphics,
    input::{mouse::MouseButton, keyboard::KeyInput, keyboard::KeyCode},
    Context, GameResult,
};

use constants::*;

pub mod board;
pub mod tile;
pub mod block;
pub mod constants;
pub mod helpers;

struct MainState {
    board: board::Board
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            board: board::Board::new(BOARD_POS)?
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
        // canvas.draw(&block::Block::draw(ctx, TILE_SIZE, [[true,true,true],[true,false,true],[true,false,true]])?, glam::vec2(100.0,100.0));

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
        self.board.key_down_event(ctx, input, repeated);
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("manufacturing", "r0ckwav3")
        .window_setup(ggez::conf::WindowSetup::default().title("A Manufacturing Game(TM)"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
