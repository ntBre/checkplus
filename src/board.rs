use std::{default, fmt::Display};

use crate::{board::file::File, pgn::Move};

mod display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// Returns `true` if the color is [`Black`].
    ///
    /// [`Black`]: Color::Black
    #[must_use]
    pub fn is_black(&self) -> bool {
        matches!(self, Self::Black)
    }

    /// Returns `true` if the color is [`White`].
    ///
    /// [`White`]: Color::White
    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Self::White)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Piece {
    Some {
        typ: PieceType,
        color: Color,
    },
    #[default]
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

mod file {
    #[derive(Clone, Copy, Debug)]
    #[repr(u8)]
    pub enum File {
        A = 0,
        B = 1,
        C = 2,
        D = 3,
        E = 4,
        F = 5,
        G = 6,
        H = 7,
    }

    impl From<char> for File {
        fn from(c: char) -> Self {
            match c {
                'a' => File::A,
                'b' => File::B,
                'c' => File::C,
                'd' => File::D,
                'e' => File::E,
                'f' => File::F,
                'g' => File::G,
                'h' => File::H,
                _ => unreachable!(),
            }
        }
    }
}

/// construct a row of black pieces
macro_rules! black {
    ($($pt:ident $(,)*)*) => {
	[$(Piece::Some { typ: PieceType::$pt, color: Color::Black },)*]
    }
}

/// construct a row of white pieces
macro_rules! white {
    ($($pt:ident $(,)*)*) => {
	[$(Piece::Some { typ: PieceType::$pt, color: Color::White },)*]
    }
}

pub struct Board {
    board: [[Piece; 8]; 8],
}

mod index {
    use super::{Board, Piece};
    use std::ops::{Index, IndexMut};

    impl Index<(usize, usize)> for Board {
        type Output = Piece;

        fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
            &self.board[i][j]
        }
    }

    impl IndexMut<(usize, usize)> for Board {
        fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
            &mut self.board[i][j]
        }
    }
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

    pub(crate) fn make_move(&mut self, m: &Move) {
        // TODO factor this into Move itself. should parse into whatever I need
        // here from the beginning
        match m.mov.len() {
            2 => {
                // forward pawn move
                let chars: Vec<_> = m.mov.chars().collect();
                let file = File::from(chars[0]);
                let rank = chars[1].to_digit(10).unwrap() - 1;
                for i in 0..8 {
                    for j in 0..8 {
                        let p = self[(i, j)];
                        let Piece::Some{ typ, color } = p else {
			    continue;
			};
                        let start_square = (color.is_white() && i == 6)
                            || (color.is_black() && i == 1);
                        let op = if color.is_white() {
                            std::ops::Sub::sub
                        } else {
                            std::ops::Add::add
                        };

                        // we know it's not a pawn capture, so the pawn must be
                        // in the same file
                        if file as usize == j
                            && color == m.color
                            && typ == PieceType::Pawn
                            && ((start_square && op(j, 2) == rank as usize)
                                || op(j, 1) == rank as usize)
                        {
                            // we found the right piece, but now I need to make
                            // the move. pretty sure I can't do it inside of
                            // this because I'm iterating over board
                            //
                            // rank and file give the target square, ij give the
                            // original
                            self[(file as usize, rank as usize)] =
                                std::mem::take(&mut self[(i, j)]);
                            return;
                        }
                    }
                }
            }
            3 if m.mov.contains("-") => {
                // short castle
            }
            3 => {
                // regular piece move
            }
            4 if m.mov.contains("x") => {
                // capture
            }
            4 => {
                // disambiguating moves like Ref7
            }
            5 => {
                // long castle
                assert_eq!(m.mov, "O-O-O");
            }
            _ => unimplemented!(),
        }
        todo!();
    }

    /// return the FEN representation of `self`
    pub(crate) fn fen(&self) -> String {
        todo!()
    }
}
