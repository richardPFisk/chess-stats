use std::error::Error;

use chess_pgn::reader::get_san_moves;
use dioxus::prelude::*;
use shakmaty::{Board, Chess, Position};

use crate::board::ChessBoard;

#[derive(PartialEq, Clone, Props)]
pub struct ChessGameComponentProps {
    pub previous_positions: Vec<Chess>,
    #[props(!optional)]
    pub chess: Option<Chess>,
}

fn create_chess_board_props(pgn: &str) -> Result<ChessGameComponentProps, Box<dyn Error>> {
    let san_moves = get_san_moves(pgn)?.unwrap_or(vec![]);

    let (positions, final_pos) = san_moves.iter().try_fold(
        (vec![Chess::default()], Chess::default()),
        |(mut positions, current_pos), san_move| -> Result<_, Box<dyn Error>> {
            let chess_move = san_move.to_move(&current_pos)?;
            let new_pos = current_pos.play(&chess_move)?;
            positions.push(new_pos.clone());

            Ok((positions, new_pos))
        },
    )?;

    Ok(ChessGameComponentProps {
        previous_positions: positions,
        chess: Some(final_pos),
    })
}

#[derive(Clone)]
struct GameState {
    positions: Vec<Chess>,
    current_index: usize,
    is_white: bool,
}

impl GameState {
    fn new(positions: Vec<Chess>) -> Self {
        Self {
            positions,
            current_index: 0,
            is_white: true,
        }
    }

    fn move_forward(&mut self) {
        if self.current_index < self.positions.len() - 1 {
            self.current_index += 1;
        }
    }

