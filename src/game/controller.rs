use crate::game::{
    board::GameBoard,
    player::Player,
    position::Position,
    types::{CellType, MoveType},
};
use device_query::{DeviceQuery, DeviceState, Keycode};

pub(crate) struct Controller {
    seed: i32,
    player: Player,
    board: GameBoard,
    device_state: DeviceState,
    is_running: bool,
}

impl Controller {
    pub(crate) fn new(seed: i32) -> Self {
        let goal_pos = Position { x: 4, y: 4 };

        let mine_list = Vec::from([
            Position { x: 2, y: 3 },
            Position { x: 4, y: 1 },
            Position { x: 3, y: 4 },
        ]);

        println!("Welcome to Mineslayer!");

        Controller {
            seed,
            player: Player {
                position: Position { x: 1, y: 1 },
                lives: 3,
            },
            board: GameBoard::new(5, 5, goal_pos, &mine_list),
            device_state: DeviceState::new(),
            is_running: false,
        }
    }

    pub(crate) fn start(&mut self) -> Result<(), String> {
        self.is_running = true;

        println!("Press an arrow key to move the player. Escape to quit.");

        while self.is_running {
            println!(
                "Lives: {}\nNearby Mines: {}",
                self.player.lives,
                self.board.get_nearby_mines(self.player.position)
            );
            self.board.print(self.player.position);
            match self.get_next_cell() {
                CellType::Mine => {
                    self.board
                        .set_cell(self.player.position, CellType::VisitedMine);
                    if self.player.lose_life() {
                        self.stop("You are out of lives. Game over.")
                    }
                }
                CellType::Goal => {
                    self.stop("You have reached the goal. You win!");
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn get_next_cell(&mut self) -> CellType {
        loop {
            if !self.device_state.get_keys().is_empty() {
                continue;
            }
            let key = self.get_input();
            let move_type = match key {
                Keycode::Up | Keycode::W => MoveType::Up,
                Keycode::Down | Keycode::S => MoveType::Down,
                Keycode::Left | Keycode::A => MoveType::Left,
                Keycode::Right | Keycode::D => MoveType::Right,
                Keycode::Escape => {
                    self.stop("Game exited.");
                    return CellType::Empty;
                }
                _ => continue,
            };
            match self.move_player(move_type) {
                Some(cell) => return cell,
                None => continue,
            }
        }
    }

    fn stop(&mut self, msg: &'static str) {
        println!("{}", msg);
        self.is_running = false;
    }

    fn get_input(&self) -> Keycode {
        loop {
            let keys: Vec<Keycode> = self.device_state.get_keys();
            if !keys.is_empty() {
                return keys[0];
            }
        }
    }

    fn move_player(&mut self, move_type: MoveType) -> Option<CellType> {
        let new_pos = self.player.position + move_type;
        let new_cell = self.board.get_cell(new_pos);
        match new_cell {
            CellType::None => None,
            _ => {
                self.player.position.x = new_pos.x;
                self.player.position.y = new_pos.y;
                Some(new_cell)
            }
        }
    }
}
