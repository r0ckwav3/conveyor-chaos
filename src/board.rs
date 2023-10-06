use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

use crate::tile::Tile;
use crate::block::BlockObject;

enum BoardMode {
    Building,
    Processing,
    Animating
}

pub struct Board {
    canvas: BoardCanvas,
    state: BoardState
}

struct BoardCanvas {
    pos: graphics::Rect,
    tile_size: f32,
    grid_thickness: f32,
    bg_color: Color,
    grid_color: Color
}

struct BoardState {
    mode: BoardMode,
    animation_duration: f32,
    animation_timer: f32,
    tiles: Vec<Tile>,
    block_objects: Vec<BlockObject>,
}
