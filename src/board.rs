use crate::{board::file::File, pgn::Move};
use piece::Piece;

mod display;
mod file;
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

type Square = (usize, usize);

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

    pub(crate) fn make_move(&mut self, m: &Move) {
        // TODO factor this into Move itself. should parse into whatever I need
        // here from the beginning
        self.en_passant_target = None;
        let chars: Vec<_> = m.mov.chars().collect();
        match chars.len() {
            2 => self.forward_pawn_move(&chars, m).unwrap(),
            3 if m.mov.contains("-") => {
                // short castle
            }
            3 => {
                // regular piece move
                let typ = PieceType::from(chars[0]);
                let file = File::from(chars[1]);
                let rank = chars[2].to_digit(10).unwrap() - 1;
                dbg!(typ, file, rank);
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
        };
    }

    fn forward_pawn_move(
        &mut self,
        chars: &Vec<char>,
        m: &Move,
    ) -> Result<(), &str> {
        let file = File::from(chars[0]);
        let rank = chars[1].to_digit(10).unwrap() - 1;
        for i in 0..8 {
            for j in 0..8 {
                let p = self[(i, j)];
                let Piece::Some{ typ, color } = p else {
			            continue;
			        };
                let start_square = (color.is_white() && i == 1)
                    || (color.is_black() && i == 6);

                let op = if color.is_white() {
                    std::ops::Add::add
                } else {
                    std::ops::Sub::sub
                };

                // en passant rank operator
                let ep = if color.is_white() {
                    std::ops::Sub::sub
                } else {
                    std::ops::Add::add
                };

                // we know it's not a pawn capture, so the pawn must be
                // in the same file
                if file as usize == j
                    && color == m.color
                    && typ == PieceType::Pawn
                {
                    let rank = rank as usize;
                    let file = file as usize;
                    // rank and file give the target square, ij give the
                    // original
                    if start_square && op(i, 2) == rank {
                        self.en_passant_target = Some((ep(rank + 1, 1), file));
                        self[(rank, file)] = std::mem::take(&mut self[(i, j)]);
                        return Ok(());
                    } else if op(i, 1) == rank {
                        self[(rank, file)] = std::mem::take(&mut self[(i, j)]);
                        return Ok(());
                    }
                }
            }
        }
        Err("expected to make a pawn move")
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
        let castle = match (can_castle_kingside, can_castle_queenside) {
            (true, true) => "kq",
            (true, false) => "k",
            (false, true) => "q",
            (false, false) => "",
        };
        castle
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
        ret.push_str(&black_castle);

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
            ret.push_str("-");
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
        board.make_move(&Move::new("e4", Color::White));
        let got = board.fen(1);
        let want = String::from(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn fen_black_c5() {
        let mut board = Board::new();
        board.make_move(&Move::new("e4", Color::White));
        board.make_move(&Move::new("c5", Color::Black));
        let got = board.fen(2);
        let want = String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn fen_2_nf3() {
        let mut board = Board::new();
        board.make_move(&Move::new("e4", Color::White));
        board.make_move(&Move::new("c5", Color::Black));
        board.make_move(&Move::new("Nf3", Color::White));
        let got = board.fen(3);
        let want = String::from(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        );
        assert_eq!(got, want);
    }
}
