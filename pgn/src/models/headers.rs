use serde::{ de, Deserialize, Deserializer};
use pgn_reader::{Visitor, Skip, SanPlus, BufferedReader};
use std::{borrow::Cow, collections::HashMap, mem};


#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PgnData {
    pub headers: HashMap<Cow<'static, str>, Cow<'static, str>>,
    pub moves: Vec<String>,
}


impl<'de> Deserialize<'de> for PgnData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pgn_str = String::deserialize(deserializer)?;
        let mut reader = BufferedReader::new_cursor(&pgn_str[..]);

        let mut header_visitor = PgnVisitor::new();
        let result = reader
            .read_game(&mut header_visitor)
            .map_err(|e| de::Error::custom(e.to_string()))?
            .unwrap_or_else(|| PgnVisitor::new());
        
        Ok(PgnData {
          headers: result.headers,
          moves: result.moves,
        })
    }
}

#[derive(Debug)]
pub struct PgnVisitor<'a> {
  pub headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
  pub moves: Vec<String>,
}


impl<'a> PgnVisitor<'a> {
  pub fn new() -> PgnVisitor<'a> {
      PgnVisitor {
          headers: HashMap::new(),
          moves: vec![],
      }
  }
}


impl<'a> Visitor for PgnVisitor<'a> {
  type Result = PgnVisitor<'a>;

  fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
      let key_str = String::from_utf8_lossy(key).into_owned().into();
      let value_str = String::from_utf8_lossy(value.0).into_owned().into();

      self.headers.insert(key_str, value_str);
  }

  fn end_headers(&mut self) -> Skip {
      Skip(false)
  }

  fn begin_game(&mut self) {}

  fn san(&mut self, san_plus: SanPlus) {
      let san = san_plus.san;

      self.moves.push(format!("{san}"));
  }

  fn begin_variation(&mut self) -> Skip {
      Skip(true)
  }

  fn end_game(&mut self) -> Self::Result {
    PgnVisitor {
      headers: mem::take(&mut self.headers),
      moves: mem::take(&mut self.moves),
    }
  }
}
