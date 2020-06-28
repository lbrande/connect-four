#![allow(dead_code)]

use lib::{
    ai::{SimpleAI, SimpleAIParams, AI},
    Color, Game,
};
use rand::prelude::*;
use std::{
    collections::HashMap,
    io,
    io::Write,
    sync::{Arc, Mutex},
};

fn common_params() -> SimpleAIParams {
    #[allow(unused_mut)]
    let mut params = SimpleAIParams::default();
    params.max_depth = 1;
    params
}

fn x_params() -> SimpleAIParams {
    #[allow(unused_mut)]
    let mut params = common_params();
    params
}

fn o_params() -> SimpleAIParams {
    #[allow(unused_mut)]
    let mut params = common_params();
    params
}

#[tokio::main]
async fn main() {
    let wins = run_ai_games(1000, true).await;
    print_wins(&wins);
}

fn run_hot_seat_game() -> HashMap<Color, usize> {
    let mut wins = HashMap::new();
    let winner = run_game(get_column_from_user, get_column_from_user, true);
    wins.insert(winner, 1);
    wins
}

fn run_game_versus_ai(self_color: char) -> HashMap<Color, usize> {
    let mut wins = HashMap::new();
    let winner = match self_color {
        ' ' => {
            if random() {
                run_game(get_column_from_user, ai(o_params()), true)
            } else {
                run_game(ai(x_params()), get_column_from_user, true)
            }
        }
        'X' => run_game(get_column_from_user, ai(o_params()), true),
        'O' => run_game(ai(x_params()), get_column_from_user, true),
        c => panic!("{} is not a color", c),
    };
    wins.insert(winner, 1);
    wins
}

async fn run_ai_games(ngames: usize, mirror: bool) -> HashMap<Color, usize> {
    let wins = Arc::new(Mutex::new(HashMap::new()));
    let mut tasks = Vec::new();
    for _ in 0..ngames {
        let wins = Arc::clone(&wins);
        tasks.push(tokio::spawn(async move {
            let winner = if mirror && ngames % 2 == 0 {
                run_game(ai(x_params()), ai(o_params()), false)
            } else {
                run_game(ai(o_params()), ai(x_params()), false)
            };
            let mut wins = wins.lock().unwrap();
            if mirror && ngames % 2 == 0 {
                wins.entry(!winner).and_modify(|c| *c += 1).or_insert(1);
            } else {
                wins.entry(winner).and_modify(|c| *c += 1).or_insert(1);
            }
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    let wins = wins.lock().unwrap();
    wins.clone()
}

fn run_game(x: impl Fn(&Game) -> usize, o: impl Fn(&Game) -> usize, verbose: bool) -> Color {
    let mut game = Game::new();
    loop {
        let col = if color_into_char(game.turn()) == 'X' {
            x(&game)
        } else {
            o(&game)
        };
        if verbose {
            println!("Chosen column: {}", col + 1);
        }
        if let Some(winner) = game.drop_piece(col).ok().flatten() {
            if verbose {
                print_board(&game);
            }
            return winner;
        }
        if verbose {
            print_board(&game);
        }
    }
}

fn ai(params: SimpleAIParams) -> impl Fn(&Game) -> usize {
    let ai = SimpleAI::with_params(params);
    move |game| ai.get_column(game)
}

fn get_column_from_user(game: &Game) -> usize {
    loop {
        print!("{} to enter column (1 - 7)> ", color_into_char(game.turn()));
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if let Ok(col) = response.trim().parse::<usize>() {
            if col >= 1 && col <= 7 {
                if !game.is_full(col - 1) {
                    return col - 1;
                } else {
                    println!("Column {} is full", col);
                }
            } else {
                println!("{} is not a column", col);
            }
        } else {
            println!("{} is not a column", response.trim());
        }
    }
}

fn print_board(game: &Game) {
    for row in (0..6).rev() {
        for col in 0..7 {
            print!("|{}", color_into_char(game.get(col, row)));
        }
        println!("|");
    }
}

fn print_wins(wins: &HashMap<Color, usize>) {
    println!("{} X wins", wins.get(&char_into_color('X')).unwrap_or(&0));
    println!("{} draws", wins.get(&char_into_color(' ')).unwrap_or(&0));
    println!("{} O wins", wins.get(&char_into_color('O')).unwrap_or(&0));
}

fn color_into_char(color: Color) -> char {
    match color {
        Color::None => ' ',
        Color::Blue => 'X',
        Color::Red => 'O',
    }
}

fn char_into_color(ch: char) -> Color {
    match ch {
        ' ' => Color::None,
        'X' => Color::Blue,
        'O' => Color::Red,
        c => panic!("{} is not a color", c),
    }
}
