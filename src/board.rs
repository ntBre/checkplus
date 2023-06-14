use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use piece::Piece;

use crate::pgn::mov::Move;

mod display;
pub(crate) mod file;
mod index;
pub use index::Coord;
mod piece;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl From<char> for PieceType {
    fn from(c: char) -> Self {
        match c {
            'K' | 'O' => Self::King,
            'Q' => Self::Queen,
            'R' => Self::Rook,
            'B' => Self::Bishop,
            'N' => Self::Knight,
            _ => todo!("what is this? {c}"),
        }
    }
}

impl PieceType {
    /// Returns `true` if the piece type is [`King`].
    ///
    /// [`King`]: PieceType::King
    #[must_use]
    pub fn is_king(&self) -> bool {
        matches!(self, Self::King { .. })
    }

    /// Returns `true` if the piece type is [`Pawn`].
    ///
    /// [`Pawn`]: PieceType::Pawn
    #[must_use]
    pub fn is_pawn(&self) -> bool {
        matches!(self, Self::Pawn)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Black => write!(f, "B"),
            Color::White => write!(f, "W"),
        }
    }
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

/// construct a row of black pieces
macro_rules! black {
    ($($pt:expr $(,)*)*) => {
	[$(Piece::Some { typ: $pt, color: Color::Black },)*]
    }
}

/// construct a row of white pieces
macro_rules! white {
    ($($pt:expr $(,)*)*) => {
	[$(Piece::Some { typ: $pt, color: Color::White },)*]
    }
}

pub(crate) type Square = (usize, usize);

pub struct Board {
    board: [[Piece; 8]; 8],
    half_move_clock: usize,
    en_passant_target: Option<Square>,
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        use piece::Piece as P;
        use PieceType::*;
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
        Self {
            board,
            half_move_clock: 0,
            en_passant_target: None,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
        }
    }

    /// move the piece in the square `from` to `to`, leaving `self[to]` empty
    fn swap<T>(&mut self, from: T, to: T)
    where
        Self: Index<T> + IndexMut<T>,
        <Self as Index<T>>::Output: Default,
    {
        self[to] = std::mem::take(&mut self[from]);
    }

    pub(crate) fn make_move(&mut self, m: &Move, c: Color) {
        use Coord::*;
        match m {
            Move::KingCastle => {
                self.half_move_clock += 1;
                match c {
                    Color::Black => {
                        self.swap(E8, G8); // King
                        self.swap(H8, F8); // Rook
                        self.black_can_castle_kingside = false;
                        self.black_can_castle_queenside = false;
                    }
                    Color::White => {
                        self.swap(E1, G1); // King
                        self.swap(H1, F1); // Rook
                        self.white_can_castle_kingside = false;
                        self.white_can_castle_queenside = false;
                    }
                }
            }
            Move::QueenCastle => {
                self.half_move_clock += 1;
                match c {
                    Color::Black => {
                        self.swap(E8, C8); // King
                        self.swap(A8, D8); // Rook
                        self.black_can_castle_kingside = false;
                        self.black_can_castle_queenside = false;
                    }
                    Color::White => {
                        self.swap(E1, C1); // King
                        self.swap(A1, D1); // Rook
                        self.white_can_castle_kingside = false;
                        self.white_can_castle_queenside = false;
                    }
                }
            }
            Move::Normal {
                typ: t,
                from_rank,
                from_file,
                dest_rank,
                dest_file,
            } => {
                for rank in 0..8 {
                    for file in 0..8 {
                        let p = self[(rank, file)];
                        // skip empty square
                        let Piece::Some{ typ, color } = p else {
			    continue;
			};
                        // skip piece type or color mismatch
                        if typ != *t || color != c {
                            continue;
                        }
                        // skip from square mismatch
                        if let Some(fr) = from_rank && *fr != rank {
			    continue;
			}
                        if let Some(ff) = from_file && *ff != file {
			    continue;
			}
                        // at this point we know the piece type, color, and
                        // possibly from_rank/from_file are correct, so we just
                        // have to verify that the piece on this square can make
                        // a legal move to (dest_file, dest_rank)
                        if p.can_move(
                            self,
                            rank,
                            file,
                            *dest_rank,
                            *dest_file as usize,
                        ) {
                            // destination is occupied => capture; pawn move is
                            // always an advance or capture
                            if self[(*dest_rank, *dest_file as usize)].is_some()
                                || typ.is_pawn()
                            {
                                self.half_move_clock = 0;
                            } else {
                                self.half_move_clock += 1;
                            }
                            self.swap(
                                (rank, file),
                                (*dest_rank, *dest_file as usize),
                            );
                            return;
                        }
                    }
                }
            }
        }
    }

    /// locate the king of `col` and determine its castling rights
    fn fen_castle_field(&self) -> String {
        let mut ret = String::new();
        if self.white_can_castle_kingside {
            ret.push('K')
        }
        if self.white_can_castle_queenside {
            ret.push('Q')
        }
        if self.black_can_castle_kingside {
            ret.push('k')
        }
        if self.black_can_castle_queenside {
            ret.push('q')
        }
        ret
    }

    /// return the FEN representation of `self`
    pub(crate) fn fen(&self, half_move: usize) -> String {
        let mut ret = String::new();
        for (i, row) in self.board.iter().enumerate() {
            let mut empty = 0;
            for p in row {
                if let Some(c) = p.to_char() {
                    if empty != 0 {
                        ret.push(char::from_digit(empty, 10).unwrap());
                        empty = 0;
                    }
                    ret.push(c);
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                ret.push(char::from_digit(empty, 10).unwrap());
            }
            if i < 7 {
                ret.push('/');
            } else {
                ret.push(' ');
            }
        }
        if (half_move & 1) == 0 {
            ret.push('w');
        } else {
            ret.push('b');
        }
        ret.push(' ');

        ret.push_str(&self.fen_castle_field());

        ret.push(' ');

        if let Some((rank, file)) = self.en_passant_target {
            ret.push(match file {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => unreachable!(),
            });
            ret.push(char::from_digit(rank as u32, 10).unwrap());
        } else {
            ret.push('-');
        }

        ret.push(' ');
        ret.push(char::from_digit(self.half_move_clock as u32, 10).unwrap());

        ret.push(' ');
        use std::fmt::Write;
        write!(ret, "{}", half_move / 2 + 1).unwrap();
        ret
    }
}
