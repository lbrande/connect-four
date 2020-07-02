use crate::{Color, Game, NCOLS};
use rand::prelude::*;
use std::collections::HashMap;

pub trait AI: 'static + Clone + Send {
    fn get_column(&mut self, game: &Game) -> usize;
}

#[derive(Clone)]
pub struct SimpleAI {
    nrollouts: usize,
    max_depth: usize,
}

impl AI for SimpleAI {
    fn get_column(&mut self, game: &Game) -> usize {
        self.get_column_helper(game, self.max_depth).0
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
        let mut wins = [0; NCOLS];
        for col in 0..NCOLS {
            if game.is_full(col) {
                wins[col] = i32::MIN;
            } else {
                let mut game = game.clone();
                if let Ok(Some(winner)) = game.drop_piece(col) {
                    wins[col] = self.nrollouts as i32 * delta_wins(self_color, winner);
                    return (col, wins[col]);
                } else if let Some(winner) = Self::drop_any_piece(&mut game) {
                    wins[col] = self.nrollouts as i32 * delta_wins(self_color, winner);
                } else if depth > 0 {
                    wins[col] = self.nrollouts as i32 - self.get_column_helper(&game, depth - 1).1;
                } else {
                    for _ in 0..self.nrollouts {
                        wins[col] += delta_wins(self_color, rollout(&game));
                    }
                }
            }
        }
        Self::max(wins)
    }

    fn drop_any_piece(game: &mut Game) -> Option<Color> {
        for col in 0..NCOLS {
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

    fn max(wins: [i32; NCOLS]) -> (usize, i32) {
        let mut max_col = 0;
        let mut max_wins = wins[max_col];
        for col in 1..NCOLS {
            if wins[col] > max_wins {
                max_col = col;
                max_wins = wins[max_col];
            }
        }
        (max_col, max_wins)
    }
}

#[derive(Clone)]
pub struct PerfectAI {
    nrollouts: usize,
    max_depth: usize,
    memo: HashMap<Game, (i32, usize, f32)>,
}

impl AI for PerfectAI {
    fn get_column(&mut self, game: &Game) -> usize {
        self.get_column_helper(&mut game.clone(), self.max_depth, -1.0, 1.0)
            .0
    }
}

impl PerfectAI {
    pub fn with(nrollouts: usize, max_depth: usize) -> Self {
        Self {
            nrollouts,
            max_depth,
            memo: HashMap::new(),
        }
    }

    fn get_column_helper(
        &mut self,
        game: &mut Game,
        depth: usize,
        mut alpha: f32,
        mut beta: f32,
    ) -> (usize, f32) {
        let alpha_original = alpha;
        if let Some(&(bound, col, value)) = self.memo.get(game) {
            if bound == -1 {
                alpha = alpha.max(value);
            } else if bound == 1 {
                beta = beta.min(value);
            } else {
                return (col, value);
            }
            if alpha >= beta {
                return (col, value);
            }
        }
        if depth == 0 {
            return Self::get_column_from_rollout(game, self.nrollouts);
        }
        let self_color = game.turn();
        let mut max_col = 0;
        let mut max_value = f32::MIN;
        for col in Self::cols_in_order(&game, self.nrollouts / 2) {
            let value = if let Ok(Some(winner)) = game.drop_piece(col) {
                delta_wins(self_color, winner) as f32
            } else {
                -self.get_column_helper(game, depth - 1, -beta, -alpha).1
            };
            game.take_piece(col);
            if value > max_value {
                max_col = col;
                max_value = value;
                alpha = alpha.max(value);
            }
            if alpha >= beta {
                break;
            }
        }
        if max_value <= alpha_original {
            self.memo.insert(game.clone(), (1, max_col, max_value));
        } else if max_value >= beta {
            self.memo.insert(game.clone(), (-1, max_col, max_value));
        } else {
            self.memo.insert(game.clone(), (0, max_col, max_value));
        }
        (max_col, max_value)
    }

    fn get_column_from_rollout(game: &Game, nrollouts: usize) -> (usize, f32) {
        let self_color = game.turn();
        let mut max_col = 0;
        let mut max_wins = -1;
        for col in 0..NCOLS {
            let mut game = game.clone();
            if game.is_full(col) {
                continue;
            } else if let Ok(Some(_)) = game.drop_piece(col) {
                return (col, 1.0);
            } else {
                let mut wins = 0;
                for _ in 0..nrollouts {
                    wins += delta_wins(self_color, rollout(&game));
                }
                if wins > max_wins {
                    max_col = col;
                    max_wins = wins;
                }
            }
        }
        (max_col, max_wins as f32 / nrollouts as f32)
    }

    fn cols_in_order(game: &Game, nrollouts: usize) -> Vec<usize> {
        let self_color = game.turn();
        let mut wins = [0; NCOLS];
        for col in 0..NCOLS {
            let mut game = game.clone();
            if game.is_full(col) {
                wins[col] = i32::MIN;
            } else if let Ok(Some(_)) = game.drop_piece(col) {
                return vec![col];
            } else {
                for _ in 0..nrollouts {
                    wins[col] += delta_wins(self_color, rollout(&game));
                }
            }
        }
        let mut cols: Vec<usize> = (0..NCOLS).filter(|c| wins[*c] != i32::MIN).collect();
        cols.sort_by(|a, b| wins[*b].cmp(&wins[*a]));
        cols
    }
}

fn rollout(game: &Game) -> Color {
    let mut game = game.clone();
    loop {
        if let Ok(Some(winner)) = game.drop_piece(random::<usize>() % NCOLS) {
            return winner;
        }
    }
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