    fn move_backward(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    fn flip_board(&mut self) {
        self.is_white = !self.is_white;
    }

    fn current_board(&self) -> Option<&Chess> {
        self.positions.get(self.current_index)
    }
}

#[component]
pub fn ChessGame(props: ChessGameComponentProps) -> Element {
    let mut game_state = use_signal(|| GameState::new(props.previous_positions));

    let handle_keydown = move |evt: KeyboardEvent| match evt.key() {
        Key::ArrowRight => game_state.write().move_forward(),
        Key::ArrowLeft => game_state.write().move_backward(),
        Key::Character(character) if &character == "x" => game_state.write().flip_board(),
        _ => {}
    };

    rsx! {
        div {
            tabindex: "0",
            onkeydown: handle_keydown,
            ChessBoard { chess: game_state.read().current_board().cloned(), is_white: game_state.read().is_white }
            div { class: "keyboard-hints",
                p { "Use arrow keys to navigate:" }
                ul {
                    li { "← : Previous position" }
                    li { "→ : Next position" }
                    li { " x  : Change colours" }
                }
            }
        }
    }
}

#[component]
pub fn ChessGameContainer() -> Element {
    let pgn = "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2022.04.02\"]\n[Round \"-\"]\n[White \"kgfritz\"]\n[Black \"Richardfisk\"]\n[Result \"1-0\"]\n[CurrentPosition \"R7/6p1/7p/8/1p3P2/kr4PK/3R4/8 b - -\"]\n[Timezone \"UTC\"]\n[ECO \"C00\"]\n[ECOUrl \"https://www.chess.com/openings/French-Defense-Knight-Variation-2...d5\"]\n[UTCDate \"2022.04.02\"]\n[UTCTime \"03:08:45\"]\n[WhiteElo \"1063\"]\n[BlackElo \"1032\"]\n[TimeControl \"300+5\"]\n[Termination \"kgfritz won by checkmate\"]\n[StartTime \"03:08:45\"]\n[EndDate \"2022.04.02\"]\n[EndTime \"03:23:39\"]\n[Link \"https://www.chess.com/game/live/42607780297\"]\n\n1. e4 {[%clk 0:05:01.6]} 1... e6 {[%clk 0:05:04.3]} 2. Nf3 {[%clk 0:04:58.3]} 2... d5 {[%clk 0:05:08.4]} 3. c3 {[%clk 0:04:57.5]} 3... dxe4 {[%clk 0:05:07.2]} 4. Ne5 {[%clk 0:04:58.8]} 4... Qf6 {[%clk 0:04:52.4]} 5. Qa4+ {[%clk 0:04:51.9]} 5... Bd7 {[%clk 0:04:41.3]} 6. Qxe4 {[%clk 0:04:53.5]} 6... Bc6 {[%clk 0:04:22.4]} 7. Nxc6 {[%clk 0:04:50.3]} 7... Nxc6 {[%clk 0:04:24.7]} 8. Bb5 {[%clk 0:04:51.7]} 8... Ne7 {[%clk 0:03:58.4]} 9. d4 {[%clk 0:04:51.2]} 9... a6 {[%clk 0:03:46.4]} 10. Ba4 {[%clk 0:04:51.5]} 10... b5 {[%clk 0:03:41]} 11. Bb3 {[%clk 0:04:52.4]} 11... Rd8 {[%clk 0:02:51.2]} 12. Be3 {[%clk 0:04:53.6]} 12... Nd5 {[%clk 0:02:47.8]} 13. Bxd5 {[%clk 0:04:55]} 13... Rxd5 {[%clk 0:02:50.4]} 14. Nd2 {[%clk 0:04:55.2]} 14... b4 {[%clk 0:02:40.4]} 15. c4 {[%clk 0:04:54]} 15... Rd7 {[%clk 0:02:26.4]} 16. O-O {[%clk 0:04:52.7]} 16... Nd8 {[%clk 0:02:11.5]} 17. Nf3 {[%clk 0:04:46.7]} 17... Be7 {[%clk 0:01:52]} 18. Ne5 {[%clk 0:04:45]} 18... Rd6 {[%clk 0:01:50.3]} 19. Ng4 {[%clk 0:04:42.5]} 19... Qg6 {[%clk 0:01:42.1]} 20. Qxg6 {[%clk 0:04:40.2]} 20... fxg6 {[%clk 0:01:46]} 21. Bf4 {[%clk 0:04:40.3]} 21... Rxd4 {[%clk 0:01:45.7]} 22. g3 {[%clk 0:04:35.3]} 22... Rxc4 {[%clk 0:01:36.9]} 23. Ne3 {[%clk 0:04:34.1]} 23... Rc6 {[%clk 0:01:18.6]} 24. b3 {[%clk 0:04:32.7]} 24... O-O {[%clk 0:01:13.5]} 25. Nc4 {[%clk 0:04:32.9]} 25... Nb7 {[%clk 0:00:37.8]} 26. Ne5 {[%clk 0:04:34.4]} 26... Rc2 {[%clk 0:00:25.7]} 27. Nd7 {[%clk 0:04:31.3]} 27... Rd8 {[%clk 0:00:19.8]} 28. Rad1 {[%clk 0:04:25.6]} 28... Nd6 {[%clk 0:00:21.3]} 29. Bxd6 {[%clk 0:04:23.9]} 29... cxd6 {[%clk 0:00:20.1]} 30. Nb6 {[%clk 0:04:23.6]} 30... d5 {[%clk 0:00:19.1]} 31. Rfe1 {[%clk 0:04:21.5]} 31... Bf6 {[%clk 0:00:21.8]} 32. Rxe6 {[%clk 0:04:18]} 32... Bc3 {[%clk 0:00:22.2]} 33. Nxd5 {[%clk 0:04:14.7]} 33... Rxd5 {[%clk 0:00:23.2]} 34. Rxd5 {[%clk 0:04:16.1]} 34... Rc1+ {[%clk 0:00:28.1]} 35. Kg2 {[%clk 0:04:18.7]} 35... h6 {[%clk 0:00:28.1]} 36. Rd8+ {[%clk 0:04:16.8]} 36... Kh7 {[%clk 0:00:32]} 37. Ree8 {[%clk 0:04:19.8]} 37... g5 {[%clk 0:00:32.5]} 38. Rh8+ {[%clk 0:04:19.4]} 38... Kg6 {[%clk 0:00:35.4]} 39. Rd6+ {[%clk 0:04:21.7]} 39... Bf6 {[%clk 0:00:34.8]} 40. h4 {[%clk 0:04:13]} 40... Ra1 {[%clk 0:00:38.1]} 41. hxg5 {[%clk 0:04:08.8]} 41... Kxg5 {[%clk 0:00:38.3]} 42. Rd5+ {[%clk 0:04:06.7]} 42... Kg6 {[%clk 0:00:41.9]} 43. Ra5 {[%clk 0:04:01.8]} 43... Bd4 {[%clk 0:00:38.4]} 44. Rxa6+ {[%clk 0:04:03.2]} 44... Kg5 {[%clk 0:00:41.7]} 45. f4+ {[%clk 0:03:59.1]} 45... Kf5 {[%clk 0:00:39]} 46. Rf8+ {[%clk 0:03:58.7]} 46... Ke4 {[%clk 0:00:37.8]} 47. Re6+ {[%clk 0:03:57.7]} 47... Kd3 {[%clk 0:00:40.8]} 48. Rd8 {[%clk 0:03:51.8]} 48... Rxa2+ {[%clk 0:00:44]} 49. Kh3 {[%clk 0:03:54.1]} 49... Rb2 {[%clk 0:00:35.7]} 50. Red6 {[%clk 0:03:55.3]} 50... Rxb3 {[%clk 0:00:24.1]} 51. Rxd4+ {[%clk 0:03:57.8]} 51... Kc2 {[%clk 0:00:24.4]} 52. Rc8+ {[%clk 0:03:57.9]} 52... Kb2 {[%clk 0:00:26.8]} 53. Rd2+ {[%clk 0:04:00.4]} 53... Ka3 {[%clk 0:00:29.7]} 54. Ra8# {[%clk 0:03:58.7]} 1-0\n";
    let props = create_chess_board_props(pgn).unwrap();

    rsx! {
      ChessGame { chess: props.chess, previous_positions: props.previous_positions }
    }
}
