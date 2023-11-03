use ggez::{event, GameResult};

use constants::*;

pub mod board;
pub mod tile;
pub mod block;
pub mod constants;
pub mod helpers;
pub mod mainstate;
pub mod sidebar;
pub mod level;
pub mod asset_cache;
pub mod popup_box;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("conveyor-chaos", "r0ckwav3")
        .window_setup(ggez::conf::WindowSetup::default().title("A Manufacturing Game(TM)"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (mut ctx, event_loop) = cb.build()?;
    let state = mainstate::MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
