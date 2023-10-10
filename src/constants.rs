use ggez::graphics::{Color, Rect};

use crate::tile::TileType;

// window
pub const SCREEN_SIZE: (f32, f32) = (1920.0,1280.0);
pub const BOARD_POS: Rect = Rect::new(640.0,0.0,1280.0,1280.0);

// graphics
pub const TILE_SIZE: f32 = 100.0;
pub const GRID_THICKNESS: f32 = 0.1;
pub const BG_COLOR: Color = Color::new(0.2, 0.2, 0.2, 1.0);
pub const TILE_BG_COLOR: Color = Color::new(0.3, 0.3, 0.3, 1.0);

// animation
pub const ANIMATION_DURATION: f32 = 1.0;

// helpers
// non-empty tile types
pub const TILETYPES: [TileType; 1] = [TileType::PushTile];
