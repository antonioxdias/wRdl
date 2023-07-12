use crate::cell::{self, Cell};
use std::collections::HashMap;
use std::fmt;

pub struct Row<'a> {
    response: &'a [char; 5],
    cells: [Cell<'a>; 5],
}

impl<'a> Row<'a> {
    pub fn new(response: &[char; 5]) -> Row {
        Row {
            response,
            cells: [
                Cell::new(&response[0]),
                Cell::new(&response[1]),
                Cell::new(&response[2]),
                Cell::new(&response[3]),
                Cell::new(&response[4]),
            ],
        }
    }

    pub fn make_guess(&mut self, guess: [char; 5]) {
        let mut letter_frequencies = HashMap::new();
        for letter in self.response.iter() {
            let count = letter_frequencies.entry(letter).or_insert(0);
            *count += 1;
        }

        for (index, cell) in self.cells.iter_mut().enumerate() {
            let mut status = cell::Status::Wrong;

            let is_possible = letter_frequencies.get(&guess[index]).copied().unwrap_or(0) > 0;
            if is_possible {
                let is_correct = &guess[index] == cell.response;
                if is_correct {
                    status = cell::Status::Correct;
                } else {
                    status = cell::Status::Possible;
                }

                let count = letter_frequencies.entry(&guess[index]).or_insert(0);
                *count -= 1;
            }

            cell.make_guess(guess[index], status);
        }
    }
}

impl fmt::Display for Row<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.cells[0], self.cells[1], self.cells[2], self.cells[3], self.cells[4]
        )
    }
}
