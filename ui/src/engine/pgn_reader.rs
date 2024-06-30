use pgn_reader::{BufferedReader, SanPlus, Skip, Visitor};
use std::{borrow::Cow, collections::HashMap, io};

pub struct MoveCounter {
    moves: usize,
}

pub struct HeaderVisitor<'a> {
    headers: HashMap<Cow<'a, str>, Cow<'a, str>>,
}


impl MoveCounter {
  fn new() -> MoveCounter {
      MoveCounter { moves: 0 }
  }
}

impl<'a> HeaderVisitor<'a> {
    fn new() -> HeaderVisitor<'a> {
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

pub fn read_pgn_headers(pgn: &str) -> io::Result<Option<HashMap<Cow<str>,Cow<str>>>> {
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut headerVisitor = HeaderVisitor::new();
    let headers = reader.read_game(&mut headerVisitor)?;
    println!("{headers:#?}");
    Ok(headers)
}

pub fn read_pgn(pgn: &str) -> io::Result<()> {
    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut counter = MoveCounter::new();
    let moves = reader.read_game(&mut counter)?;
    println!("{moves:#?}");

    Ok(())
}

#[cfg(test)]
mod tests {
    static PGN: &str = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2022.04.16"]
[Round "-"]
[White "Richardfisk"]
[Black "Hazardmine"]
[Result "1-0"]
[CurrentPosition "r1b1r3/p4kp1/2p3B1/4p2P/2Pp1p2/P4PpR/2P3P1/2K1R3 b - -"]
[Timezone "UTC"]
[ECO "B33"]
[ECOUrl "https://www.chess.com/openings/Sicilian-Defense-Open-3...cxd4-4.Nxd4-Nf6-5.Nc3"]
[UTCDate "2022.04.16"]
[UTCTime "05:09:11"]
[WhiteElo "1052"]
[BlackElo "1113"]
[TimeControl "300+5"]
[Termination "Richardfisk won by resignation"]
[StartTime "05:09:11"]
[EndDate "2022.04.16"]
[EndTime "05:15:44"]
[Link "https://www.chess.com/game/live/43824858935"]

1. e4 {[%clk 0:05:05]} 1... c5 {[%clk 0:05:04.4]} 2. Nf3 {[%clk 0:05:09.7]} 2... Nc6 {[%clk 0:05:07]} 3. d4 {[%clk 0:05:11.7]} 3... cxd4 {[%clk 0:05:07.3]} 4. Nxd4 {[%clk 0:05:15.8]} 4... Nf6 {[%clk 0:05:08.3]} 5. Nc3 {[%clk 0:05:19.2]} 5... e6 {[%clk 0:05:11.5]} 6. Nxc6 {[%clk 0:05:18]} 6... bxc6 {[%clk 0:05:13.5]} 7. Bg5 {[%clk 0:05:19.8]} 7... h6 {[%clk 0:05:11.2]} 8. e5 {[%clk 0:05:21.8]} 8... hxg5 {[%clk 0:05:12.4]} 9. exf6 {[%clk 0:05:23.9]} 9... Qxf6 {[%clk 0:05:11.1]} 10. Qd2 {[%clk 0:05:15.3]} 10... d5 {[%clk 0:05:11.1]} 11. O-O-O {[%clk 0:05:18]} 11... Bb4 {[%clk 0:05:12.9]} 12. a3 {[%clk 0:05:06.8]} 12... Bxc3 {[%clk 0:05:09.7]} 13. Qxc3 {[%clk 0:05:10.7]} 13... Qxc3 {[%clk 0:05:13.2]} 14. bxc3 {[%clk 0:05:14.5]} 14... e5 {[%clk 0:05:13.1]} 15. f3 {[%clk 0:04:58]} 15... f5 {[%clk 0:05:03.4]} 16. c4 {[%clk 0:05:01.4]} 16... d4 {[%clk 0:05:02.9]} 17. Bd3 {[%clk 0:04:59]} 17... Kf7 {[%clk 0:05:04.3]} 18. Rde1 {[%clk 0:04:57.6]} 18... Re8 {[%clk 0:04:58.8]} 19. h4 {[%clk 0:04:53.6]} 19... g4 {[%clk 0:04:29.1]} 20. h5 {[%clk 0:04:41.7]} 20... g3 {[%clk 0:03:08]} 21. Rh3 {[%clk 0:04:18.3]} 21... f4 {[%clk 0:03:05.2]} 22. Bg6+ {[%clk 0:04:21.2]} 1-0"#;

    use super::*;

    #[test]
    fn read() {
        
        let x = read_pgn(PGN);
        println!("{x:#?}");
    }

    #[test]
    fn read_hashmap() {
        
        let x = read_pgn_headers(PGN);
        println!("{x:#?}");
    }
}
