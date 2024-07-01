use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Side {
  White,
  Black
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ECO(String);

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Opening {
  pub eco: ECO,
  pub name: String,
  pub pgn: String,
}

pub struct OpeningLookupByEco {
  pub lookup: HashMap<ECO, Opening>
}

pub struct OpeningLookup {
  pub lookup: HashMap<(Side, ECO), Opening>
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