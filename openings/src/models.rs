use std::collections::HashMap;
use chess_pgn::models::headers::PgnData;
use serde::{Deserialize, Serialize};
use serde::{Serializer, Deserializer};

use std::fmt;
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Side {
  White,
  Black
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ECO(pub String);

#[derive(PartialEq, Eq, Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Opening {
  pub eco: ECO,
  pub name: ChessOpeningName,
  pub pgn: PgnData,
}


#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct ChessOpeningName {
  pub family: String,
  pub variation: Option<String>,
  pub sub_variation: Option<String>,
}

impl Serialize for ChessOpeningName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.variation {
            Some(variation) => serializer.serialize_str(&format!("{}: {}", self.family, variation)),
            None => serializer.serialize_str(&self.family),
        }
    }
}

impl<'de> Deserialize<'de> for ChessOpeningName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ChessOpeningName::from_string(&s).map_err(serde::de::Error::custom)
    }
}

impl ChessOpeningName {
 fn from_string(s: &str) -> Result<Self, &'static str> {
      let mut parts = s.split(": ");
      let family = parts.next().ok_or("Missing family")?.to_string();
      let mut opening = ChessOpeningName{ family, ..Default::default() };

      if let Some(rest) = parts.next() {
          let mut var_parts = rest.split(", ");
          opening.variation = var_parts.next().map(String::from);
          opening.sub_variation = var_parts.next().map(String::from);
      }

      Ok(opening)
}
}

impl fmt::Display for ChessOpeningName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.variation {
            Some(variation) => write!(f, "{}: {}", self.family, variation),
            None => write!(f, "{}", self.family),
        }
    }
}

pub struct OpeningLookupByEco {
  pub lookup: HashMap<ECO, Opening>
}

pub struct OpeningLookup {
  pub lookup: HashMap<(Side, ECO), Opening>
}

impl Default for OpeningLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl OpeningLookup {
  pub fn new() -> Self {
      OpeningLookup {
          lookup: HashMap::new()
      }
  }

  pub fn insert(&mut self, side: Side, eco: ECO, opening: Opening) {
      self.lookup.insert((side, eco), opening);
  }

  pub fn get(&self, side: &Side, eco: &ECO) -> Option<&Opening> {
      self.lookup.get(&(side.clone(), eco.clone()))
  }
}