use crate::{Color, Game};
use rand::prelude::*;

pub trait AI {
    fn get_column(&self, game: &Game) -> usize;
}

#[derive(Clone)]
pub struct SimpleAI {
    params: SimpleAIParams,
}

impl AI for SimpleAI {
    fn get_column(&self, game: &Game) -> usize {
        self.get_column_helper(game, 1).0
    }
}

impl Default for SimpleAI {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleAI {
    pub fn new() -> Self {
        Self {
            params: SimpleAIParams::default(),
        }
    }

    pub fn with_params(params: SimpleAIParams) -> Self {
        Self { params }
    }

    fn get_column_helper(&self, game: &Game, depth: usize) -> (usize, f32) {
        let self_color = game.turn();
        let f32_nrollouts = self.params.nrollouts as f32;
        let mut wins = [0.0; 7];
        for col in 0..7 {
            if !game.is_full(col) {
                'col: for _ in 0..self.params.nrollouts {
                    let mut game = game.clone();
                    if let Ok(Some(winner)) = game.drop_piece(col) {
                        wins[col] = f32_nrollouts * self.delta_wins(self_color, winner);
                        break;
                    } else if depth < self.params.max_depth {
                        wins[col] = f32_nrollouts - self.get_column_helper(&game, depth + 1).1;
                        break;
                    }
                    for other_col in 0..7 {
                        if !game.is_full(other_col) {
                            if let Ok(Some(winner)) = game.drop_piece(other_col) {
                                wins[col] = f32_nrollouts * self.delta_wins(self_color, winner);
                                break 'col;
                            } else {
                                game.take_piece(other_col);
                            }
                        }
                    }
                    loop {
                        if let Ok(Some(winner)) = game.drop_piece(random::<usize>() % 7) {
                            wins[col] += self.delta_wins(self_color, winner);
                            break;
                        }
                    }
                }
            } else {
                wins[col] = -1.0;
            }
        }
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

    fn delta_wins(&self, self_color: Color, winner: Color) -> f32 {
        if winner == self_color {
            1.0
        } else if winner == Color::None {
            0.5
        } else {
            0.0
        }
    }
}

#[derive(Clone)]
pub struct SimpleAIParams {
    pub nrollouts: usize,
    pub max_depth: usize,
}

impl Default for SimpleAIParams {
    fn default() -> Self {
        Self {
            nrollouts: 1000,
            max_depth: 2,
        }
    }
}
