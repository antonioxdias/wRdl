use colored::Colorize;
use std::fmt;

pub enum Status {
    Wrong,
    Possible,
    Correct,
}

pub struct Cell<'a> {
    pub response: &'a char,
    guess: Option<char>,
    status: Status,
}

impl<'a> Cell<'a> {
    pub fn new(response: &char) -> Cell {
        Cell {
            response,
            guess: None,
            status: Status::Wrong,
        }
    }

    fn guess(&self) -> char {
        self.guess.unwrap_or('_')
    }

    pub fn make_guess(&mut self, guess: char, status: Status) {
        self.guess = Some(guess);
        self.status = status;
    }
}

impl<'a> fmt::Display for Cell<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            Status::Wrong => {
                write!(f, "{}", self.guess())
            }
            Status::Possible => {
                write!(f, "{}", self.guess().to_string().on_yellow())
            }
            Status::Correct => {
                write!(f, "{}", self.guess().to_string().on_green())
            }
        }
    }
}
