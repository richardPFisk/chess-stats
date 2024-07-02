use pgn_reader::BufferedReader;
use std::{borrow::Cow, collections::HashMap, io};

use crate::models::{headers::PgnVisitor, move_counter::MoveCounter};

pub fn get_headers(pgn: &str) -> io::Result<Option<HashMap<Cow<str>, Cow<str>>>> {
    let mut reader = BufferedReader::new_cursor(pgn);

    let mut header_visitor = PgnVisitor::new();
    let headers = reader
        .read_game(&mut header_visitor)?
        .map(|p| p.headers);

    Ok(headers)
}

pub fn count_moves(pgn: &str) -> io::Result<()> {
    let mut reader = BufferedReader::new_cursor(pgn);

    let mut counter = MoveCounter::new();
    let moves = reader.read_game(&mut counter)?;
    println!("{moves:#?}");

    Ok(())
}
