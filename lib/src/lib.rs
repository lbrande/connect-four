pub type Color = bool;

#[derive(Default)]
pub struct Game {
    board: [[Option<Color>; 6]; 7],
    turn: Color,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[None; 6]; 7],
            turn: false,
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<Option<Color>, ()> {
        for (row, cell) in self.board[col].iter_mut().enumerate() {
            if cell.is_none() {
                *cell = Some(self.turn);
                self.turn = !self.turn;
                return Ok(self.winner(col, row));
            }
        }
        Err(())
    }

    pub fn cell(&self, col: usize, row: usize) -> Option<Color> {
        self.board[col][row]
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    fn winner(&self, col: usize, row: usize) -> Option<Color> {
        let cell = self.board[col][row];
        for d_col in -1..=1 {
            'next: for d_row in -1..=1 {
                if (d_col != 0 || d_row != 0) && Self::is_within_bounds(col, row, d_col, d_row, 3) {
                    for d_t in 1..=3 {
                        let other_col = (col as i32 + d_t * d_col) as usize;
                        let other_row = (row as i32 + d_t * d_row) as usize;
                        if self.board[other_col][other_row] != cell {
                            continue 'next;
                        }
                    }
                    return cell;
                }
            }
        }
        None
    }

    fn is_within_bounds(col: usize, row: usize, d_col: i32, d_row: i32, d_t: i32) -> bool {
        let col = col as i32 + d_t * d_col;
        let row = row as i32 + d_t * d_row;
        col >= 0 && col < 7 && row >= 0 && row < 6
    }
}
