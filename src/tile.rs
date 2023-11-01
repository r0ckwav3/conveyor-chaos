use crate::helpers::*;

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
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum TileType{
    Empty,
    PushTile,
    PrioTile,
    AltTile,
    RotTileCW,
    RotTileCCW
}

impl Tile{
    pub fn new(tiletype: TileType, pos: BoardPos) -> Tile{
        Tile{
            tiletype,
            dir: Direction::Right,
            pos
        }
    }

    pub fn new_directional(tiletype: TileType, pos: BoardPos, dir: Direction) -> Tile{
        Tile{
            tiletype,
            dir,
            pos
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
            TileType::RotTileCCW => "rot_tile_ccw"
        }.to_string();

        if self.tiletype.rotatable(){
            image_name = image_name + "_" + self.dir.to_string();
        }

        asset_cache::get_scaled_image(ctx, image_name, tilesize)
    }
}

impl TileType {
    // big numbers are high priority
    pub fn get_priority(&self) -> u8{
        match self{
            TileType::Empty => 0,
            TileType::PushTile => 2,
            TileType::PrioTile => 3,
            TileType::AltTile => 2,
            TileType::RotTileCW => 1,
            TileType::RotTileCCW => 1
        }
    }

    pub fn rotatable(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => true,
            TileType::PrioTile => true,
            TileType::AltTile => true,
            TileType::RotTileCW => false,
            TileType::RotTileCCW => false
        }
    }

    pub fn is_push_tile(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => true,
            TileType::PrioTile => true,
            TileType::AltTile => true,
            TileType::RotTileCW => false,
            TileType::RotTileCCW => false
        }
    }

    pub fn is_rot_tile(&self) -> bool{
        match self{
            TileType::Empty => false,
            TileType::PushTile => false,
            TileType::PrioTile => false,
            TileType::AltTile => false,
            TileType::RotTileCW => true,
            TileType::RotTileCCW => true
        }
    }
}
