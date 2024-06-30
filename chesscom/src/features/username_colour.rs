use crate::models::CompletedGame;
use super::Colour;

pub fn username_colour(
    username: &str,
    game: &CompletedGame,
) -> Colour {
    if game.white.username == username {
        return Colour::White;
    }
    Colour::Black
}