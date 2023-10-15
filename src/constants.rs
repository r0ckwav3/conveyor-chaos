use std::time::Duration;

use ggez::graphics::{Color, Rect};

use crate::tile::TileType;

// window and other setup
pub const SCREEN_SIZE: (f32, f32) = (1920.0,1280.0);
pub const BOARD_POS: Rect = Rect::new(640.0,0.0,1280.0,1280.0);
pub const SIDEBAR_POS: Rect = Rect::new(0.0,0.0,640.0,1280.0);

// UX
pub const CLICK_TIME_THRESHOLD: Duration = Duration::from_millis(250);

// graphics
pub const TILE_SIZE: f32 = 100.0;
pub const GRID_THICKNESS: f32 = 0.1;
pub const BLOCK_ROUNDNESS: f32 = 0.3; // 0.0 is a square, 0.5 is a circle

// colors
pub const BOARD_BG_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
pub const SIDEBAR_BG_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const TILE_BG_COLOR: Color = Color::new(0.3, 0.3, 0.3, 1.0);
pub const TILE_SYMBOL_COLOR: Color = Color::new(0.1, 0.9, 1.0, 1.0);
pub const BLOCK_COLOR: Color = Color::new(0.9, 0.9, 0.9, 1.0);
pub const BLOCK_INNER_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
pub const TRANSPARENT_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.0);

// animation
pub const ANIMATION_DURATION: f32 = 1.0;

// helpers
// non-empty tile types
pub const TILETYPES: [TileType; 1] = [TileType::PushTile];

// Sidebar
pub const SIDEBAR_TILESIZE: f32 = 100.0;
pub const SIDEBAR_SPACING_X: f32 = 50.0;
pub const SIDEBAR_SPACING_Y: f32 = 50.0;
pub const SIDEBAR_MARGING_X: f32 = 50.0;
pub const SIDEBAR_MARGIN_Y: f32 = 50.0;
