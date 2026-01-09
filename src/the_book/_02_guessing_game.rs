use colored::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};

#[allow(unused_imports)]
use crate::Runable;

use super::Test;

impl Test {
    pub fn gussing_game() {
        println!("Guess the number");

        let secret_number = rand::rng().random_range(1..=100);

        loop {
            println!("Please input your guess");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("failed to read line");

            let guess = match guess.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a vaild number");
                    continue;
                }
            };

            println!("You guess: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too Small!".red()),
                Ordering::Greater => println!("{}", "Too Big!".red()),
                Ordering::Equal => {
                    println!("{}", "You Gussed It Right, YOU WIN!".green());
                    break;
                }
            };
        }
    }
}

// impl Runable for Test {
//     fn run() {
//         Self::gussing_game();
//     }
// }
