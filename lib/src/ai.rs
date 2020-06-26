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

    pub fn with_params(params: &SimpleAIParams) -> Self {
        Self {
            params: params.clone(),
        }
    }

    fn get_column_helper(&self, game: &Game, depth: usize) -> (usize, f32) {
        let self_color = game.turn();
        let nrollouts =
            self.params.nrollouts as f32 * self.params.turn_factor.powi(game.turn_num() as i32);
        let mut wins = [0.0; 7];
        for col in 0..7 {
            if !game.is_full(col) {
                for _ in 0..nrollouts as usize {
                    let mut game = game.clone();
                    if let Ok(Some(winner)) = game.drop_piece(col) {
                        wins[col] = nrollouts * self.delta_wins(&game, self_color, winner);
                        break;
                    } else if depth < self.params.max_depth {
                        wins[col] = nrollouts - self.get_column_helper(&game, depth + 1).1;
                        break;
                    }
                    loop {
                        if let Ok(Some(winner)) = game.drop_piece(random::<usize>() % 7) {
                            wins[col] += self.delta_wins(&game, self_color, winner);
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
        //dbg!(&wins);
        (max_col, max_wins)
    }

    fn delta_wins(&self, game: &Game, self_color: Color, winner: Color) -> f32 {
        if winner == self_color {
            self.params.win_value * self.params.win_factor.powi(game.turn_num() as i32)
        } else if winner == Color::None {
            self.params.draw_value * self.params.win_factor.powi(game.turn_num() as i32)
        } else {
            self.params.loss_value * self.params.win_factor.powi(game.turn_num() as i32)
        }
    }
}

#[derive(Clone)]
pub struct SimpleAIParams {
    pub nrollouts: usize,
    pub turn_factor: f32,
    pub max_depth: usize,
    pub win_factor: f32,
    pub win_value: f32,
    pub draw_value: f32,
    pub loss_value: f32,
}

impl Default for SimpleAIParams {
    fn default() -> Self {
        Self {
            nrollouts: 1000,
            turn_factor: 1.0,
            max_depth: 2,
            win_factor: 1.0,
            win_value: 1.0,
            draw_value: 0.5,
            loss_value: 0.0,
        }
    }
}
