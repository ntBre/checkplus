use crate::board::Color;
use crate::board::PieceType;

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
    pub fn to_char(&self) -> Option<char> {
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
}
