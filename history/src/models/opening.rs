use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Side {
    White,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Opening {
    pub url: String,
    pub name: String,
    pub side: Side,
}

impl fmt::Display for Opening {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.side.to_string().to_lowercase())
    }
}

impl Opening {
    pub fn new(url: String, name: String, side: Side) -> Self {
        Opening { url, name, side }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::White => write!(f, "White"),
            Side::Black => write!(f, "Black"),
        }
    }
}