use std::process;

use board::Board;
use utils::read_input;

use crate::player::Player;

pub mod player;
pub mod board;
pub mod utils;

fn main() {
    print!("\x1b[?1049h");
    let config = setup();
    let mut board = Board::create(config.0, config.1, config.2);
    board.start();
    match read_input("Press any key to exit") {
        _ => {
            print!("\x1b[?1049l");
            process::exit(0);
        }
    };
}

fn setup() -> (u8, u8, Player) {
    let size = loop {
        match str::parse::<u8>(read_input("Board Size? ").as_str()) {
            Ok(parsed) => break parsed,
            Err(_) => println!("Invalid input (Must be from range 0-255)")
        };
    };

    let amount_in_row = loop {
        match str::parse::<u8>(read_input("How many in a row? ").as_str()) {
            Ok(parsed) => if parsed > size { 
                println!("This cannot be larger than the boards size!");
            } else {
                break parsed
            },
            Err(_) => println!("Invalid input")
        };
    };

    let starting_player = loop {
        match read_input("Who starts first? (X or O) ").to_uppercase().as_str() {
            "X" => break Player::X,
            "O" => break Player::O,
            _ => println!("Invalid input")
        };
    };

    (size, amount_in_row, starting_player)
}
