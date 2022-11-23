use crate::board::position::Position;

pub(crate) struct Player<'p> {
    pub position: &'p mut Position,
    pub lives: &'p mut u8,
}
