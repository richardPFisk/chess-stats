use std::error::Error;

use pgn_parser::ParsedGame;
use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};

struct MoveCounter {
  moves: usize,
}

impl MoveCounter {
  fn new() -> MoveCounter {
      MoveCounter { moves: 0 }
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

pub fn read_pgn(pgn: &str) -> Result<Option<usize>, Box<dyn Error>> {
  let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut visitor = MoveCounter::new();
    let pos = reader.read_game(&mut visitor)?;
    Ok(pos)
}

pub fn get_pgn(pgn: &str) -> Result<(), Box<dyn Error>> {
  let x: ParsedGame = pgn_parser::parse_pgn(pgn)?;
  let y = x.game_result;
  println!("{y:?}");
  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::engine::pgn::get_pgn;

use super::read_pgn;

  #[test]
  fn test_pgn() {
    let pgn = "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2022.04.01\"]\n[Round \"-\"]\n[White \"Yassnov38\"]\n[Black \"Richardfisk\"]\n[Result \"1-0\"]\n[CurrentPosition \"5r1k/5pp1/7p/p7/R3Q3/6P1/1p1q1PKP/1R6 b - -\"]\n[Timezone \"UTC\"]\n[ECO \"C02\"]\n[ECOUrl \"https://www.chess.com/openings/French-Defense-Advance-Nimzowitsch-System\"]\n[UTCDate \"2022.04.01\"]\n[UTCTime \"22:46:05\"]\n[WhiteElo \"1047\"]\n[BlackElo \"1033\"]\n[TimeControl \"300+5\"]\n[Termination \"Yassnov38 won on time\"]\n[StartTime \"22:46:05\"]\n[EndDate \"2022.04.01\"]\n[EndTime \"23:01:54\"]\n[Link \"https://www.chess.com/game/live/42592168707\"]\n\n1. e4 {[%clk 0:05:05]} 1... e6 {[%clk 0:05:04.9]} 2. Nf3 {[%clk 0:05:09.4]} 2... d5 {[%clk 0:05:09.5]} 3. e5 {[%clk 0:05:13.8]} 3... c5 {[%clk 0:05:13.7]} 4. d4 {[%clk 0:05:17.7]} 4... Nc6 {[%clk 0:05:17.9]} 5. c4 {[%clk 0:05:04.3]} 5... Qb6 {[%clk 0:05:20.9]} 6. cxd5 {[%clk 0:05:00.3]} 6... exd5 {[%clk 0:05:23.5]} 7. Be3 {[%clk 0:04:37]} 7... Qxb2 {[%clk 0:05:24.9]} 8. Nbd2 {[%clk 0:04:21.5]} 8... Bg4 {[%clk 0:04:57.3]} 9. dxc5 {[%clk 0:03:51.6]} 9... Nxe5 {[%clk 0:04:35]} 10. Be2 {[%clk 0:03:44.7]} 10... Nf6 {[%clk 0:03:59.9]} 11. Bd4 {[%clk 0:03:35.2]} 11... Nxf3+ {[%clk 0:03:43.6]} 12. Nxf3 {[%clk 0:02:51.7]} 12... Qb4+ {[%clk 0:03:35.7]} 13. Kf1 {[%clk 0:01:51.5]} 13... Bxc5 {[%clk 0:03:27]} 14. Rb1 {[%clk 0:01:28.9]} 14... Qa5 {[%clk 0:03:28.7]} 15. Bxf6 {[%clk 0:01:09.4]} 15... O-O {[%clk 0:02:19.9]} 16. Bd4 {[%clk 0:00:43.6]} 16... Be6 {[%clk 0:01:58.8]} 17. Ng5 {[%clk 0:00:29.2]} 17... Bxd4 {[%clk 0:01:56.9]} 18. Qxd4 {[%clk 0:00:32.4]} 18... Qxa2 {[%clk 0:01:57.2]} 19. Re1 {[%clk 0:00:23]} 19... a5 {[%clk 0:01:50.1]} 20. g3 {[%clk 0:00:26.5]} 20... h6 {[%clk 0:01:43]} 21. Nf3 {[%clk 0:00:23.3]} 21... b5 {[%clk 0:01:42.9]} 22. Kg2 {[%clk 0:00:27]} 22... b4 {[%clk 0:01:29.4]} 23. Ne5 {[%clk 0:00:29.5]} 23... Qc2 {[%clk 0:01:12.2]} 24. Bd3 {[%clk 0:00:28.6]} 24... Qc3 {[%clk 0:01:02.2]} 25. Qe3 {[%clk 0:00:26.3]} 25... d4 {[%clk 0:00:58]} 26. Qf3 {[%clk 0:00:24.9]} 26... Rad8 {[%clk 0:00:55.8]} 27. Nc6 {[%clk 0:00:25.4]} 27... Bd5 {[%clk 0:00:55.7]} 28. Ne7+ {[%clk 0:00:20.1]} 28... Kh8 {[%clk 0:00:48.4]} 29. Nxd5 {[%clk 0:00:21.1]} 29... Rxd5 {[%clk 0:00:25.3]} 30. Qxd5 {[%clk 0:00:26]} 30... Qxd3 {[%clk 0:00:29.3]} 31. Rd1 {[%clk 0:00:27.6]} 31... Qc3 {[%clk 0:00:20.9]} 32. Rxd4 {[%clk 0:00:31.2]} 32... b3 {[%clk 0:00:23.5]} 33. Rc4 {[%clk 0:00:25.8]} 33... Qb2 {[%clk 0:00:23.6]} 34. Qe4 {[%clk 0:00:25]} 34... Qd2 {[%clk 0:00:14.6]} 35. Rb1 {[%clk 0:00:25.4]} 35... b2 {[%clk 0:00:15.3]} 36. Ra4 {[%clk 0:00:24.1]} 1-0\n";
    let result = read_pgn(&pgn);
    println!("{result:?}");

    assert_eq!(result.unwrap(), Some(31));
  }

  #[test]
  fn test_get_pgn() {
    let pgn = "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2022.04.01\"]\n[Round \"-\"]\n[White \"Yassnov38\"]\n[Black \"Richardfisk\"]\n[Result \"1-0\"]\n[CurrentPosition \"5r1k/5pp1/7p/p7/R3Q3/6P1/1p1q1PKP/1R6 b - -\"]\n[Timezone \"UTC\"]\n[ECO \"C02\"]\n[ECOUrl \"https://www.chess.com/openings/French-Defense-Advance-Nimzowitsch-System\"]\n[UTCDate \"2022.04.01\"]\n[UTCTime \"22:46:05\"]\n[WhiteElo \"1047\"]\n[BlackElo \"1033\"]\n[TimeControl \"300+5\"]\n[Termination \"Yassnov38 won on time\"]\n[StartTime \"22:46:05\"]\n[EndDate \"2022.04.01\"]\n[EndTime \"23:01:54\"]\n[Link \"https://www.chess.com/game/live/42592168707\"]\n\n1. e4 {[%clk 0:05:05]} 1... e6 {[%clk 0:05:04.9]} 2. Nf3 {[%clk 0:05:09.4]} 2... d5 {[%clk 0:05:09.5]} 3. e5 {[%clk 0:05:13.8]} 3... c5 {[%clk 0:05:13.7]} 4. d4 {[%clk 0:05:17.7]} 4... Nc6 {[%clk 0:05:17.9]} 5. c4 {[%clk 0:05:04.3]} 5... Qb6 {[%clk 0:05:20.9]} 6. cxd5 {[%clk 0:05:00.3]} 6... exd5 {[%clk 0:05:23.5]} 7. Be3 {[%clk 0:04:37]} 7... Qxb2 {[%clk 0:05:24.9]} 8. Nbd2 {[%clk 0:04:21.5]} 8... Bg4 {[%clk 0:04:57.3]} 9. dxc5 {[%clk 0:03:51.6]} 9... Nxe5 {[%clk 0:04:35]} 10. Be2 {[%clk 0:03:44.7]} 10... Nf6 {[%clk 0:03:59.9]} 11. Bd4 {[%clk 0:03:35.2]} 11... Nxf3+ {[%clk 0:03:43.6]} 12. Nxf3 {[%clk 0:02:51.7]} 12... Qb4+ {[%clk 0:03:35.7]} 13. Kf1 {[%clk 0:01:51.5]} 13... Bxc5 {[%clk 0:03:27]} 14. Rb1 {[%clk 0:01:28.9]} 14... Qa5 {[%clk 0:03:28.7]} 15. Bxf6 {[%clk 0:01:09.4]} 15... O-O {[%clk 0:02:19.9]} 16. Bd4 {[%clk 0:00:43.6]} 16... Be6 {[%clk 0:01:58.8]} 17. Ng5 {[%clk 0:00:29.2]} 17... Bxd4 {[%clk 0:01:56.9]} 18. Qxd4 {[%clk 0:00:32.4]} 18... Qxa2 {[%clk 0:01:57.2]} 19. Re1 {[%clk 0:00:23]} 19... a5 {[%clk 0:01:50.1]} 20. g3 {[%clk 0:00:26.5]} 20... h6 {[%clk 0:01:43]} 21. Nf3 {[%clk 0:00:23.3]} 21... b5 {[%clk 0:01:42.9]} 22. Kg2 {[%clk 0:00:27]} 22... b4 {[%clk 0:01:29.4]} 23. Ne5 {[%clk 0:00:29.5]} 23... Qc2 {[%clk 0:01:12.2]} 24. Bd3 {[%clk 0:00:28.6]} 24... Qc3 {[%clk 0:01:02.2]} 25. Qe3 {[%clk 0:00:26.3]} 25... d4 {[%clk 0:00:58]} 26. Qf3 {[%clk 0:00:24.9]} 26... Rad8 {[%clk 0:00:55.8]} 27. Nc6 {[%clk 0:00:25.4]} 27... Bd5 {[%clk 0:00:55.7]} 28. Ne7+ {[%clk 0:00:20.1]} 28... Kh8 {[%clk 0:00:48.4]} 29. Nxd5 {[%clk 0:00:21.1]} 29... Rxd5 {[%clk 0:00:25.3]} 30. Qxd5 {[%clk 0:00:26]} 30... Qxd3 {[%clk 0:00:29.3]} 31. Rd1 {[%clk 0:00:27.6]} 31... Qc3 {[%clk 0:00:20.9]} 32. Rxd4 {[%clk 0:00:31.2]} 32... b3 {[%clk 0:00:23.5]} 33. Rc4 {[%clk 0:00:25.8]} 33... Qb2 {[%clk 0:00:23.6]} 34. Qe4 {[%clk 0:00:25]} 34... Qd2 {[%clk 0:00:14.6]} 35. Rb1 {[%clk 0:00:25.4]} 35... b2 {[%clk 0:00:15.3]} 36. Ra4 {[%clk 0:00:24.1]} 1-0";
    let result = get_pgn(&pgn);
    println!("{result:?}");

    // assert_eq!(result.unwrap(), Some(31));
  }
}