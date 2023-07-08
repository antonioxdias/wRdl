use colored::Colorize;
use std::fmt;

struct Cell<'a> {
    value: &'a char,
    guess: Option<&'a char>,
}

impl<'a> Cell<'a> {
    fn guess(&self) -> &char {
        self.guess.unwrap_or(&'_')
    }

    fn is_correct(&self) -> bool {
        self.value == self.guess()
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

fn greet() {
    println!("\n\nHello, {}_{}ld\n\n", "w".on_yellow(), "r".on_green());
}

fn main() {
    greet();

    let value = 'a';
    let guess = Some('a');

    let cell = Cell {
        value: &value,
        guess: guess.as_ref(),
    };

    println!("{}", cell);
}
