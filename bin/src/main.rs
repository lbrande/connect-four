#![allow(dead_code)]

use lib::{
    ai::{SimpleAI, AI, PerfectAI},
    Color, Game, NCOLS, NROWS,
};
use std::{
    collections::HashMap,
    io,
    io::Write,
    sync::{Arc, Mutex},
};

#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    let x = PerfectAI::with(500, 5);
    let o = PerfectAI::with(500, 5);
    let x: Option<SimpleAI> = None;
    // let x = Some(x);
    // let o: Option<SimpleAI> = None;
    let o = Some(o);
    run_verbose_game(x, o);
    // run_ai_games(x, o, 10, true).await;
}

async fn run_ai_games(x: impl AI, o: impl AI, ngames: usize, mirror: bool) {
    let wins = Arc::new(Mutex::new(HashMap::new()));
    let mut tasks = Vec::new();
    for game_num in 0..ngames {
        let wins = Arc::clone(&wins);
        let x = x.clone();
        let o = o.clone();
        tasks.push(tokio::spawn(async move {
            let winner = if mirror && game_num % 2 == 0 {
                run_ai_game(o, x)
            } else {
                run_ai_game(x, o)
            };
            let mut wins = wins.lock().unwrap();
            if mirror && game_num % 2 == 0 {
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

fn run_verbose_game(mut x: Option<impl AI>, mut o: Option<impl AI>) {
    let mut game = Game::new();
    loop {
        let col = if color_into_char(game.turn()) == 'X' {
            get_column_verbose(&game, x.as_mut())
        } else {
            get_column_verbose(&game, o.as_mut())
        };
        if let Some(winner) = game.drop_piece(col).ok().flatten() {
            print_board(&game);
            print_winner(winner);
            return;
        }
    }
}

fn run_ai_game(mut x: impl AI, mut o: impl AI) -> Color {
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

fn get_column_verbose(game: &Game, player: Option<&mut impl AI>) -> usize {
    print_board(&game);
    if let Some(ai) = player {
        get_column_from_ai(&game, ai)
    } else {
        get_column_from_user(&game)
    }
}

fn get_column_from_user(game: &Game) -> usize {
    loop {
        let color = color_into_char(game.turn());
        print!("{} to enter column (1 - {})> ", color, NCOLS);
        io::stdout().flush().unwrap();
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        if let Ok(col) = response.trim().parse::<usize>() {
            if col >= 1 && col <= NCOLS {
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

fn get_column_from_ai(game: &Game, ai: &mut impl AI) -> usize {
    let col = ai.get_column(&game);
    let color = color_into_char(game.turn());
    println!("{} chooses column {}", color, col + 1);
    col
}

fn print_board(game: &Game) {
    for row in (0..NROWS).rev() {
        for col in 0..NCOLS {
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
