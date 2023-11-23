use std::time::Duration;
use std::sync::mpsc;

use ggez::{
    event,
    input::{mouse::MouseButton, keyboard::KeyInput, keyboard::KeyCode},
    Context, GameResult,
};

use crate::scene_level::level::LevelState;
use crate::scene_main_menu::main_menu::MainMenuState;
use crate::constants::*;
use crate::helpers::*;

pub struct MainState {
    scene_channel_r: mpsc::Receiver<SceneMessage>,
    scene: Box<dyn SceneState>,
    click_time: Duration
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (s,r) = mpsc::channel();
        Ok(MainState{
            scene_channel_r: r,
            scene: Box::new(MainMenuState::new(ctx,s)?),
            click_time: Duration::ZERO
        })
    }

    fn process_scene_channel(&mut self, ctx: &mut Context) -> GameResult{
        while let Ok(message) = self.scene_channel_r.try_recv(){
            match message{
                SceneMessage::EnterSceneLevel { levelname } => {
                    self.scene.cleanup(ctx)?;
                    let (s,r) = mpsc::channel();
                    self.scene = Box::new(LevelState::new(ctx, s, &levelname)?);
                    self.scene_channel_r = r;
                }
                SceneMessage::EnterSceneMainMenu => {
                    self.scene.cleanup(ctx)?;
                    let (s,r) = mpsc::channel();
                    self.scene = Box::new(MainMenuState::new(ctx, s)?);
                    self.scene_channel_r = r;
                }
            }
        }
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.scene.update(ctx)?;
        self.process_scene_channel(ctx)?;
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

    fn cleanup(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
