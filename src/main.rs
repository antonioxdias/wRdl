mod cell {
    use colored::Colorize;
    use std::fmt;

    pub struct Cell<'a> {
        value: &'a char,
        guess: Option<&'a char>,
    }

    impl<'a> Cell<'a> {
        pub fn new(value: &'a char) -> Cell {
            Cell { value, guess: None }
        }

        fn guess(&self) -> &char {
            self.guess.unwrap_or(&'_')
        }

        fn is_correct(&self) -> bool {
            self.value == self.guess()
        }

        pub fn try_to_guess(&mut self, guess: &'a char) {
            self.guess = Some(guess);
        }
    }

    impl<'a> fmt::Display for Cell<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_correct() {
                write!(f, "{}", self.value.to_string().on_green())
            } else {
                write!(f, "{}", self.value)
            }
        }
    }
}

use cell::Cell;
use colored::Colorize;

fn greet() {
    println!("\n\nHello, {}_{}ld\n\n", "w".on_yellow(), "r".on_green());
}

fn main() {
    greet();

    let value = 'a';

    let mut cell = Cell::new(&value);

    println!("{}", cell);

    let guess = 'a';
    cell.try_to_guess(&guess);
    println!("{}", cell);

    let guess = 'b';
    cell.try_to_guess(&guess);
    println!("{}", cell);
}
