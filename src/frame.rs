use crate::{Description, Doom};

use std::any::Any;
use std::any::TypeId;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

#[derive(Clone)]
pub struct Frame {
    tag: &'static str,
    description: Description,
    location: Option<(&'static str, u32)>,

    type_id: TypeId,
    variants: &'static [&'static str],
    original: Option<Arc<dyn Any>>,
}

impl Frame {
    pub(crate) fn new<D>(doom: &D) -> Self
    where
        D: Doom,
    {
        Frame {
            tag: D::VARIANTS[doom.variant()],
            description: doom.description(),
            location: None,
            type_id: TypeId::of::<D>(),
            variants: D::VARIANTS,
            original: None,
        }
    }

    pub(crate) fn store<D>(mut self, doom: D) -> Self
    where
        D: Doom,
    {
        self.original = Some(Arc::new(doom));
        self
    }

    pub fn spot(mut self, location: (&'static str, u32)) -> Self {
        self.location = Some(location);
        self
    }

    pub fn tag(&self) -> &'static str {
        &self.tag
    }

    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    pub fn location(&self) -> Option<(&'static str, u32)> {
        self.location
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn variants(&self) -> &'static [&'static str] {
        self.variants
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.tag)
    }
}

impl Debug for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        if let Some(location) = self.location {
            write!(
                f,
                "[{} @ {}:{}] {}",
                self.tag, location.0, location.1, self.description
            )?;
        } else {
            write!(f, "[{}] {}", self.tag, self.description)?;
        }
        Ok(())
    }
}
