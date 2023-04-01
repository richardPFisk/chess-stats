use regex::Regex;

use crate::models::CompletedGame;

pub fn result(
    username: &str,
    game: CompletedGame,
) -> chesscom_openapi::models::player_result::Result {
    if game.white.username == username {
        return game.white.result;
    }
    game.black.result
}

pub fn opening(game: CompletedGame) -> Option<String> {
    _opening(&game.pgn)
}

fn _opening(input: &str) -> Option<String> {
    let re = Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"]*)"#).unwrap();

    let captures = re.captures_iter(input);
    let first = &captures.into_iter().next();

    first.as_ref().map_or(None, |c| {
        c.name("opening")
            .map_or(None, |m| Some(m.as_str().to_owned()))
    })
}
