use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Hash, Debug)]
pub struct BoardPos {
    pub x: i32,
    pub y: i32
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy)]
pub enum MovementType{
    Translation(Direction),
    Rotation{cw: bool, around: BoardPos},
    None
}

#[derive(Deserialize)]
pub struct SerializedBlockObject{
    pub input: bool,
    pub blocks: Vec<BoardPos>,
    pub counter: i32
}

pub struct SimulationError{
    pub message: String,
    pub relevant_locations: Vec<BoardPos>
}

impl SimulationError{
    pub fn from_string(message: String) -> SimulationError{
        SimulationError{
            message,
            relevant_locations: vec![]
        }
    }
}

pub type SimulationResult<T = ()> = Result<T, SimulationError>;

impl Direction {
    pub fn clockwise(&self) -> Direction{
        match self{
            Direction::Right => Direction::Down,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Up    => Direction::Right,
        }
    }

    pub fn counterclockwise(&self) -> Direction{
        match self{
            Direction::Right => Direction::Up,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
            Direction::Up    => Direction::Left,
        }
    }

    // convert to a radian counterclockwise rotation
    pub fn to_rot(&self) -> f32{
        let pi = std::f32::consts::PI;
        match self{
            Direction::Right => 0.0,
            Direction::Down  => pi*0.5,
            Direction::Left  => pi,
            Direction::Up    => pi*1.5
        }
    }

    pub fn to_string(&self) -> &str{
        match self{
            Direction::Right => "right",
            Direction::Down  => "down",
            Direction::Left  => "left",
            Direction::Up    => "up"
        }
    }
}
