use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "north"),
            Direction::East => write!(f, "east"),
            Direction::South => write!(f, "south"),
            Direction::West => write!(f, "west"),
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PenStatus {
    Up,
    Down,
}

impl Default for PenStatus {
    fn default() -> Self {
        PenStatus::Up
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Area {
    pub top_left: Position,
    pub bottom_right: Position,
}

impl Area {
    pub fn contains(&self, position: Position) -> bool {
        self.top_left.x <= position.x
            && position.x <= self.bottom_right.x
            && self.bottom_right.y <= position.y
            && position.y <= self.top_left.y
    }
}
