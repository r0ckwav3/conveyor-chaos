use std::time::Duration;

use ggez::graphics::Color;

// window and other setup
pub const SCREEN_SIZE: (f32, f32) = (1920.0,1280.0);

// UX
pub const CLICK_TIME_THRESHOLD: Duration = Duration::from_millis(250);

// colors
pub const TRANSPARENT_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);
