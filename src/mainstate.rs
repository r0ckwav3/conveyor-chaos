use std::time::Duration;

use ggez::{
    glam,
    event,
    graphics,
    input::{mouse::MouseButton, keyboard::{KeyInput, KeyMods}, keyboard::KeyCode},
    Context, GameResult,
};

use crate::board::Board;
use crate::level::LevelState;
use crate::tile::{Tile, TileType};
use crate::block::{BlockObjectMode, BlockObject, Block};
use crate::sidebar::Sidebar;
use crate::helpers::*;
use crate::constants::*;

pub struct MainState {
    scene: Box<dyn SceneState>,
    click_time: Duration
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState{
            scene: Box::new(LevelState::new(ctx)?),
            click_time: Duration::ZERO
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.scene.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.scene.draw(ctx)
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        if button == MouseButton::Left{
            self.click_time = ctx.time.time_since_start();
        }

        self.scene.mouse_button_down_event(ctx, button, x, y)
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult{
        if button == MouseButton::Left{
            let time_since_click = ctx.time.time_since_start() - self.click_time;
            if time_since_click < CLICK_TIME_THRESHOLD{
                self.scene.mouse_click_event(ctx,button,x,y)?;
            }
        }

        self.scene.mouse_button_up_event(ctx, button, x, y)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) -> GameResult{
        self.scene.mouse_motion_event(ctx,x,y,dx,dy)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, repeated: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }
        self.scene.key_down_event(ctx, input, repeated)?;
        Ok(())
    }
}

pub trait SceneState: event::EventHandler{
    fn mouse_click_event(
        &mut self,
            _ctx: &mut Context,
            _button: MouseButton,
            _x: f32,
            _y: f32
    ) -> GameResult{
        Ok(())
    }
}
