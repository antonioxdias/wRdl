mod cell {
    use colored::Colorize;
    use std::fmt;

    pub struct Cell<'a> {
        response: &'a char,
        guess: Option<char>,
    }

    impl<'a> Cell<'a> {
        pub fn new(response: &'a char) -> Cell {
            Cell {
                response,
                guess: None,
            }
        }

        fn guess(&self) -> char {
            self.guess.unwrap_or('_')
        }

        fn is_correct(&self) -> bool {
            self.response == &self.guess()
        }

        pub fn try_to_guess(&mut self, guess: char) {
            self.guess = Some(guess);
        }
    }

    impl<'a> fmt::Display for Cell<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_correct() {
                write!(f, "{}", self.guess().to_string().on_green())
            } else {
                write!(f, "{}", self.guess())
            }
        }
    }
}

mod row {
    use crate::cell::Cell;
    use std::fmt;

    pub struct Row<'a>(Cell<'a>, Cell<'a>, Cell<'a>, Cell<'a>, Cell<'a>);

    impl<'a> Row<'a> {
        pub fn new(response: &[char; 5]) -> Row {
            Row(
                Cell::new(&response[0]),
                Cell::new(&response[1]),
                Cell::new(&response[2]),
                Cell::new(&response[3]),
                Cell::new(&response[4]),
            )
        }

        pub fn try_to_guess(&mut self, guess: [char; 5]) {
            self.0.try_to_guess(guess[0]);
            self.1.try_to_guess(guess[1]);
            self.2.try_to_guess(guess[2]);
            self.3.try_to_guess(guess[3]);
            self.4.try_to_guess(guess[4]);
        }
    }

    impl fmt::Display for Row<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} {} {} {}", self.0, self.1, self.2, self.3, self.4)
        }
    }
}

mod board {
    use crate::row::Row;
    use std::fmt;

    pub struct Board<'a> {
        rows: [Row<'a>; 5],
        pub guess_number: usize,
    }

    impl<'a> Board<'a> {
        pub fn new(response: &[char; 5]) -> Board {
            Board {
                rows: [
                    Row::new(response),
                    Row::new(response),
                    Row::new(response),
                    Row::new(response),
                    Row::new(response),
                ],
                guess_number: 0,
            }
        }

        pub fn try_to_guess(&mut self, guess: [char; 5]) {
            for (index, row) in self.rows.iter_mut().enumerate() {
                if index == self.guess_number {
                    row.try_to_guess(guess);
                };
            }
            self.guess_number += 1;
        }
    }

    impl fmt::Display for Board<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}\n{}\n{}\n{}\n{}",
                self.rows[0], self.rows[1], self.rows[2], self.rows[3], self.rows[4]
            )
        }
    }
}

use board::Board;
use colored::Colorize;
use core::result::Result;
use rand::Rng;
use std::io;

mod official_allowed_guesses;
mod shuffled_real_wordles;

fn greet() {
    println!("\n\nHello, {}_{}ld\n\n", "w".on_yellow(), "r".on_green());
}

fn pick_word<'a>() -> &'a str {
    let words = shuffled_real_wordles::SHUFFLED_REAL_WORDLES;
    let word = words[rand::thread_rng().gen_range(0..=words.len() - 1)];

    println!("The word is {}", word);
    word
}

fn word_to_chars(word: &str) -> [char; 5] {
    let mut response: [char; 5] = ['_'; 5];

    for (index, letter) in word.chars().into_iter().enumerate() {
        response[index] = letter;
    }

    response
}

fn parse_guess(input: &str) -> Result<[char; 5], &'static str> {
    let clean_input = input.trim().to_lowercase();

    if clean_input.len() > 5 {
        return Err("Guess is too long");
    }

    if clean_input.len() < 5 {
        return Err("Guess is too small");
    }

    let possibilities = shuffled_real_wordles::SHUFFLED_REAL_WORDLES;
    if !possibilities
        .iter()
        .any(|option| clean_input.contains(option))
    {
        return Err("Unknown word");
    }

    let guess = word_to_chars(&clean_input);

    for letter in guess.into_iter() {
        if !letter.is_alphabetic() {
            return Err("Only letters are allowed");
        }
    }

    Ok(guess)
}

fn main() {
    greet();
    let word = pick_word();
    let response = word_to_chars(&word);

    let mut board = Board::new(&response);
    println!("{}", board);

    loop {
        println!("\nMake a guess");
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let guess = match parse_guess(&input) {
            Ok(guess) => guess,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };

        board.try_to_guess(guess);
        println!("\n{}", board);

        if guess == response {
            println!("\nGuessed in {} attempts!", board.guess_number);
            println!("You {}!", "WOW".blue());
            break;
        }

        if board.guess_number >= 5 {
            println!("\nThe word was {}.", word.red());
            println!("Better luck next time.");
            break;
        }
    }

    println!("\n");
}
