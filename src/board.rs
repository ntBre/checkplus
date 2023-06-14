use std::ops::{Index, IndexMut};

use piece::Piece;

use crate::pgn::mov::Move;

mod display;
pub(crate) mod file;
mod index;
pub use index::Coord;
mod piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    King {
        can_castle_kingside: bool,
        can_castle_queenside: bool,
    },
    Queen,
    Rook {
        has_moved: bool,
    },
    Bishop,
    Knight,
    Pawn,
}

impl From<char> for PieceType {
    fn from(c: char) -> Self {
        match c {
            'K' | 'O' => Self::King {
                can_castle_kingside: true,
                can_castle_queenside: true,
            },
            'Q' => Self::Queen,
            'R' => Self::Rook { has_moved: false },
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
            black![
                Rook { has_moved: false },
                Knight,
                Bishop,
                Queen,
                King {
                    can_castle_queenside: true,
                    can_castle_kingside: true
                },
                Bishop,
                Knight,
                Rook { has_moved: false }
            ],
            black![Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            [P::None; 8],
            [P::None; 8],
            [P::None; 8],
            [P::None; 8],
            white![Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            white![
                Rook { has_moved: false },
                Knight,
                Bishop,
                Queen,
                King {
                    can_castle_queenside: true,
                    can_castle_kingside: true
                },
                Bishop,
                Knight,
                Rook { has_moved: false }
            ],
        ];
        Self {
            board,
            half_move_clock: 0,
            en_passant_target: None,
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
                    }
                    Color::White => {
                        self.swap(E1, G1); // King
                        self.swap(H1, F1); // Rook
                    }
                }
            }
            Move::QueenCastle => {
                self.half_move_clock += 1;
                match c {
                    Color::Black => {
                        self.swap(E8, C8); // King
                        self.swap(A8, D8); // Rook
                    }
                    Color::White => {
                        self.swap(E1, C1); // King
                        self.swap(A1, D1); // Rook
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
                            c,
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
    fn fen_castle_field(&self, col: Color) -> &str {
        let white_king = self
            .board
            .iter()
            .flatten()
            .find(|p| match p {
                Piece::Some { typ, color } => typ.is_king() && *color == col,
                Piece::None => false,
            })
            .expect("have to have a king");
        let Piece::Some { typ, .. } = white_king else {
	            unreachable!()
	        };
        let PieceType::King { can_castle_kingside, can_castle_queenside } = typ else {
	            unreachable!()
	        };
        match (can_castle_kingside, can_castle_queenside) {
            (true, true) => "kq",
            (true, false) => "k",
            (false, true) => "q",
            (false, false) => "",
        }
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

        let white_castle = self.fen_castle_field(Color::White);
        ret.push_str(&white_castle.to_ascii_uppercase());

        let black_castle = self.fen_castle_field(Color::Black);
        ret.push_str(black_castle);

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
        ret.push(char::from_digit((half_move / 2) as u32 + 1, 10).unwrap());
        ret
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::pgn::mov::Move;

    use super::*;

    #[test]
    fn fen_starting_position() {
        let board = Board::new();
        let got = board.fen(0);
        let want = String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn fen_e4() {
        let mut board = Board::new();
        board.make_move(&"e4".parse().unwrap(), Color::White);
        let got = board.fen(1);
        let want = String::from(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn fen_black_c5() {
        let mut board = Board::new();
        board.make_move(&Move::from_str("e4").unwrap(), Color::White);
        board.make_move(&Move::from_str("c5").unwrap(), Color::Black);
        let got = board.fen(2);
        let want = String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn fen_2_nf3() {
        let mut board = Board::new();
        board.make_move(&Move::from_str("e4").unwrap(), Color::White);
        board.make_move(&Move::from_str("c5").unwrap(), Color::Black);
        board.make_move(&Move::from_str("Nf3").unwrap(), Color::White);
        let got = board.fen(3);
        let want = String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        );
        assert_eq!(got, want);
    }
}
