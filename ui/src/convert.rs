use dioxus::prelude::*;
use shakmaty::{Color, Piece, Role};

use crate::components::{
    black_bishop::BlackBishop, black_king::BlackKing, black_knight::BlackKnight,
    black_pawn::BlackPawn, black_queen::BlackQueen, black_rook::BlackRook, white_king::WhiteKing, white_rook::WhiteRook,
};

pub fn piece_to_component(piece: Piece) -> Element {
    match (piece.color, piece.role) {
        (Color::White, Role::King) => rsx! { WhiteKing {} },
        
        // (Color::White, Role::Queen) => rsx! { WhiteQueen {} },
        (Color::White, Role::Rook) => rsx! { WhiteRook {} },
        // (Color::White, Role::Bishop) => rsx! { WhiteBishop {} },
        // (Color::White, Role::Knight) => rsx! { WhiteKnight {} },
        // (Color::White, Role::Pawn) => rsx! { WhitePawn {} },
        (Color::Black, Role::King) => rsx! { BlackKing {} },
        (Color::Black, Role::Queen) => rsx! { BlackQueen {} },
        (Color::Black, Role::Rook) => rsx! { BlackRook {} },
        (Color::Black, Role::Bishop) => rsx! { BlackBishop {} },
        (Color::Black, Role::Knight) => rsx! { BlackKnight {} },
        (Color::Black, Role::Pawn) => rsx! { BlackPawn {} },
        _ => rsx! { WhiteKing {} },
    }
}
