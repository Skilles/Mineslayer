mod board;
mod player;

use crate::board::{
    position::Position,
    types::{CellType, MoveType},
    GameBoard,
};
use crate::player::Player;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn handle_mine(board: &mut GameBoard, player: &mut Player) -> bool {
    *player.lives -= 1;
    board.set_cell(player.position, CellType::VisitedMine);
    println!("Ouch! You hit a mine.");
    *player.lives == 0
}

fn get_input(device_state: &DeviceState) -> Keycode {
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        if !keys.is_empty() {
            return keys[0];
        }
    }
}

fn main() -> Result<(), String> {
    println!("Welcome to Mineslayer!");

    let device_state = DeviceState::new();

    let mut player = Player {
        position: &mut Position { x: 1, y: 1 },
        lives: &mut 3,
    };

    let goal_pos = Position { x: 4, y: 4 };

    let mine_list = Vec::from([
        Position { x: 2, y: 3 },
        Position { x: 4, y: 1 },
        Position { x: 3, y: 4 },
    ]);

    let mut board = GameBoard::create(&goal_pos, &mine_list);

    println!("Press an arrow key to move the player. Escape to quit.");

    let mut run_game = true;

    while run_game {
        println!("Lives: {}", player.lives);
        board.print(player.position);
        match loop {
            if !device_state.get_keys().is_empty() {
                continue;
            }
            let key = get_input(&device_state);
            let move_type = match key {
                Keycode::Up => MoveType::Up,
                Keycode::Down => MoveType::Down,
                Keycode::Left => MoveType::Left,
                Keycode::Right => MoveType::Right,
                Keycode::Escape => {
                    println!("Game exited.");
                    run_game = false;
                    break CellType::Empty;
                }
                _ => continue,
            };
            match board.move_player(player.position, *player.position + move_type) {
                CellType::None => continue,
                cell => break cell,
            };
        } {
            CellType::Mine => {
                if handle_mine(&mut board, &mut player) {
                    println!("You are out of lives. Game over.");
                    run_game = false;
                }
            }
            CellType::Goal => {
                println!("You have reached the goal. You win!");
                run_game = false;
            }
            _ => {}
        }
    }
    Ok(())
}
