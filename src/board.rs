use piece::Piece;

use crate::pgn::mov::Move;

mod display;
pub(crate) mod file;
mod index;
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
            'K' => Self::King {
                can_castle_kingside: true,
                can_castle_queenside: true,
            },
            'Q' => Self::Queen,
            'R' => Self::Rook { has_moved: false },
            'B' => Self::Bishop,
            'N' => Self::Knight,
            _ => todo!(),
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

    pub(crate) fn make_move(
        &mut self,
        Move {
            typ: t,
            from_rank,
            from_file,
            dest_rank,
            dest_file,
        }: &Move,
        c: Color,
    ) {
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
                if let Some(from_rank) = from_rank && *from_rank != rank {
		    continue;
		}
                if let Some(from_file) = from_file && *from_file != file {
		    continue;
		}
                // at this point we know the piece type, color, and possibly
                // from_rank/from_file are correct, so we just have to verify
                // that the piece on this square can make a legal move to
                // (dest_file, dest_rank)
                if p.can_move(
                    self,
                    rank,
                    file,
                    *dest_rank,
                    *dest_file as usize,
                    c,
                ) {
                    self[(*dest_rank, *dest_file as usize)] =
                        std::mem::take(&mut self[(rank, file)]);
                    return;
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
