use colored::Colorize;

use crate::row::Row;
use std::fmt;

pub struct Board<'a> {
    response: &'a [char; 5],
    rows: [Row<'a>; 6],
    pub guess_number: usize,
    guessed_letters: Vec<char>,
}

impl<'a> Board<'a> {
    pub fn new(response: &[char; 5]) -> Board {
        Board {
            response,
            rows: [
                Row::new(response),
                Row::new(response),
                Row::new(response),
                Row::new(response),
                Row::new(response),
                Row::new(response),
            ],
            guess_number: 0,
            guessed_letters: Vec::new(),
        }
    }

    pub fn make_guess(&mut self, guess: [char; 5]) {
        for (index, row) in self.rows.iter_mut().enumerate() {
            if index == self.guess_number {
                row.make_guess(guess);
            };
        }
        self.guess_number += 1;
        self.update_guessed_letters(guess);
    }

    fn update_guessed_letters(&mut self, guess: [char; 5]) {
        for letter in guess.iter() {
            if self.guessed_letters.contains(letter) {
                continue;
            }
            if self.response.contains(letter) {
                continue;
            }
            self.guessed_letters.push(letter.clone())
        }
    }

    fn keyboard_row(&self, f: &mut fmt::Formatter<'_>, row: u8) -> () {
        let keys = match row {
            0 => "q w e r t y u i o p",
            1 => " a s d f g h j k l",
            2 => "  z x c v b n m",
            _ => "",
        };
        keys.split("").for_each(|key| {
            let letter = &key.chars().next().unwrap_or('a');
            if self.guessed_letters.contains(letter) {
                write!(f, "{}", key.to_string().black()).unwrap_or_default();
            } else {
                write!(f, "{}", key.to_string().white()).unwrap_or_default();
            };
        })
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rows[0],)?;
        write!(f, "\n{}\t", self.rows[1],)?;
        self.keyboard_row(f, 0);
        write!(f, "\n{}\t", self.rows[2],)?;
        self.keyboard_row(f, 1);
        write!(f, "\n{}\t", self.rows[3],)?;
        self.keyboard_row(f, 2);
        write!(f, "\n{}", self.rows[4],)?;
        write!(f, "\n{}", self.rows[5],)
    }
}
