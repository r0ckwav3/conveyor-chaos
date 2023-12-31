use ggez::graphics::{Color, Rect};

use super::tile::TileType;

// window and other setup
pub const BOARD_POS: Rect = Rect::new(640.0,0.0,1280.0,1280.0);
pub const SIDEBAR_POS: Rect = Rect::new(0.0,0.0,640.0,1280.0);

// graphics
pub const TILESIZE: f32 = 100.0;
pub const HELD_TILESIZE: f32 = 100.0;
pub const BLOCK_ROUNDNESS: f32 = 0.3; // 0.0 is a square, 0.5 is a circle
pub const OUTPUT_OUTLINE_WIDTH: f32 = 10.0;
pub const SIDEBAR_TILESIZE: f32 = 100.0;
pub const SIDEBAR_SPACING_X: f32 = 50.0;
pub const SIDEBAR_SPACING_Y: f32 = 50.0;
pub const SIDEBAR_MARGING_X: f32 = 50.0;
pub const SIDEBAR_MARGIN_Y: f32 = 50.0;
pub const SIDEBAR_COUNTER_CIRCLE_RAD: f32 = 35.0;
pub const POPUP_WIDTH: f32 = 800.0;
pub const POPUP_HEIGHT: f32 = 600.0;
pub const POPUP_MARGIN_X: f32 = 50.0;
pub const POPUP_MARGIN_Y: f32 = 50.0;
pub const POPUP_CORNER_RAD: f32 = 20.0;

// text
pub const SIDEBAR_COUNTER_SCALE: f32 = 48.0;
pub const POPUP_FONT: &str = "LiberationMono-Regular";
pub const POPUP_SCALE: f32 = 48.0;

// colors
pub const BOARD_BG_COLOR: Color = Color::new(106.0/255.0, 86.0/255.0, 73.0/255.0, 1.0);
pub const SIDEBAR_BG_COLOR: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const BLOCK_COLOR: Color = Color::new(0.9, 0.9, 0.9, 1.0);
// pub const BLOCK_INNER_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);
pub const OUTPUT_BLOCK_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const OUTPUT_OUTLINE_COLOR: Color = Color::new(0.7, 0.2, 0.2, 1.0);
pub const SIDEBAR_COUNTER_CIRCLE_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.6);
pub const SIDEBAR_COUNTER_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const POPUP_BG_COLOR: Color = Color::new(0.7, 0.7, 0.7, 1.0);
pub const POPUP_TEXT_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const POPUP_OVERLAY_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.5);

// alpha values
pub const BUILDING_BLOCKOBJECT_ALPHA: f32 = 0.5;
pub const RUNNING_BLOCKOBJECT_ALPHA: f32 = 0.8;
pub const HELD_OBJECT_ALPHA: f32 = 0.5;

// animation
// in seconds unless otherwise specified
pub const ANIMATION_DURATION: f32 = 0.4;

// helpers
// non-empty tile types
pub const TILETYPES: [TileType; 6] = [TileType::PushTile, TileType::PrioTile, TileType::AltTile, TileType::RotTileCCW, TileType::RotTileCW, TileType::DelayTile];
