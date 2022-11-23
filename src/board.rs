use position::Position;
use types::CellType;

pub(crate) mod position;
pub(crate) mod types;

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

pub(crate) struct GameBoard {
    board: [[CellType; BOARD_WIDTH + 2]; BOARD_HEIGHT + 2],
}

impl GameBoard {
    pub(crate) fn print(&self, player_pos: &Position) {
        let border = String::from_iter(['-'; (BOARD_WIDTH + 2) * 2]);
        let mut output = format!("{}\n", border);
        for (i, row) in self.board.iter().enumerate() {
            if i == 0 || i == BOARD_HEIGHT + 1 {
                continue;
            }
            output.push('|');
            for (j, cell) in row.iter().enumerate() {
                if j == 0 || j == BOARD_WIDTH + 1 {
                    continue;
                }
                let is_player = player_pos.y == i as u8 && player_pos.x == j as u8;
                output.push_str(&format!(
                    " {}",
                    if is_player {
                        'P'
                    } else {
                        match cell {
                            CellType::Goal => 'G',
                            CellType::VisitedMine => 'X',
                            _ => 'O',
                        }
                    }
                ));
            }
            output.push_str(" |\n")
        }
        output.push_str(&border);
        println!("{}", output);
    }

    pub(crate) fn create(goal_pos: &Position, mine_pos_list: &[Position]) -> GameBoard {
        let mut board = GameBoard {
            board: [[CellType::Empty; BOARD_HEIGHT + 2]; BOARD_WIDTH + 2],
        };
        for i in 0..board.board.len() {
            for j in 0..board.board[0].len() {
                let pos = Position {
                    x: j as u8,
                    y: i as u8,
                };
                if i == 0 || j == 0 || i == BOARD_HEIGHT + 1 || j == BOARD_WIDTH + 1 {
                    board.set_cell(&pos, CellType::None)
                }
            }
        }
        for mine_pos in mine_pos_list {
            board.set_cell(mine_pos, CellType::Mine);
        }
        board.set_cell(goal_pos, CellType::Goal);
        board
    }

    pub(crate) fn set_cell(&mut self, pos: &Position, cell_type: CellType) {
        self.board[pos.y as usize][pos.x as usize] = cell_type;
    }

    pub(crate) fn get_cell(&self, pos: &Position) -> CellType {
        self.board[pos.y as usize][pos.x as usize]
    }

    pub(crate) fn move_player(&mut self, player_pos: &mut Position, new_pos: Position) -> CellType {
        let new_cell = self.get_cell(&new_pos);

        match new_cell {
            CellType::None => {}
            _ => {
                player_pos.x = new_pos.x;
                player_pos.y = new_pos.y;
            }
        }

        new_cell
    }
}
