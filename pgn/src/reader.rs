use pgn_reader::{BufferedReader, San};
use std::{borrow::Cow, collections::HashMap, io};

use crate::models::{headers::PgnVisitor, move_counter::MoveCounter, pgn_moves::PgnMoves};

pub fn get_headers(pgn: &str) -> io::Result<Option<HashMap<Cow<str>, Cow<str>>>> {
    let mut reader = BufferedReader::new_cursor(pgn);

    let mut header_visitor = PgnVisitor::new();
    let headers = reader
        .read_game(&mut header_visitor)?
        .map(|p| p.headers);

    Ok(headers)
}

pub fn count_moves(pgn: &str) -> io::Result<Option<usize>> {
    let mut reader = BufferedReader::new_cursor(pgn);

    let mut counter = MoveCounter::new();
    let moves = reader.read_game(&mut counter)?;

    Ok(moves)
}

pub fn get_san_moves(pgn: &str) -> io::Result<Option<Vec<San>>> {
    let mut reader = BufferedReader::new_cursor(pgn);

    let mut pgn_moves: PgnMoves = PgnMoves::new();
    let moves = reader.read_game(&mut pgn_moves)?;

    Ok(moves)
}
