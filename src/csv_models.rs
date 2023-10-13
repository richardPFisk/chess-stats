use std::{error::Error, io};

use chesscom_openapi::models::PlayerResult;
use chesscom_openapi::models::completed_game::{TimeClass, Rules};

use crate::models::CompletedGame;
use std::convert::From;
use std::option::Option as StdOption;
use serde::{Deserialize, Serialize};

// impl From<CompletedGame> for FlattenedGame {
//     fn from(game: CompletedGame) -> Self {
//         FlattenedGame {
//             result: (),
//             oponent_rating: (),
//             url: (),
//             fen: (),
//             pgn: (),
//             start_time: (),
//             end_time: (),
//             rated: (),
//             time_control: (),
//             time_class: (),
//             rules: (),
//             eco: (),
//             tournament: (),
//             _match: (),
//         }
//     }
// }
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlattenedGame {
    // #[serde(flatten)]
    // pub result: PlayerResult,
    // #[serde(flatten)]
    // pub oponent_rating: i32,
    pub url: String,
    pub fen: String,
    pub pgn: String,
    // , with = "chrono::serde::ts_seconds_option"
    #[serde(skip_serializing_if = "StdOption::is_none")]
    pub start_time: StdOption<chrono::DateTime<chrono::Utc>>,
    /// Timestamp of the game end
    #[serde(rename = "end_time", with = "chrono::serde::ts_seconds")]
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub rated: bool,
    /// PGN-compliant time control
    #[serde(rename = "time_control")]
    pub time_control: String,
    /// Time-per-move grouping, used for ratings
    #[serde(rename = "time_class")]
    pub time_class: TimeClass,
    /// Game variant information (e.g., \"chess960\")
    pub rules: Rules,
    /// URL pointing to ECO opening (if available)
    #[serde(rename = "eco", skip_serializing_if = "Option::is_none")]
    pub eco: Option<String>,
    /// URL pointing to tournament (if available)
    #[serde(rename = "tournament", skip_serializing_if = "Option::is_none")]
    pub tournament: Option<String>,
    /// URL pointing to team match (if available)
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub _match: Option<String>,
}

