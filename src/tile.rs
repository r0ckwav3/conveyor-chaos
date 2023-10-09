pub struct Tile {
    tiletype: TileType,
    x: i32,
    y: i32
}

pub enum TileType{
    PushTile
}

impl Tile{
    pub fn new(tiletype: TileType, x: i32, y: i32) -> Tile{
        Tile{
            tiletype,
            x,
            y
        }
    }

    pub fn posEq(&self, other: &Tile) -> bool{
        self.x==other.x && self.y == other.y
    }
}
