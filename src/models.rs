use std::option::Option;

use chesscom_openapi::models::completed_game::Rules;

use chesscom_openapi::models::completed_game::TimeClass;

use chesscom_openapi::models::PlayerResult;
use chrono;

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::option::Option as StdOption;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub(crate) games: Vec<CompletedGame>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletedGame {
    pub white: PlayerResult,
    pub black: PlayerResult,
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
    pub accuracies: Option<Accuracies>,
    #[serde(skip)]
    pub opening: Option<String>,
}

fn opening(string: &str) -> Option<&str> {
  let input = r#"\"https://www.chess.com/openings/Ruy-Lopez-Opening-Old-Steinitz-Defense-4.c3\""#;
  let re = Regex::new(r#"\\"https:\\/\\/www.chess.com\\/openings\\/(?P<opening>[^"]*)\\["]"#).unwrap();

  let captures = re.captures_iter(input);
  let first = &captures.into_iter().next();
  first.as_ref().map_or(None, |c| {
    let x= c.name("opening");
    println!("{:#?}", x);
    x.map_or(None, |m| Some(m.as_str()))
  })
  // let caps = re.captures("abc123").unwrap();

  // let text1 = caps.get(1).map_or("", |m| m.as_str());
  // text1
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accuracies {
    pub(crate) white: f32,
    pub(crate) black: f32,
}

#[cfg(test)]
mod tests {
  static game_str: &str = r#"
  {
    "url": "https://www.chess.com/game/live/71795087967",
    "pgn": "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2023.03.05\"]\n[Round \"-\"]\n[White \"Richardfisk\"]\n[Black \"rauf2005\"]\n[Result \"0-1\"]\n[CurrentPosition \"8/8/5p2/2p1p3/2P1Pp2/4kP2/2q1B3/5K2 w - -\"]\n[Timezone \"UTC\"]\n[ECO \"C62\"]\n[ECOUrl \"https://www.chess.com/openings/Ruy-Lopez-Opening-Old-Steinitz-Defense\"]\n[UTCDate \"2023.03.05\"]\n[UTCTime \"22:42:09\"]\n[WhiteElo \"1072\"]\n[BlackElo \"1014\"]\n[TimeControl \"300+5\"]\n[Termination \"rauf2005 won by resignation\"]\n[StartTime \"22:42:09\"]\n[EndDate \"2023.03.05\"]\n[EndTime \"22:56:19\"]\n[Link \"https://www.chess.com/game/live/71795087967\"]\n\n1. e4 {[%clk 0:05:05]} 1... e5 {[%clk 0:05:04.6]} 2. Nf3 {[%clk 0:05:08.6]} 2... Nc6 {[%clk 0:05:08.1]} 3. Bb5 {[%clk 0:05:12.2]} 3... d6 {[%clk 0:05:08]} 4. Ba4 {[%clk 0:05:15.1]} 4... Bd7 {[%clk 0:05:12.3]} 5. c3 {[%clk 0:05:14.7]} 5... Nf6 {[%clk 0:05:16.1]} 6. d3 {[%clk 0:05:16.2]} 6... a6 {[%clk 0:05:16.3]} 7. Bc2 {[%clk 0:05:18.3]} 7... Be7 {[%clk 0:05:12.2]} 8. O-O {[%clk 0:05:22]} 8... Bg4 {[%clk 0:05:11.5]} 9. h3 {[%clk 0:05:24.6]} 9... Bxf3 {[%clk 0:05:15.4]} 10. Qxf3 {[%clk 0:05:28.1]} 10... h6 {[%clk 0:05:18.1]} 11. Re1 {[%clk 0:05:24.1]} 11... Nh7 {[%clk 0:05:16.3]} 12. Nd2 {[%clk 0:05:26.9]} 12... Ng5 {[%clk 0:05:19.9]} 13. Qg3 {[%clk 0:05:25.6]} 13... Ne6 {[%clk 0:05:13.8]} 14. Nf1 {[%clk 0:05:21]} 14... Bh4 {[%clk 0:05:16.9]} 15. Qg4 {[%clk 0:05:13.6]} 15... Qf6 {[%clk 0:05:05.1]} 16. Ng3 {[%clk 0:04:54.8]} 16... Nf4 {[%clk 0:04:57.6]} 17. Nf5 {[%clk 0:04:46.3]} 17... g6 {[%clk 0:04:19.8]} 18. Nxh4 {[%clk 0:04:48.3]} 18... h5 {[%clk 0:04:22.4]} 19. Qg3 {[%clk 0:04:40.9]} 19... g5 {[%clk 0:04:17.2]} 20. Nf5 {[%clk 0:04:43.6]} 20... h4 {[%clk 0:04:14.3]} 21. Qg4 {[%clk 0:04:43.3]} 21... Ne7 {[%clk 0:03:56.4]} 22. Nxe7 {[%clk 0:04:40.5]} 22... Qxe7 {[%clk 0:04:01.3]} 23. Bxf4 {[%clk 0:04:44.4]} 23... gxf4 {[%clk 0:04:03.2]} 24. Qe2 {[%clk 0:04:36.4]} 24... O-O-O {[%clk 0:04:01.6]} 25. f3 {[%clk 0:04:30.4]} 25... Rhg8 {[%clk 0:03:48.2]} 26. d4 {[%clk 0:04:15.6]} 26... Rg3 {[%clk 0:03:50.8]} 27. Rf1 {[%clk 0:04:09.6]} 27... Rdg8 {[%clk 0:03:52.8]} 28. Rf2 {[%clk 0:04:13.3]} 28... Rxh3 {[%clk 0:03:52]} 29. dxe5 {[%clk 0:03:53.4]} 29... dxe5 {[%clk 0:03:51.6]} 30. Rd1 {[%clk 0:03:52.4]} 30... Rhg3 {[%clk 0:03:54.3]} 31. Qd2 {[%clk 0:03:43.8]} 31... h3 {[%clk 0:03:55.6]} 32. Bd3 {[%clk 0:03:16.2]} 32... Rxg2+ {[%clk 0:03:43.6]} 33. Rxg2 {[%clk 0:03:15.4]} 33... Rxg2+ {[%clk 0:03:47.2]} 34. Qxg2 {[%clk 0:03:14.7]} 34... hxg2 {[%clk 0:03:50.8]} 35. Kxg2 {[%clk 0:03:17.2]} 35... Qg5+ {[%clk 0:03:50.7]} 36. Kf2 {[%clk 0:03:19.3]} 36... Qg3+ {[%clk 0:03:53.8]} 37. Ke2 {[%clk 0:03:21.4]} 37... Qg2+ {[%clk 0:03:54.4]} 38. Ke1 {[%clk 0:03:25.3]} 38... Qg1+ {[%clk 0:03:56.8]} 39. Bf1 {[%clk 0:03:27.6]} 39... Qe3+ {[%clk 0:03:53.1]} 40. Be2 {[%clk 0:03:29.2]} 40... b5 {[%clk 0:03:51.7]} 41. Kf1 {[%clk 0:03:29.8]} 41... Kb7 {[%clk 0:03:49.1]} 42. a4 {[%clk 0:03:27.1]} 42... bxa4 {[%clk 0:03:52]} 43. Ra1 {[%clk 0:03:22]} 43... a5 {[%clk 0:03:34.3]} 44. Rxa4 {[%clk 0:03:25.4]} 44... Kb6 {[%clk 0:03:33]} 45. Ra3 {[%clk 0:03:13.9]} 45... Qc1+ {[%clk 0:03:36]} 46. Kf2 {[%clk 0:03:17]} 46... Qxb2 {[%clk 0:03:40]} 47. Ra4 {[%clk 0:03:11.7]} 47... c5 {[%clk 0:03:42.4]} 48. Rc4 {[%clk 0:03:06.7]} 48... Qa2 {[%clk 0:03:38.4]} 49. Ke1 {[%clk 0:02:49.8]} 49... a4 {[%clk 0:03:37.1]} 50. Bd1 {[%clk 0:02:48.2]} 50... a3 {[%clk 0:03:40.3]} 51. Ra4 {[%clk 0:02:37.8]} 51... Qb2 {[%clk 0:03:41.7]} 52. c4 {[%clk 0:02:35.9]} 52... Qc3+ {[%clk 0:03:42.9]} 53. Ke2 {[%clk 0:02:28.3]} 53... Qb2+ {[%clk 0:03:43.1]} 54. Ke1 {[%clk 0:02:28.3]} 54... a2 {[%clk 0:03:47]} 55. Ra8 {[%clk 0:02:27]} 55... a1=Q {[%clk 0:03:48]} 56. Rb8+ {[%clk 0:02:25.8]} 56... Kc6 {[%clk 0:03:51.9]} 57. Rxb2 {[%clk 0:02:28.4]} 57... Qxb2 {[%clk 0:03:56.3]} 58. Be2 {[%clk 0:02:30.6]} 58... Qc1+ {[%clk 0:03:59.1]} 59. Kf2 {[%clk 0:02:34.8]} 59... Qc2 {[%clk 0:04:02.5]} 60. Kf1 {[%clk 0:02:38.2]} 60... Kb6 {[%clk 0:04:04.5]} 61. Kf2 {[%clk 0:02:42.3]} 61... Ka5 {[%clk 0:04:08.8]} 62. Kf1 {[%clk 0:02:46]} 62... Kb4 {[%clk 0:04:13.2]} 63. Kf2 {[%clk 0:02:50.3]} 63... Kc3 {[%clk 0:04:16]} 64. Kf1 {[%clk 0:02:53.9]} 64... Kd4 {[%clk 0:04:20.2]} 65. Kf2 {[%clk 0:02:57.7]} 65... f6 {[%clk 0:04:22.5]} 66. Kf1 {[%clk 0:03:01.6]} 66... Ke3 {[%clk 0:04:26.9]} 0-1\n",
    "time_control": "300+5",
    "end_time": 1678056979,
    "rated": true,
    "accuracies": {
      "white": 71.53,
      "black": 82.28
    },
    "tcn": "mC0Kgv5QfHZRHy6Zks!TltWOyk90egZEpxEvdv3VfeT3bl3MvwMSlf0FwE7TfwSDwL2ULFVNEwUMFLNFwEQ0L0T0cDMDEm86nv?!tB!wef7!fnwxBKRKadxwmlFxktwono!oloxogo0MonMwnmwomeogtfgufmXHef6XiyHydaOGayXPyqucfncjqyYIyAjineGymdyqAyijsAjsemsjmeqiy4i~45PQ5jajdmjcencknfQPfnPGnfGzfnzsnfsBfn1TnfBu",
    "uuid": "f16d97f4-bba6-11ed-9170-6cfe544c0428",
    "initial_setup": "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    "fen": "8/8/5p2/2p1p3/2P1Pp2/4kP2/2q1B3/5K2 w - -",
    "time_class": "blitz",
    "rules": "chess",
    "white": {
      "rating": 1072,
      "result": "resigned",
      "@id": "https://api.chess.com/pub/player/richardfisk",
      "username": "Richardfisk",
      "uuid": "fc5b463e-f8d6-11e3-8028-000000000000"
    },
    "black": {
      "rating": 1014,
      "result": "win",
      "@id": "https://api.chess.com/pub/player/rauf2005",
      "username": "rauf2005",
      "uuid": "d938c9f8-36d1-11ea-a734-61a3ff83c430"
    }
  }"#;
    use serde_test::{Token, assert_tokens};

    use super::{CompletedGame, opening};


    #[test]
    fn test_opening() {
      let game: CompletedGame = serde_json::from_str(game_str).unwrap();
      let o = opening(&game.pgn);
      assert_eq!(Some(""),o);
    }

    #[test]
    #[should_panic]
    fn test_ser_de() {
        
        let game: CompletedGame = serde_json::from_str(game_str).unwrap();

        assert_tokens(
            &game,
            &[
                Token::Struct {
                    name: "CompletedGame",
                    len: 11,
                },
                Token::Str("white"),
                Token::Struct {
                    name: "PlayerResult",
                    len: 4,
                },
                Token::Str("username"),
                Token::Str("Richardfisk"),
                Token::Str("rating"),
                Token::I32(1072),
                Token::Str("result"),
                Token::UnitVariant{ name: "Result", variant: "resigned"},
                Token::Str("@id"),
                Token::Str("https://api.chess.com/pub/player/richardfisk")
            ],
        );
    }
}
