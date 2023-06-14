use crate::board::Color;
use crate::board::PieceType;

use super::Board;

mod display;

#[derive(Clone, Copy, Debug, Default)]
pub enum Piece {
    Some {
        typ: PieceType,
        color: Color,
    },
    #[default]
    None,
}

impl Piece {
    /// Returns `true` if the piece is [`Some`].
    ///
    /// [`Some`]: Piece::Some
    #[must_use]
    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some { .. })
    }

    pub fn to_char(self) -> Option<char> {
        match self {
            Piece::Some { typ, color } => {
                let c = match typ {
                    PieceType::King { .. } => 'k',
                    PieceType::Queen => 'q',
                    PieceType::Rook { .. } => 'r',
                    PieceType::Bishop => 'b',
                    PieceType::Knight => 'n',
                    PieceType::Pawn => 'p',
                };
                if color.is_white() {
                    return c.to_uppercase().next();
                }
                Some(c)
            }
            Piece::None => None,
        }
    }

    pub fn can_move(
        &self,
        board: &mut Board,
        from_rank: usize,
        from_file: usize,
        dest_rank: usize,
        dest_file: usize,
        color: Color,
    ) -> bool {
        board.en_passant_target = None;
        let Self::Some { typ, .. } = self else {
	    return false;
	};
        match typ {
            PieceType::King { .. } => todo!(),
            PieceType::Queen => todo!(),
            PieceType::Rook { .. } => todo!(),
            PieceType::Bishop => todo!(),
            PieceType::Knight => {
                // cast as isize to prevent underflow
                let from_rank = from_rank as isize;
                let dest_rank = dest_rank as isize;
                let from_file = from_file as isize;
                let dest_file = dest_file as isize;
                if from_rank + 2 == dest_rank && from_file + 1 == dest_file {
                    return true;
                }
                if from_rank + 1 == dest_rank && from_file + 2 == dest_file {
                    return true;
                }
                if from_rank - 2 == dest_rank && from_file + 1 == dest_file {
                    return true;
                }
                if from_rank - 1 == dest_rank && from_file + 2 == dest_file {
                    return true;
                }
                if from_rank + 2 == dest_rank && from_file - 1 == dest_file {
                    return true;
                }
                if from_rank + 1 == dest_rank && from_file - 2 == dest_file {
                    return true;
                }
                if from_rank - 2 == dest_rank && from_file - 1 == dest_file {
                    return true;
                }
                if from_rank - 1 == dest_rank && from_file - 2 == dest_file {
                    return true;
                }
            }
            PieceType::Pawn => {
                let start_square = (color.is_white() && from_rank == 1)
                    || (color.is_black() && from_rank == 6);

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

                if start_square && op(from_rank, 2) == dest_rank {
                    board.en_passant_target =
                        Some((ep(dest_rank + 1, 1), from_file));
                    return true;
                } else if op(from_rank, 1) == dest_rank {
                    return true;
                }
            }
        }
        false
    }
}
