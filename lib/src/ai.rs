use crate::{Color, Game};
use rand::prelude::*;

pub trait AI: 'static + Clone + Copy + Send {
    fn get_column(&self, game: &Game) -> usize;
}

#[derive(Clone, Copy)]
pub struct SimpleAI {
    nrollouts: usize,
    max_depth: usize,
}

impl AI for SimpleAI {
    fn get_column(&self, game: &Game) -> usize {
        self.get_column_helper(game, 1).0
    }
}

impl SimpleAI {
    pub fn with(nrollouts: usize, max_depth: usize) -> Self {
        Self {
            nrollouts,
            max_depth,
        }
    }

    fn get_column_helper(&self, game: &Game, depth: usize) -> (usize, i32) {
        let self_color = game.turn();
        let mut wins = [i32::MIN; 7];
        for col in 0..7 {
            if !game.is_full(col) {
                let mut game = game.clone();
                if let Ok(Some(winner)) = game.drop_piece(col) {
                    wins[col] = self.nrollouts as i32 * Self::delta_wins(self_color, winner);
                    return (col, wins[col]);
                } else if let Some(winner) = Self::drop_any_piece(&mut game) {
                    wins[col] = self.nrollouts as i32 * Self::delta_wins(self_color, winner);
                } else if depth < self.max_depth {
                    wins[col] = self.nrollouts as i32 - self.get_column_helper(&game, depth + 1).1;
                } else {
                    for _ in 0..self.nrollouts {
                        wins[col] += Self::delta_wins(self_color, Self::rollout(&game));
                    }
                }
            }
        }
        Self::max(wins)
    }

    fn drop_any_piece(game: &mut Game) -> Option<Color> {
        for col in 0..7 {
            if !game.is_full(col) {
                if let Ok(Some(winner)) = game.drop_piece(col) {
                    return Some(winner);
                } else {
                    game.take_piece(col);
                }
            }
        }
        None
    }

    fn rollout(game: &Game) -> Color {
        let mut game = game.clone();
        loop {
            if let Ok(Some(winner)) = game.drop_piece(random::<usize>() % 7) {
                return winner;
            }
        }
    }

    fn max(wins: [i32; 7]) -> (usize, i32) {
        let mut max_col = 0;
        let mut max_wins = wins[max_col];
        for col in 1..7 {
            if wins[col] > max_wins {
                max_col = col;
                max_wins = wins[max_col];
            }
        }
        (max_col, max_wins)
    }

    fn delta_wins(self_color: Color, winner: Color) -> i32 {
        if winner == self_color {
            1
        } else if winner == Color::None {
            0
        } else {
            -1
        }
    }
}
