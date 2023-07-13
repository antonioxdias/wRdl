mod board;
mod cell;
mod row;

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

    // println!("The word is {}", word);
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

    let possibilities = [
        shuffled_real_wordles::SHUFFLED_REAL_WORDLES.as_slice(),
        official_allowed_guesses::OFFICIAL_ALLOWED_GUESSES.as_slice(),
    ]
    .concat();

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

        board.make_guess(guess);
        println!("\n{}", board);

        if guess == response {
            println!("\nGuessed in {} attempts!", board.guess_number);
            println!("You {}!", "WOW".blue());
            break;
        }

        if board.guess_number >= 6 {
            println!("\nThe word was {}.", word.red());
            println!("Better luck next time.");
            break;
        }
    }

    println!("\n");
}
