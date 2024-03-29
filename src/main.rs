mod minimax;
mod tic_tac_toe;

use minimax::Minimax;
use std::io::{self, Write};
use tic_tac_toe::{Player, TicTacToe};
fn main() {
    let mut game = TicTacToe::new();
    let minimax = Minimax::new(Player::O);
    game.print_board();
    loop {
        let index = if game.get_current_player() == minimax.get_player() {
            minimax.get_best_move(&game)
        } else {
            get_input()
        };
        println!("{} entered at index: {}", game.get_current_player(), index);
        game.play_at(index).unwrap();
        game.print_board();

        if let Some(_winner) = game.check_wins() {
            println!("Player {} wins!", game.get_other_player());
            break;
        }

        if !game.has_empty_cells() {
            println!("It's a draw!");
            break;
        }
    }
}

fn get_input() -> usize {
    loop {
        print!("Enter a number between 1 and 9: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().parse::<usize>();
        match input {
            Ok(num) => {
                if num >= 1 && num <= 9 {
                    return num - 1;
                }
                println!("Please enter a number between 1 and 9");
            }
            Err(e) => {
                eprintln!("error while parsing {}", e);
                continue;
            }
        }
    }
}
