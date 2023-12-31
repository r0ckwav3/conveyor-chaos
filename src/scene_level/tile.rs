use super::helpers::*;

use ggez::{
    graphics::Image,
    Context, GameResult
};

use crate::asset_cache;

#[derive(Clone)]
pub struct Tile {
    tiletype: TileType,
    dir: Direction,
    pos: BoardPos,
    orinal_dir: Direction // only used by alternating tiles
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum TileType{
    Empty,
    PushTile,
    PrioTile,
    AltTile,
    RotTileCW,
    RotTileCCW,
    DelayTile
}

impl Tile{
    pub fn new(tiletype: TileType, pos: BoardPos) -> Tile{
        Tile{
            tiletype,
            dir: Direction::Right,
            pos,
            orinal_dir: Direction::Right,
        }
    }

    pub fn new_directional(tiletype: TileType, pos: BoardPos, dir: Direction) -> Tile{
        Tile{
            tiletype,
            dir,
            pos,
            orinal_dir: dir,
        }
    }

    pub fn get_pos(&self) -> BoardPos{
        self.pos
    }

    pub fn get_x(&self) -> i32{
        self.pos.x
    }

    pub fn get_y(&self) -> i32{
        self.pos.y
    }

    pub fn get_type(&self) -> TileType{
        self.tiletype
    }

    pub fn get_dir(&self) -> Direction{
        self.dir
    }

    pub fn pos_eq(&self, other: &Tile) -> bool{
        self.pos.x==other.pos.x && self.pos.y == other.pos.y
    }

    pub fn rotate_cw(&mut self){
        self.dir = self.dir.clockwise();
    }

    pub fn rotate_ccw(&mut self){
        self.dir = self.dir.counterclockwise();
    }

    pub fn set_dir(&mut self, dir: Direction){
        self.dir = dir;
    }

    pub fn draw(&self, ctx: &mut Context, tilesize: f32) -> GameResult<Image>{
        let mut image_name = match self.tiletype{
            TileType::Empty => "empty_tile",
            TileType::PushTile => "push_tile",
            TileType::PrioTile => "prio_tile",
            TileType::AltTile => "alt_tile",
            TileType::RotTileCW => "rot_tile_cw",
            TileType::RotTileCCW => "rot_tile_ccw",
            TileType::DelayTile => "delay_tile"
        }.to_string();

        if self.tiletype.rotatable(){
            image_name = image_name + "_" + self.dir.to_string();
        }

        asset_cache::get_scaled_image(ctx, image_name, tilesize)
    }

    pub fn save_dir(&mut self){
        self.orinal_dir = self.dir;
    }

    pub fn load_dir(&mut self){
        self.dir = self.orinal_dir;
    }

    pub fn flip_dir(&mut self){
        self.dir = self.dir.clockwise().clockwise();
    }
}

impl TileType {
    // big numbers are high priority
    pub fn get_priority(&self) -> u8{
        match self{
            TileType::Empty => 0,
            TileType::PushTile => 3,
            TileType::PrioTile => 4,
            TileType::AltTile => 3,
            TileType::RotTileCW => 1,
            TileType::RotTileCCW => 1,
            TileType::DelayTile => 2 // this should never get into prio fights
        }
    }

    pub fn rotatable(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => true,
            TileType::PrioTile => true,
            TileType::AltTile => true,
            TileType::RotTileCW => false,
            TileType::RotTileCCW => false,
            TileType::DelayTile => true
        }
    }

    pub fn is_push_tile(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => true,
            TileType::PrioTile => true,
            TileType::AltTile => true,
            TileType::RotTileCW => false,
            TileType::RotTileCCW => false,
            TileType::DelayTile => true
        }
    }

    pub fn is_rot_tile(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => false,
            TileType::PrioTile => false,
            TileType::AltTile => false,
            TileType::RotTileCW => true,
            TileType::RotTileCCW => true,
            TileType::DelayTile => false
        }
    }
}
