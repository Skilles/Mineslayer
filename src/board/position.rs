use crate::board::types::MoveType;
use std::ops;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl ops::Add<MoveType> for Position {
    type Output = Position;

    fn add(self, move_type: MoveType) -> Self::Output {
        match move_type {
            MoveType::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            MoveType::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            MoveType::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            MoveType::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}
