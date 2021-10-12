use crate::Entry;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct Stack {
    entries: Vec<Entry>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[Entry] {
        self.entries.as_slice()
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        let last = self.entries.pop();

        let last = match last {
            Some(last) => last.spot(location),
            None => panic!("called `spot` on an empty `Stack`"),
        };

        self.entries.push(last);
        self
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if self.entries.is_empty() {
            write!(f, "<empty>")
        } else {
            write!(f, "<top: {}>", self.entries.last().unwrap())
        }
    }
}

impl Debug for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if self.entries.is_empty() {
            write!(f, "<empty>")?;
        } else {
            for frame in self.entries.iter().rev() {
                writeln!(f, "{:?}", frame)?;
            }
        }

        Ok(())
    }
}
