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

mod row {
    use crate::Cell;
    use std::fmt;

    pub struct Word<'a>(
        pub &'a char,
        pub &'a char,
        pub &'a char,
        pub &'a char,
        pub &'a char,
    );

    pub struct Row<'a>(Cell<'a>, Cell<'a>, Cell<'a>, Cell<'a>, Cell<'a>);

    impl<'a> Row<'a> {
        pub fn new(values: Word<'a>) -> Row<'a> {
            Row(
                Cell::new(values.0),
                Cell::new(values.1),
                Cell::new(values.2),
                Cell::new(values.3),
                Cell::new(values.4),
            )
        }

        pub fn try_to_guess(&mut self, guesses: Word<'a>) {
            self.0.try_to_guess(guesses.0);
            self.1.try_to_guess(guesses.1);
            self.2.try_to_guess(guesses.2);
            self.3.try_to_guess(guesses.3);
            self.4.try_to_guess(guesses.4);
        }
    }

    impl<'a> fmt::Display for Row<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} {} {} {} {}", self.0, self.1, self.2, self.3, self.4)
        }
    }
}

use cell::Cell;
use colored::Colorize;
use row::{Row, Word};

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

    let value0 = 'b';
    let value1 = 'a';
    let value2 = 'c';
    let value3 = 'o';
    let value4 = 'n';
    let mut row = Row::new(Word(&value0, &value1, &value2, &value3, &value4));
    println!("{}", row);

    let guess0 = 'b';
    let guess1 = 'a';
    let guess2 = 't';
    let guess3 = 'o';
    let guess4 = 'n';
    row.try_to_guess(Word(&guess0, &guess1, &guess2, &guess3, &guess4));
    println!("{}", row);
}
