use crate::board::piece::Piece;
use crate::board::Color;
use crate::board::PieceType;
use std::fmt::Display;

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Piece::Some { typ, color } => write!(
                f,
                "{}",
                match (typ, color) {
                    (PieceType::Bishop, Color::Black) => "♝",
                    (PieceType::Knight, Color::Black) => "♞",
                    (PieceType::Pawn, Color::Black) => "♟",
                    (PieceType::Queen, Color::Black) => "♛",
                    (PieceType::Rook { .. }, Color::Black) => "♜",
                    (PieceType::King { .. }, Color::Black) => "♚",
                    (PieceType::Bishop, Color::White) => "♗",
                    (PieceType::Knight, Color::White) => "♘",
                    (PieceType::Pawn, Color::White) => "♙",
                    (PieceType::Queen, Color::White) => "♕",
                    (PieceType::Rook { .. }, Color::White) => "♖",
                    (PieceType::King { .. }, Color::White) => "♔",
                }
            ),
            Piece::None => write!(f, " "),
        }
    }
}
