use std::error::Error;

use chess_pgn::reader::get_san_moves;
use dioxus::prelude::*;
use shakmaty::{Board, Chess, Position};

use crate::{board::ChessBoard, moves::make_move};

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
pub struct GameState {
    pub positions: Vec<Chess>,
    pub current_index: usize,
    pub is_white_perspective: bool,
}

impl GameState {
    pub fn new(positions: Vec<Chess>) -> Self {
        Self {
            positions,
            current_index: 0,
            is_white_perspective: true,
        }
    }

    pub fn move_forward(&mut self) {
        if self.current_index < self.positions.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn move_backward(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn flip_board(&mut self) {
        self.is_white_perspective = !self.is_white_perspective;
    }

    pub fn current_board(&self) -> Option<&Board> {
        self.positions.get(self.current_index).map(|c| c.board())
    }

    pub fn current_position(&self) -> Option<&Chess> {
        self.positions.get(self.current_index)
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> Option<Chess> {
        if let Some(position) = self.current_position() {
            let new_position = make_move(position.clone(), to, from, self.is_white_perspective);
            if let Some(p) = new_position {
                self.positions[self.current_index] = p.clone();
                return Some(p)
            }
        }
        None
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
            ChessBoard { game_state: game_state }
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
    let pgn = r#"[Event "Live Chess"]
[Site "Chess.com"]
[Date "2024.05.28"]
[Round "?"]
[White "Tibiao"]
[Black "Richardfisk"]
[Result "0-1"]
[ECO "C01"]
[WhiteElo "1260"]
[BlackElo "1301"]
[TimeControl "900+10"]
[EndTime "16:29:13 PDT"]
[Termination "Richardfisk won by resignation"]

1. e4 e6 2. d4 d5 3. exd5 exd5 4. Nc3 Nf6 5. Nf3 Bb4 6. Bd2 O-O 7. Bd3 Bg4 8.
O-O c5 9. dxc5 Bxc5 10. h3 Bh5 11. Be2 Re8 12. Bg5 Nbd7 13. Re1 Qb6 14. Rf1 Ne4
15. Bh4 Qxb2 16. Nxd5 Bxf3 17. Bxf3 g5 18. Bxe4 Rxe4 19. Bxg5 Qe5 20. Bf4 Qf5
21. Qg4+ Qg6 22. Qxd7 Rae8 23. Qg4 Bd4 24. Qxg6+ hxg6 25. Rab1 b6 26. Be3 Bg7
27. Rb4 R4e5 28. Nf4 Ra5 29. a4 Rc8 30. c4 g5 31. Nd5 Bf8 32. Nf6+ Kg7 33. Nh5+
Kg6 34. c5 Kxh5 35. g4+ Kg6 36. Re4 Bxc5 37. Rc1 Rd8 38. Bxc5 Rxc5 39. Rxc5 bxc5
40. Rc4 Rc8 41. Kg2 f5 42. f3 f4 43. Kf2 Kf6 44. Ke2 Ke5 45. Re4+ Kf6 46. Kd3 a5
47. Kc4 Rc7 48. Re8 Kf7 49. Re5 Kf6 50. Rxc5 Rxc5+ 51. Kxc5 Ke5 52. Kb5 Kd4 53.
Kxa5 Ke3 54. Kb6 Kxf3 55. a5 Kg3 56. a6 f3 57. a7 f2 58. a8=Q f1=Q 59. Qb8+ Kxh3
60. Qh8+ Kxg4 61. Qg7 Qb1+ 62. Kc5 Qf5+ 63. Kd4 Qf2+ 64. Kd5 Qa2+ 65. Kc6 Qa6+
66. Kd5 Qd3+ 67. Kc6 Qf3+ 68. Kb5 Qf5+ 69. Kc6 Kf4 70. Qc3 Qc8+ 0-1"#;
    let props = create_chess_board_props(pgn).unwrap();

    rsx! {
      ChessGame { chess: props.chess, previous_positions: props.previous_positions }
    }
}
