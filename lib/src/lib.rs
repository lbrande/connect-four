#![allow(clippy::needless_range_loop)]

use std::ops::Not;

pub mod ai;

pub const NROWS: usize = 6;
pub const NCOLS: usize = 7;
pub const NWIN: usize = 4;

#[derive(Clone)]
pub struct Game {
    board: [[Color; NROWS]; NCOLS],
    next_row: [usize; NCOLS],
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
            board: [[Color::None; NROWS]; NCOLS],
            next_row: [0; NCOLS],
            turn: Color::Blue,
        }
    }

    pub fn drop_piece(&mut self, col: usize) -> Result<Option<Color>, ()> {
        let row = self.next_row[col];
        if row < NROWS {
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

    pub fn get(&self, col: usize, row: usize) -> Color {
        self.board[col][row]
    }

    pub fn is_full(&self, col: usize) -> bool {
        self.next_row[col] >= NROWS
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    fn winner(&self, col: usize, row: usize) -> Option<Color> {
        let cell = self.board[col][row];
        for (delta_col, delta_row) in &[(0, 1), (1, 0), (1, -1), (1, 1)] {
            let mut count = 0;
            let mut increment_count = |range: &[i32]| {
                for delta in range {
                    let col = col as i32 + delta * delta_col;
                    let row = row as i32 + delta * delta_row;
                    if cell == self.i32_get(col, row) {
                        count += 1;
                    } else {
                        break;
                    }
                }
            };
            increment_count(&[-1, -2, -3]);
            increment_count(&[1, 2, 3]);
            if count >= NWIN - 1 {
                return Some(cell);
            }
        }
        if (0..NCOLS).all(|c| self.is_full(c)) {
            Some(Color::None)
        } else {
            None
        }
    }

    fn i32_get(&self, col: i32, row: i32) -> Color {
        if col >= 0 && col < NCOLS as i32 && row >= 0 && row < NROWS as i32 {
            self.board[col as usize][row as usize]
        } else {
            Color::None
        }
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
