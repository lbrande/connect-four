#![allow(dead_code)]

use lib::{
    ai::{SimpleAI, SimpleAIParams, AI},
    Color, Game,
};
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
    let x = SimpleAI::with_params(x_params());
    //let x: Option<&SimpleAI> = None;
    let o = SimpleAI::with_params(o_params());
    //let o: Option<&SimpleAI> = None;
    run_verbose_game(Some(&x), Some(&o));
}

async fn run_ai_games(x: impl AI<'static>, o: impl AI<'static>, ngames: usize, mirror: bool) {
    let wins = Arc::new(Mutex::new(HashMap::new()));
    let mut tasks = Vec::new();
    for _ in 0..ngames {
        let wins = Arc::clone(&wins);
        let x = x.clone();
        let o = o.clone();
        tasks.push(tokio::spawn(async move {
            let winner = if mirror && ngames % 2 == 0 {
                run_ai_game(x, o)
            } else {
                run_ai_game(o, x)
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
    println!("{} X wins", wins.get(&char_into_color('X')).unwrap_or(&0));
    println!("{} draws", wins.get(&char_into_color(' ')).unwrap_or(&0));
    println!("{} O wins", wins.get(&char_into_color('O')).unwrap_or(&0));
}

fn run_ai_game<'a>(x: impl AI<'a>, o: impl AI<'a>) -> Color {
    let mut game = Game::new();
    loop {
        let col = if color_into_char(game.turn()) == 'X' {
            x.get_column(&game)
        } else {
            o.get_column(&game)
        };
        if let Some(winner) = game.drop_piece(col).ok().flatten() {
            return winner;
        }
    }
}

fn run_verbose_game<'a>(x: Option<&impl AI<'a>>, o: Option<&impl AI<'a>>) {
    let mut game = Game::new();
    loop {
        let col = if color_into_char(game.turn()) == 'X' {
            get_column_verbose(&game, x)
        } else {
            get_column_verbose(&game, o)
        };
        if let Some(winner) = game.drop_piece(col).ok().flatten() {
            print_board(&game);
            print_winner(winner);
            return;
        }
    }
}

fn get_column_verbose<'a>(game: &Game, player: Option<&impl AI<'a>>) -> usize {
    print_board(&game);
    if let Some(ai) = player {
        let col = ai.get_column(&game);
        println!("X chooses column {}", col + 1);
        col
    } else {
        get_column_from_user(&game)
    }
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

fn print_winner(winner: Color) {
    match color_into_char(winner) {
        ' ' => println!("Draw"),
        'X' => println!("X wins"),
        'O' => println!("O wins"),
        _ => unreachable!(),
    }
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
