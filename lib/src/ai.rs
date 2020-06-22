use crate::{Color, Game};
use rand::prelude::*;

pub trait AI {
    fn get_column(&mut self, game: &Game) -> usize;
}

pub struct SimpleAI {
    params: SimpleAIParams,
    color: Color,
}

impl AI for SimpleAI {
    fn get_column(&mut self, game: &Game) -> usize {
        self.get_column_depth(game, 1)
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
            color: Color::None,
        }
    }

    pub fn with_params(params: &SimpleAIParams) -> Self {
        Self {
            params: params.clone(),
            color: Color::None,
        }
    }

    fn get_column_depth(&mut self, game: &Game, depth: usize) -> usize {
        self.color = game.turn();
        let mut wins = [0.0; 7];
        for col in 0..7 {
            if depth < self.params.max_depth {
                //recursive
            }
            else if !game.col_is_full(col) {
                for _ in 0..self.params.nrollouts {
                    let mut game = game.clone();
                    if let Ok(Some(winner)) = game.drop_piece(col) {
                        wins[col] = self.params.nrollouts as f32 * self.d_wins(winner);
                        break;
                    };
                    loop {
                        if let Ok(Some(winner)) = game.drop_piece(random::<usize>() % 7) {
                            wins[col] += self.d_wins(winner);
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
        max_col
    }

    fn d_wins(&self, winner: Color) -> f32 {
        if winner == self.color {
            self.params.win_value
        } else if winner == Color::None {
            self.params.draw_value
        } else {
            self.params.loss_value
        }
    }
}

#[derive(Clone)]
pub struct SimpleAIParams {
    pub nrollouts: usize,
    pub max_depth: usize,
    pub loss_value: f32,
    pub draw_value: f32,
    pub win_value: f32,
}

impl Default for SimpleAIParams {
    fn default() -> Self {
        Self {
            nrollouts: 1000,
            max_depth: 1,
            loss_value: 0.0,
            draw_value: 0.5,
            win_value: 1.0,
        }
    }
}
