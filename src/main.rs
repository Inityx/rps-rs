mod game;

use std::io::{stdin, stdout, Write};
use game::{Outcome, Item};

fn main() {
    println!("Hello!");

    'game: loop {
        let computer_input = Item::random();

        let player_input: Item = 'input: loop {
            println!("Please enter rock, paper, scissors, or quit.");
            print!("> ");
            stdout().flush().unwrap();

            let mut text = String::new();
            stdin().read_line(&mut text).unwrap();

            if text.is_empty() || text.trim().to_lowercase() == "quit" {
                break 'game;
            }

            if let Ok(item) = text.parse() {
                break 'input item;
            }

            print!("What? ");
        };

        println!("You chose {}, computer chose {}.", player_input, computer_input);

        let outcome = player_input.versus(computer_input);
        println!("{}", match outcome {
            Outcome::Win => "You win!",
            Outcome::Lose => "You lose!",
            Outcome::Draw => "It's a draw!",
        });

        println!();
    }

    println!("Goodbye");
}
