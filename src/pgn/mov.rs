use crate::board::file;
use crate::board::PieceType;

#[derive(Clone, Debug, PartialEq)]
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

mod display {
    use std::fmt::Display;

    use super::Move;

    impl Display for Move {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Move::Normal {
                    typ,
                    from_rank,
                    from_file,
                    dest_rank,
                    dest_file,
                } => {
                    let mut s = String::new();
                    let t = char::from(*typ);
                    // skip pawn
                    if t != 'P' {
                        s.push(t);
                        if let Some(n) = from_file {
                            let c = match *n {
                                0 => 'a',
                                1 => 'b',
                                2 => 'c',
                                3 => 'd',
                                4 => 'e',
                                5 => 'f',
                                6 => 'g',
                                7 => 'h',
                                _ => unimplemented!(),
                            };
                            s.push(c);
                        }
                    }
                    if let Some(n) = from_rank {
                        s.push(char::from_digit(*n as u32 + 1, 10).unwrap());
                    }
                    s.push((*dest_file).into());
                    s.push(
                        char::from_digit(*dest_rank as u32 + 1, 10).unwrap(),
                    );
                    write!(f, "{s}")
                }
                Move::KingCastle => write!(f, "O-O"),
                Move::QueenCastle => write!(f, "O-O-O"),
            }
        }
    }
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
                // capture
                None
            };
            // disambiguating rank, eg N2f4
            let from_rank = char::to_digit(chars[1], 10).map(|r| r as usize);
            let (dest_file, dest_rank) = pawn_dest(&chars[2..]);
            return Ok(Move::Normal {
                typ,
                from_rank,
                from_file,
                dest_rank,
                dest_file,
            });
        } else if chars.len() == 5 {
            // both a rank and a capture, eg Ng6f4
            let (f, r) = pawn_dest(&chars[1..]);
            let (dest_file, dest_rank) = pawn_dest(&chars[3..]);
            return Ok(Move::Normal {
                typ,
                from_rank: Some(r),
                from_file: Some(f as usize),
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

        #[test]
        fn disambiguation() {
            let got = Move::from_str("Ng6f4").unwrap();
            let want = Move::Normal {
                typ: Knight,
                from_rank: Some(5),
                from_file: Some(6),
                dest_rank: 3,
                dest_file: File::F,
            };
            assert_eq!(got, want);
        }
    }
}
