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

pub fn count_results(
    games: Vec<CompletedGame>,
) -> Vec<(chesscom_openapi::models::player_result::Result, usize)> {
    let mut results_count = vec![];
    for game in games {
        let mut found = false;
        for (_i, (result, count)) in results_count.iter_mut().enumerate() {
            if result == &game.white.result {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            results_count.push((game.white.result, 1));
        }
    }
    results_count
}