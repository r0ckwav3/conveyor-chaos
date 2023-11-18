use ggez::{event, GameResult};

use constants::*;

mod scene_level;
pub use scene_level::level;

pub mod constants;
pub mod helpers;
pub mod mainstate;
pub mod asset_cache;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("conveyor-chaos", "r0ckwav3")
        .window_setup(ggez::conf::WindowSetup::default().title("A Manufacturing Game(TM)"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
    let (mut ctx, event_loop) = cb.build()?;
    let state = mainstate::MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