pub fn to_csv(games: &Vec<FlattenedGame>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    for g in games {
        println!("end_time {}", g.end_time);
        wtr.serialize(g)?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::models::CompletedGame;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_stuff() {
        let game_str = r#"
      {
        "white": {
          "username": "Richardfisk",
          "rating": 780,
          "result": "timeout",
          "@id": "https://api.chess.com/pub/player/richardfisk"
        },
        "black": {
          "username": "Marthinus1978",
          "rating": 948,
          "result": "win",
          "@id": "https://api.chess.com/pub/player/marthinus1978"
        },
        "url": "https://www.chess.com/game/live/74134340941",
        "fen": "2r5/8/8/3k4/4R1K1/1p3P2/5P2/8 w - -",
        "pgn": "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2023.04.02\"]\n[Round \"-\"]\n[White \"Richardfisk\"]\n[Black \"Marthinus1978\"]\n[Result \"0-1\"]\n[CurrentPosition \"2r5/8/8/3k4/4R1K1/1p3P2/5P2/8 w - -\"]\n[Timezone \"UTC\"]\n[ECO \"C70\"]\n[ECOUrl \"https://www.chess.com/openings/Ruy-Lopez-Opening-Morphy-Defense-Deferred-Classical-Defense-5.c3\"]\n[UTCDate \"2023.04.02\"]\n[UTCTime \"00:30:58\"]\n[WhiteElo \"780\"]\n[BlackElo \"948\"]\n[TimeControl \"120+1\"]\n[Termination \"Marthinus1978 won on time\"]\n[StartTime \"00:30:58\"]\n[EndDate \"2023.04.02\"]\n[EndTime \"00:36:05\"]\n[Link \"https://www.chess.com/game/live/74134340941\"]\n\n1. e4 {[%clk 0:02:01]} 1... e5 {[%clk 0:02:01]} 2. Nf3 {[%clk 0:02:01]} 2... Nc6 {[%clk 0:02:01.3]} 3. Bb5 {[%clk 0:02:01]} 3... a6 {[%clk 0:02:00.8]} 4. Ba4 {[%clk 0:02:01.1]} 4... Bc5 {[%clk 0:01:59.9]} 5. c3 {[%clk 0:02:00.7]} 5... b5 {[%clk 0:01:58.2]} 6. Bc2 {[%clk 0:01:59.9]} 6... d6 {[%clk 0:01:57.4]} 7. d4 {[%clk 0:01:59.9]} 7... exd4 {[%clk 0:01:56.9]} 8. cxd4 {[%clk 0:02:00]} 8... Bb4+ {[%clk 0:01:56.5]} 9. Nc3 {[%clk 0:01:58.4]} 9... Bg4 {[%clk 0:01:55.3]} 10. O-O {[%clk 0:01:57.3]} 10... h6 {[%clk 0:01:40.8]} 11. e5 {[%clk 0:01:57.1]} 11... dxe5 {[%clk 0:01:33.9]} 12. Re1 {[%clk 0:01:55.2]} 12... Nxd4 {[%clk 0:01:23.8]} 13. Rxe5+ {[%clk 0:01:51.4]} 13... Ne7 {[%clk 0:01:22.5]} 14. h3 {[%clk 0:01:17.2]} 14... Bxc3 {[%clk 0:01:21.7]} 15. bxc3 {[%clk 0:00:55.6]} 15... Nxf3+ {[%clk 0:01:21.1]} 16. gxf3 {[%clk 0:00:55]} 16... Qxd1+ {[%clk 0:01:19.6]} 17. Bxd1 {[%clk 0:00:53.2]} 17... Bxh3 {[%clk 0:01:18.6]} 18. Ba3 {[%clk 0:00:45.7]} 18... Be6 {[%clk 0:01:11.7]} 19. Bxe7 {[%clk 0:00:42.6]} 19... Kxe7 {[%clk 0:01:11.2]} 20. Bb3 {[%clk 0:00:40]} 20... Rhe8 {[%clk 0:01:09.5]} 21. Rae1 {[%clk 0:00:39.7]} 21... Kf8 {[%clk 0:01:08.9]} 22. Bxe6 {[%clk 0:00:34.8]} 22... Rxe6 {[%clk 0:01:05.9]} 23. Rxe6 {[%clk 0:00:31]} 23... fxe6 {[%clk 0:01:05.7]} 24. Rxe6 {[%clk 0:00:30.5]} 24... c5 {[%clk 0:01:04.1]} 25. Rc6 {[%clk 0:00:27.7]} 25... c4 {[%clk 0:01:03.2]} 26. Kg2 {[%clk 0:00:27.5]} 26... Ke8 {[%clk 0:00:57.4]} 27. Kg3 {[%clk 0:00:27.5]} 27... Kd7 {[%clk 0:00:57.5]} 28. Rg6 {[%clk 0:00:25.9]} 28... a5 {[%clk 0:00:45.1]} 29. Rxg7+ {[%clk 0:00:25.5]} 29... Ke6 {[%clk 0:00:45.1]} 30. Rg6+ {[%clk 0:00:25.5]} 30... Kf5 {[%clk 0:00:44]} 31. Rxh6 {[%clk 0:00:25]} 31... b4 {[%clk 0:00:44.7]} 32. cxb4 {[%clk 0:00:23.3]} 32... axb4 {[%clk 0:00:45.5]} 33. Rh5+ {[%clk 0:00:06.8]} 33... Kg6 {[%clk 0:00:44.8]} 34. Rh4 {[%clk 0:00:05.3]} 34... Rc8 {[%clk 0:00:40.2]} 35. Rg4+ {[%clk 0:00:05]} 35... Kf5 {[%clk 0:00:39.6]} 36. Rf4+ {[%clk 0:00:04.8]} 36... Ke5 {[%clk 0:00:39.1]} 37. Re4+ {[%clk 0:00:05]} 37... Kd5 {[%clk 0:00:38.6]} 38. Kg4 {[%clk 0:00:02.3]} 38... b3 {[%clk 0:00:37.7]} 39. axb3 {[%clk 0:00:01.6]} 39... cxb3 {[%clk 0:00:31.4]} 0-1\n",
        "end_time": 1680395765,
        "rated": true,
        "time_control": "120+1",
        "time_class": "bullet",
        "rules": "chess",
        "accuracies": null
      }
      "#;
        let game: FlattenedGame = serde_json::from_str(game_str).unwrap();
        println!("game {:#?}", game);
        let result = to_csv(&vec![game]);
        assert!(result.is_ok());
    }
}
