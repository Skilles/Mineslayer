use crate::game::position::Position;

pub(crate) struct Player {
    pub position: Position,
    pub lives: u8,
}

impl Player {
    pub(crate) fn lose_life(&mut self) -> bool {
        self.lives -= 1;
        println!("Ouch! You hit a mine.");
        self.lives == 0
    }
}
