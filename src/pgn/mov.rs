use crate::board::file;
use crate::board::PieceType;

#[derive(Debug, PartialEq)]
pub enum Move {
    Normal {
        /// the type of the piece involved
        typ: PieceType,

        from_rank: Option<usize>,
        from_file: Option<usize>,

        dest_rank: usize,
        dest_file: file::File,
    },
    KingCastle,
    QueenCastle,
}

mod from_str {
    use crate::{
        board::{
            file::{self, File},
            PieceType,
        },
        pgn::mov::Move,
    };
    use core::str::FromStr;

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            // skip check and mate indicators
            let chars: Vec<_> =
                s.chars().filter(|&x| !"+#".contains(x)).collect();
            let typ = if !chars[0].is_ascii_uppercase() {
                PieceType::Pawn
            } else {
                PieceType::from(chars[0])
            };

            match typ {
                PieceType::King { .. } => {
                    if s == "O-O" {
                        Ok(Self::KingCastle)
                    } else if s == "O-O-O" {
                        Ok(Self::QueenCastle)
                    } else {
                        knight_move(chars, typ)
                    }
                }
                PieceType::Queen => knight_move(chars, typ),
                PieceType::Rook { .. } => knight_move(chars, typ),
                PieceType::Bishop => bishop_move(chars, typ),
                PieceType::Knight => knight_move(chars, typ),
                PieceType::Pawn => pawn_move(chars, typ),
            }
        }
    }

    fn bishop_move(chars: Vec<char>, typ: PieceType) -> Result<Move, ()> {
        knight_move(chars, typ)
    }

    fn knight_move(chars: Vec<char>, typ: PieceType) -> Result<Move, ()> {
        if chars.len() == 3 {
            let (dest_file, dest_rank) = pawn_dest(&chars[1..]);
            return Ok(Move::Normal {
                typ,
                from_rank: None,
                from_file: None,
                dest_rank,
                dest_file,
            });
        } else if chars.len() == 4 {
            // disambiguating file, eg Nbd7
            let from_file = if let Ok(file) = File::try_from(chars[1]) {
                Some(file as usize)
            } else {
                None
            };
            let from_rank = char::to_digit(chars[1], 10).map(|r| r as usize);
            let (dest_file, dest_rank) = pawn_dest(&chars[2..]);
            return Ok(Move::Normal {
                typ,
                from_rank,
                from_file,
                dest_rank,
                dest_file,
            });
        }
        eprintln!("chars = {chars:?}");
        Err(())
    }

    fn pawn_dest(chars: &[char]) -> (file::File, usize) {
        (
            file::File::from_unchecked(chars[0]),
            char::to_digit(chars[1], 10).unwrap() as usize - 1,
        )
    }

    fn pawn_move(chars: Vec<char>, typ: PieceType) -> Result<Move, ()> {
        if chars.len() == 2 {
            let (dest_file, dest_rank) = pawn_dest(&chars);
            return Ok(Move::Normal {
                typ,
                from_file: Some(dest_file as usize),
                from_rank: None,
                dest_file,
                dest_rank,
            });
        } else if chars.len() == 4 {
            // split 'exf4' into 'e' and 'f4'
            let mut res = chars.split(|c| *c == 'x');
            let x = res.next().unwrap()[0];
            let from_file = Some(File::from_unchecked(x) as usize);
            let y = res.next().unwrap();
            let (dest_file, dest_rank) = pawn_dest(y);
            return Ok(Move::Normal {
                typ,
                from_rank: None,
                from_file,
                dest_rank,
                dest_file,
            });
        }
        eprintln!("chars = {chars:?}");
        Err(())
    }

    #[cfg(test)]
    mod tests {
        use crate::board::file::File;

        use super::*;
        use PieceType::*;

        #[test]
        fn e4() {
            let got = Move::from_str("e4").unwrap();
            let want = Move::Normal {
                typ: Pawn,
                from_rank: None,
                from_file: Some(4),
                dest_rank: 3,
                dest_file: File::E,
            };
            assert_eq!(got, want);
        }

        #[test]
        fn c5() {
            let got = Move::from_str("c5").unwrap();
            let want = Move::Normal {
                typ: Pawn,
                from_rank: None,
                from_file: Some(2),
                dest_rank: 4,
                dest_file: File::C,
            };
            assert_eq!(got, want);
        }
    }
}
