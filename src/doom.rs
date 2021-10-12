use crate::Description;

use std::error;

pub trait Doom: error::Error + 'static + Sized {
    const VARIANTS: &'static [&'static str];

    fn acquire();
    fn release();
    fn store() -> bool;

    fn variant(&self) -> usize;
    fn description(&self) -> Description;
}
