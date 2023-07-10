use crate::row::Row;
use std::fmt;

pub struct Board<'a> {
    rows: [Row<'a>; 6],
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
                Row::new(response),
            ],
            guess_number: 0,
        }
    }

    pub fn make_guess(&mut self, guess: [char; 5]) {
        for (index, row) in self.rows.iter_mut().enumerate() {
            if index == self.guess_number {
                row.make_guess(guess);
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
