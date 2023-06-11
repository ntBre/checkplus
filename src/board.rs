use std::fmt::Display;

#[derive(Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
enum Color {
    Black,
    White,
}

macro_rules! black {
    ($($pt:ident $(,)*)*) => {
	[$(Piece::Some { typ: PieceType::$pt, color: Color::Black },)*]
    }
}

macro_rules! white {
    ($($pt:ident $(,)*)*) => {
	[$(Piece::Some { typ: PieceType::$pt, color: Color::White },)*]
    }
}

#[derive(Clone, Copy)]
enum Piece {
    Some { typ: PieceType, color: Color },
    None,
}

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
                    (PieceType::Rook, Color::Black) => "♜",
                    (PieceType::King, Color::Black) => "♚",
                    (PieceType::Bishop, Color::White) => "♗",
                    (PieceType::Knight, Color::White) => "♘",
                    (PieceType::Pawn, Color::White) => "♙",
                    (PieceType::Queen, Color::White) => "♕",
                    (PieceType::Rook, Color::White) => "♖",
                    (PieceType::King, Color::White) => "♔",
                }
            ),
            Piece::None => write!(f, " "),
        }
    }
}

pub struct Board {
    board: [[Piece; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        use Piece as P;
        let board = [
            black![Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
            black![Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            [P::None; 8],
            [P::None; 8],
            [P::None; 8],
            [P::None; 8],
            white![Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            white![Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
        ];
        Self { board }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board {
            for p in row {
                write!(f, "{p}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
