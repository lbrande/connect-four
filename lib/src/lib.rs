#![allow(clippy::needless_range_loop)]

use std::ops::Not;

pub mod ai;

#[derive(Clone)]
pub struct Game {
    board: [[Color; 6]; 7],
    next_row: [usize; 7],
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
            next_row: [0; 7],
            turn: Color::Blue,
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<Option<Color>, ()> {
        let row = self.next_row[col];
        if row < 6 {
            self.board[col][row] = self.turn;
            self.next_row[col] += 1;
            self.turn = !self.turn;
            if let Some(winner) = self.winner(col, row) {
                Ok(Some(winner))
            } else {
                Ok(None)
            }
        } else {
            Err(())
        }
    }

    pub fn take_piece(&mut self, col: usize) {
        self.turn = !self.turn;
        self.next_row[col] -= 1;
        self.board[col][self.next_row[col]] = Color::None;
    }

    pub fn cell(&self, col: usize, row: usize) -> Color {
        self.board[col][row]
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn col_is_full(&self, col: usize) -> bool {
        self.next_row[col] >= 6
    }

    fn winner(&self, col: usize, row: usize) -> Option<Color> {
        let cell = self.board[col][row];
        for (d_col, d_row) in &[(0, 1), (1, 0), (1, -1), (1, 1)] {
            let mut count = 0;
            let mut increment_count = |range: &[i32]| {
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
            increment_count(&[-1, -2, -3]);
            increment_count(&[1, 2, 3]);
            if count >= 3 {
                return Some(cell);
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
