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

#[tokio::main]
async fn main() -> io::Result<()> {
    #[allow(unused_mut)]
    let mut ai_params = SimpleAIParams::default();
    //ai_params.turn_factor = 0.98;
    ai_params.max_depth = 1;
    let ai_o = SimpleAI::with_params(&ai_params);
    ai_params.win_factor = 0.99;
    let ai_x = SimpleAI::with_params(&ai_params);
    let wins: Arc<Mutex<HashMap<Color, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut tasks = Vec::new();
    for _ in 0..1000 {
        let wins = Arc::clone(&wins);
        let ai_x = ai_x.clone();
        let ai_o = ai_o.clone();
        tasks.push(tokio::spawn(async move {
            let mut game = Game::new();
            loop {
                let col = if color_into_char(game.turn()) == 'X' {
                    ai_x.get_column(&game)
                } else {
                    ai_o.get_column(&game)
                };
                //println!("{} chose column {}", color_into_char(game.turn()), col + 1);
                if let Some(winner) = game.drop_piece(col).ok().flatten() {
                    wins.lock()
                        .unwrap()
                        .entry(winner)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                    //print_board(&game);
                    break;
                }
                //print_board(&game);
            }
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    let wins = wins.lock().unwrap();
    println!("{} X wins", wins.get(&char_into_color('X')).unwrap_or(&0));
    println!("{} O wins", wins.get(&char_into_color('O')).unwrap_or(&0));
    println!("{} draws", wins.get(&char_into_color(' ')).unwrap_or(&0));
    Ok(())
}

fn get_column_from_user(game: &Game) -> io::Result<usize> {
    loop {
        print!("{} to enter column (1 - 7)> ", color_into_char(game.turn()));
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if let Ok(col) = response.trim().parse::<usize>() {
            if col >= 1 && col <= 7 {
                if !game.is_full(col - 1) {
                    return Ok(col - 1);
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
