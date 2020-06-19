use lib::{Color, Game};
use std::{io, io::Write};

fn main() -> io::Result<()> {
    let mut game = Game::new();
    loop {
        print_board(&game);
        print!("{} to enter column (1 - 7)> ", cell_char(Some(game.turn())));
        io::stdout().flush()?;
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        if let Ok(col) = response.trim().parse::<usize>() {
            if col >= 1 && col <= 7 {
                if let Ok(result) = game.drop_piece(col - 1) {
                    if let Some(winner) = result {
                        print_board(&game);
                        println!("{} won", cell_char(Some(winner)));
                        break;
                    }
                } else {
                    println!("Column {} is full", col);
                }
            } else {
                println!("{} is not a column", col);
            }
        } else {
            println!("{} is not a column", response.trim());
            continue;
        }
    }
    Ok(())
}

fn print_board(game: &Game) {
    for row in (0..6).rev() {
        for col in 0..7 {
            print!("|{}", cell_char(game.cell(col, row)));
        }
        println!("|");
    }
}

fn cell_char(cell: Option<Color>) -> char {
    if let Some(color) = cell {
        if color {
            'O'
        } else {
            'X'
        }
    } else {
        ' '
    }
}
