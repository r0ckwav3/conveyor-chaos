use std::sync::mpsc;

use ggez::{
    event,
    graphics,
    input::{mouse::MouseButton, keyboard::{KeyInput, KeyMods}, keyboard::KeyCode},
    Context, GameResult, GameError
};

use crate::constants::*;
use crate::mainstate::SceneState;
use crate::helpers::*;


pub struct MainMenuState {
    scene_channel_s: mpsc::Sender<SceneMessage>
 }

 impl MainMenuState {
    pub fn new(_ctx: &mut Context, s: mpsc::Sender<SceneMessage>) -> GameResult<MainMenuState> {
        Ok(MainMenuState {
            scene_channel_s: s
        })
    }
}

impl SceneState for MainMenuState {
    fn mouse_click_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult{
        Ok(())
    }
}

impl event::EventHandler for MainMenuState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, graphics::Color::new(1.0, 0.0, 1.0, 1.0));
        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) -> GameResult{
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        if input.keycode == Some(KeyCode::Return){
            self.scene_channel_s.send(SceneMessage::EnterSceneLevel { levelname: "Testlevel2".to_string() })
                .map_err(|e| GameError::CustomError(e.to_string()))?;
        }
        Ok(())
    }
}
