use crate::game::position::Position;
use crate::game::types::{CellType, MoveType};
use ndarray::Array2;

pub(crate) struct GameBoard {
    board: Array2<CellType>,
    width: usize,
    height: usize,
}

impl GameBoard {
    pub(crate) fn print(&self, player_pos: Position) {
        let border = "-".repeat((self.width * 2) + 3);
        let mut output = format!("{}\n", border);
        for i in 0..self.board.nrows() {
            if i == 0 || i == self.height + 1 {
                continue;
            }
            output.push('|');
            let row = self.board.row(i);
            for (j, cell) in row.indexed_iter() {
                if j == 0 || j == self.width + 1 {
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

    pub(crate) fn new(
        width: usize,
        height: usize,
        goal_pos: Position,
        mine_pos_list: &[Position],
    ) -> Self {
        Self {
            board: {
                let mut cell_board =
                    Array2::<CellType>::from_elem((width + 2, height + 2), CellType::Empty);
                for i in 0..cell_board.nrows() {
                    for j in 0..cell_board.ncols() {
                        if i == 0 || j == 0 || i == height + 1 || j == width + 1 {
                            cell_board[[j, i]] = CellType::None;
                        }
                    }
                }
                for mine_pos in mine_pos_list {
                    cell_board[[mine_pos.y as usize, mine_pos.x as usize]] = CellType::Mine;
                }
                cell_board[[goal_pos.y as usize, goal_pos.x as usize]] = CellType::Goal;
                cell_board.to_owned()
            },
            width,
            height,
        }
    }

    pub(crate) fn set_cell(&mut self, pos: Position, cell_type: CellType) {
        self.board[[pos.y as usize, pos.x as usize]] = cell_type;
    }

    pub(crate) fn get_cell(&self, pos: Position) -> CellType {
        self.board[[pos.y as usize, pos.x as usize]]
    }

    pub(crate) fn get_nearby_mines(&self, pos: Position) -> u8 {
        let mut counter: u8 = 0;

        for move_type in [
            MoveType::Up,
            MoveType::Down,
            MoveType::Left,
            MoveType::Right,
        ] {
            if let CellType::Mine = self.get_cell(pos + move_type) {
                counter += 1;
            }
        }

        counter
    }
}
