use crate::board::Color;
use crate::board::PieceType;

use super::Board;

mod display;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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
    ) -> bool {
        board.en_passant_target = None;
        let Self::Some { typ, color } = self else {
            return false;
        };
        match typ {
            PieceType::King => {
                // cast as isize to prevent underflow
                let from_rank = from_rank as isize;
                let dest_rank = dest_rank as isize;
                let from_file = from_file as isize;
                let dest_file = dest_file as isize;
                match color {
                    Color::Black => {
                        board.black_can_castle_kingside = false;
                        board.black_can_castle_queenside = false;
                    }
                    Color::White => {
                        board.white_can_castle_kingside = false;
                        board.white_can_castle_queenside = false;
                    }
                };
                if from_rank + 1 == dest_rank && from_file + 1 == dest_file {
                    return true;
                }
                if from_rank + 1 == dest_rank && from_file == dest_file {
                    return true;
                }
                if from_rank + 1 == dest_rank && from_file - 1 == dest_file {
                    return true;
                }
                if from_rank - 1 == dest_rank && from_file + 1 == dest_file {
                    return true;
                }
                if from_rank - 1 == dest_rank && from_file == dest_file {
                    return true;
                }
                if from_rank - 1 == dest_rank && from_file - 1 == dest_file {
                    return true;
                }
                if from_rank == dest_rank && from_file + 1 == dest_file {
                    return true;
                }
                if from_rank == dest_rank && from_file - 1 == dest_file {
                    return true;
                }
            }
            PieceType::Queen => {
                let bishop = Self::Some {
                    typ: PieceType::Bishop,
                    color: *color,
                };
                let rook = Self::Some {
                    typ: PieceType::Rook,
                    color: *color,
                };
                return bishop.can_move(
                    board, from_rank, from_file, dest_rank, dest_file,
                ) || rook.can_move(
                    board, from_rank, from_file, dest_rank, dest_file,
                );
            }
            PieceType::Rook => {
                use crate::board::Coord::*;
                // it's okay to set these again if the rook happens to return to
                // its starting square. *_can_castle_* start off true and will
                // never be set back to true once they've been set to false
                match color {
                    Color::Black => match (from_file, from_rank).into() {
                        A8 => board.black_can_castle_queenside = false,
                        H8 => board.black_can_castle_kingside = false,
                        _ => {}
                    },
                    Color::White => match (from_file, from_rank).into() {
                        A1 => board.white_can_castle_queenside = false,
                        H1 => board.white_can_castle_kingside = false,
                        _ => {}
                    },
                }
                if from_rank == dest_rank {
                    let (beg, end) =
                        (from_file.min(dest_file), from_file.max(dest_file));
                    for file in beg + 1..end {
                        if board[(from_rank, file)].is_some() {
                            return false;
                        }
                    }
                } else if from_file == dest_file {
                    let (beg, end) =
                        (from_rank.min(dest_rank), from_rank.max(dest_rank));
                    for rank in beg + 1..end {
                        if board[(rank, from_file)].is_some() {
                            return false;
                        }
                    }
                } else {
                    return false;
                }
                return true;
            }
            PieceType::Bishop => {
                {
                    // cast as isize to prevent underflow
                    let from_rank = from_rank as isize;
                    let dest_rank = dest_rank as isize;
                    let from_file = from_file as isize;
                    let dest_file = dest_file as isize;
                    if (dest_rank - from_rank).abs()
                        != (dest_file - from_file).abs()
                    {
                        return false;
                    }
                }
                match (dest_file > from_file, dest_rank > from_rank) {
                    (true, true) => {
                        let end = dest_file - from_file;
                        for i in 1..end {
                            if board[(from_rank + i, from_file + i)].is_some() {
                                return false;
                            }
                        }
                        return from_rank + end == dest_rank
                            && from_file + end == dest_file;
                    }
                    (true, false) => {
                        let end = dest_file - from_file;
                        for i in 1..end {
                            if board[(from_rank - i, from_file + i)].is_some() {
                                return false;
                            }
                        }
                        return from_rank - end == dest_rank
                            && from_file + end == dest_file;
                    }
                    (false, true) => {
                        let end = dest_rank - from_rank;
                        for i in 1..end {
                            if board[(from_rank + i, from_file - i)].is_some() {
                                return false;
                            }
                        }
                        return from_rank + end == dest_rank
                            && from_file - end == dest_file;
                    }
                    (false, false) => {
                        let end = from_file - dest_file;
                        for i in 1..end {
                            if board[(from_rank - i, from_file - i)].is_some() {
                                return false;
                            }
                        }
                        return from_rank - end == dest_rank
                            && from_file - end == dest_file;
                    }
                };
            }
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
