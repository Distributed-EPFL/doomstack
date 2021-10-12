use crate::{Doom, Stack, Top};

pub trait ResultExt<O> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom;

    fn spot(self, location: (&'static str, u32)) -> Self;
}

impl<O> ResultExt<O> for Result<O, Stack> {
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom,
    {
        self.map_err(|error| error.push(doom))
    }

    fn spot(self, location: (&'static str, u32)) -> Self {
        self.map_err(|error| error.spot(location))
    }
}

impl<O, E> ResultExt<O> for Result<O, Top<E>>
where
    E: Doom,
{
    fn push<D>(self, doom: D) -> Result<O, Top<D>>
    where
        D: Doom,
    {
        self.map_err(|error| error.push(doom))
    }

    fn spot(self, location: (&'static str, u32)) -> Self {
        self.map_err(|error| error.spot(location))
    }
}
