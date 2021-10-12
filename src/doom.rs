use crate::{Description, Stack, Top};

use std::error;

pub trait Doom: error::Error + 'static + Sized {
    const VARIANTS: &'static [&'static str];

    fn acquire();
    fn release();
    fn store() -> bool;

    fn variant(&self) -> usize;
    fn description(&self) -> Description;

    fn fail(self) -> Top<Self> {
        Stack::new().push(self)
    }

    fn fail_as_stack(self) -> Stack {
        self.fail().into()
    }
}
