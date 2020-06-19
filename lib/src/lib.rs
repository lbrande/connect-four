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
            for d_row in -1..=1 {
                if d_col != 0 || d_row != 0 {
                    let mut count = 0;
                    for d_t in -3..=3 {
                        let i_col = col as i32 + d_t * d_col;
                        let i_row = row as i32 + d_t * d_row;
                        if self.is_equals(cell, i_col, i_row) {
                            count += 1;
                            if count == 4 {
                                return cell;
                            }
                        } else {
                            count = 0;
                        }
                    }
                }
            }
        }
        None
    }

    fn is_equals(&self, cell: Option<Color>, i_col: i32, i_row: i32) -> bool {
        (i_col >= 0 && i_col < 7 && i_row >= 0 && i_row < 6)
            && cell == self.board[i_col as usize][i_row as usize]
    }
}
