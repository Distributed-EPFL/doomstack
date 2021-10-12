use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub enum Description {
    Static(&'static str),
    Dynamic(String),
}

impl Description {
    pub fn as_str(&self) -> &str {
        match self {
            Description::Static(description) => &description,
            Description::Dynamic(description) => &description,
        }
    }
}

impl From<&'static str> for Description {
    fn from(description: &'static str) -> Self {
        Description::Static(description)
    }
}

impl From<String> for Description {
    fn from(description: String) -> Self {
        Description::Dynamic(description)
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

impl Debug for Description {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.as_str())
    }
}
