use std::sync::LazyLock;

use regex::Regex;
use fp_core::chain::Chain;

use crate::models::{opening::{Opening, Side}, CompletedGame};

static SHORT_OPENING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"0-9]*)"#).unwrap());
static LONG_OPENING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"https://www.chess.com/openings/(?P<opening>[^"]*)"#).unwrap());

pub fn opening(input: &str, use_short_opening: bool) -> Option<String> {
    let re: &Regex = if use_short_opening { &SHORT_OPENING_RE } else { &LONG_OPENING_RE };
    
    re.captures(input)
        .chain(|captures| 
            captures.name("opening")
                .map(|m| m.as_str().to_owned())
        )
}

pub fn game_to_opening(username: &str, game: &CompletedGame) -> Option<Opening> {
  let side = if game.white.username == username { Side::White } else { Side::Black };
  
  opening(&game.pgn, false).map(|opening_name| {
      let url = game.pgn
          .lines()
          .find(|line| line.starts_with("[ECOUrl "))
          .and_then(|line| line.split_once('"').map(|(_, url)| url.trim_end_matches('"').to_string()))
          .unwrap_or_default();
      
      Opening::new(url, opening_name, side)
  })
}

pub fn get_all_openings(username: &str, games: &[CompletedGame]) -> Vec<Opening> {
    games.iter()
        .filter_map(|game| {
            let side = if game.white.username == username { Side::White } else { Side::Black };
            opening(&game.pgn, false).map(|opening_name| {
                let url = game.pgn
                    .lines()
                    .find(|line| line.starts_with("[ECOUrl "))
                    .and_then(|line| line.split_once('"').map(|(_, url)| url.trim_end_matches('"').to_string()))
                    .unwrap_or_default();
                
                Opening::new(url, opening_name, side)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  const GAME_JSON: &str = r#"
  {
      "white": {
          "username": "Richardfisk",
          "rating": 1053,
          "result": "win",
          "@id": "https://api.chess.com/pub/player/richardfisk"
      },
      "black": {
          "username": "yab123",
          "rating": 1027,
          "result": "resigned",
          "@id": "https://api.chess.com/pub/player/yab123"
      },
      "url": "https://www.chess.com/game/live/58410028355",
      "fen": "r3kb1r/p4pp1/1pnpq2p/2pPp3/8/1PP1BN1P/1P3PP1/RN1Q1RK1 b kq -",
      "pgn": "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2022.10.02\"]\n[Round \"-\"]\n[White \"Richardfisk\"]\n[Black \"yab123\"]\n[Result \"1-0\"]\n[CurrentPosition \"r3kb1r/p4pp1/1pnpq2p/2pPp3/8/1PP1BN1P/1P3PP1/RN1Q1RK1 b kq -\"]\n[Timezone \"UTC\"]\n[ECO \"C41\"]\n[ECOUrl \"https://www.chess.com/openings/Philidor-Defense-3.Bc4-Nf6\"]\n[UTCDate \"2022.10.02\"]\n[UTCTime \"00:40:07\"]\n[WhiteElo \"1053\"]\n[BlackElo \"1027\"]\n[TimeControl \"300+5\"]\n[Termination \"Richardfisk won by resignation\"]\n[StartTime \"00:40:07\"]\n[EndDate \"2022.10.02\"]\n[EndTime \"00:43:31\"]\n[Link \"https://www.chess.com/game/live/58410028355\"]\n\n1. e4 {[%clk 0:05:05]} 1... e5 {[%clk 0:05:04.9]} 2. Nf3 {[%clk 0:05:09.2]} 2... d6 {[%clk 0:05:07.4]} 3. Bc4 {[%clk 0:05:11.4]} 3... Nf6 {[%clk 0:05:10.2]} 4. d3 {[%clk 0:05:12.4]} 4... h6 {[%clk 0:05:12.6]} 5. Bb3 {[%clk 0:05:00.5]} 5... c5 {[%clk 0:05:16]} 6. O-O {[%clk 0:04:57.7]} 6... Nc6 {[%clk 0:05:12.2]} 7. c3 {[%clk 0:04:57.7]} 7... Qe7 {[%clk 0:04:58.3]} 8. h3 {[%clk 0:04:41.8]} 8... Be6 {[%clk 0:05:01.4]} 9. Be3 {[%clk 0:04:34.4]} 9... b6 {[%clk 0:05:00.3]} 10. d4 {[%clk 0:04:08.4]} 10... Bxb3 {[%clk 0:04:57]} 11. axb3 {[%clk 0:04:03.5]} 11... Qe6 {[%clk 0:04:48.5]} 12. d5 {[%clk 0:04:04.1]} 12... Nxd5 {[%clk 0:04:44.6]} 13. exd5 {[%clk 0:04:07.6]} 1-0\n",
      "end_time": 1664671411,
      "rated": true,
      "time_control": "300+5",
      "time_class": "blitz",
      "rules": "chess",
      "accuracies": {
          "white": 95.45,
          "black": 72.65
      }
  }
  "#;

  #[test]
  fn test_get_all_openings() {
      // Deserialize the JSON into a CompletedGame struct
      let game: CompletedGame = serde_json::from_str(GAME_JSON).expect("Failed to parse JSON");

      // Test get_all_openings function
      let openings = get_all_openings("yab123", &[game]);

      // Assert the result
      assert_eq!(openings.len(), 1);
      assert_eq!(openings[0], Opening {
          url: "https://www.chess.com/openings/Philidor-Defense-3.Bc4-Nf6".to_string(),
          name: "Philidor Defense 3.Bc4 Nf6".to_string(),
          side: Side::Black,
      });
      assert_eq!(openings[0].to_string(), "Philidor Defense 3.Bc4 Nf6 (black)");
  }
}