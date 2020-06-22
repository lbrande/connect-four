#![allow(clippy::needless_range_loop)]

use std::ops::Not;

pub mod ai;

#[derive(Clone)]
pub struct Game {
    board: [[Color; 6]; 7],
    turn: Color,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[Color::None; 6]; 7],
            turn: Color::Blue,
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<Option<Color>, ()> {
        for (row, cell) in self.board[col].iter_mut().enumerate() {
            if cell.is_none() {
                *cell = self.turn;
                if let Some(winner) = self.winner(col, row) {
                    self.turn = Color::None;
                    return Ok(Some(winner));
                } else {
                    self.turn = !self.turn;
                    return Ok(None);
                }
            }
        }
        Err(())
    }

    pub fn cell(&self, col: usize, row: usize) -> Color {
        self.board[col][row]
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn col_is_full(&self, col: usize) -> bool {
        self.board[col][5].is_some()
    }

    fn winner(&self, col: usize, row: usize) -> Option<Color> {
        let cell = self.board[col][row];
        for d_col in 0..=1 {
            for d_row in 0..=1 {
                if d_col != 0 || d_row != 0 {
                    let mut count = 0;
                    let mut inc_count = |range: &[i32]| {
                        for d_t in range {
                            let i_col = col as i32 + d_t * d_col;
                            let i_row = row as i32 + d_t * d_row;
                            if self.is_equals_to_cell(i_col, i_row, cell) {
                                count += 1;
                            } else {
                                break;
                            }
                        }
                    };
                    inc_count(&[-1, -2, -3]);
                    inc_count(&[1, 2, 3]);
                    if count >= 3 {
                        return Some(cell);
                    }
                }
            }
        }
        if (0..7).all(|c| self.col_is_full(c)) {
            Some(Color::None)
        } else {
            None
        }
    }

    fn is_equals_to_cell(&self, i_col: i32, i_row: i32, cell: Color) -> bool {
        (i_col >= 0 && i_col < 7 && i_row >= 0 && i_row < 6)
            && cell == self.board[i_col as usize][i_row as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    None,
    Blue,
    Red,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Self::None => Self::None,
            Self::Blue => Self::Red,
            Self::Red => Self::Blue,
        }
    }
}

impl Color {
    pub fn is_none(self) -> bool {
        self == Self::None
    }

    pub fn is_some(self) -> bool {
        self != Self::None
    }
}
