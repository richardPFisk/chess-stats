use pgn_reader::{SanPlus, Skip, Visitor};
use std::{borrow::Cow, collections::HashMap};
pub struct MoveCounter {
    moves: usize,
}

pub struct HeaderVisitor<'a> {
    headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl MoveCounter {
    pub fn new() -> MoveCounter {
        MoveCounter { moves: 0 }
    }
}

impl<'a> HeaderVisitor<'a> {
    pub fn new() -> HeaderVisitor<'a> {
        HeaderVisitor {
            headers: HashMap::new(),
        }
    }
}

impl<'a> Visitor for HeaderVisitor<'a> {
    type Result = HashMap<Cow<'a, str>, Cow<'a, str>>;

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        let key_str = String::from_utf8_lossy(key).into_owned().into();
        let value_str = String::from_utf8_lossy(value.0).into_owned().into();

        self.headers.insert(key_str, value_str);
    }

    fn end_headers(&mut self) -> Skip {
        Skip(true)
    }

    fn begin_game(&mut self) {}

    fn san(&mut self, san_plus: SanPlus) {
        let san = san_plus.san;
        println!("{san}");
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        self.headers.clone()
    }
}

impl Visitor for MoveCounter {
    type Result = usize;

    fn begin_game(&mut self) {
        self.moves = 0;
    }

    fn san(&mut self, _san_plus: SanPlus) {
        self.moves += 1;
    }

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_game(&mut self) -> Self::Result {
        self.moves
    }
}
