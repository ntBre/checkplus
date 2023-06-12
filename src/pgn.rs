use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::board::Color;

#[derive(Debug)]
pub struct Move {
    pub(crate) mov: String,
    pub(crate) color: Color,
}

impl Move {
    pub fn new(mov: impl Into<String>, color: Color) -> Self {
        let mov = mov.into().replace("+", "");
        Self { mov, color }
    }
}

#[derive(Debug)]
pub struct Pgn {
    pub moves: Vec<Move>,
    pub result: String,
}

impl Pgn {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::open(path)?;
        let r = BufReader::new(f);
        let mut moves = Vec::new();
        for line in r
            .lines()
            .flatten()
            .skip_while(|s| s.starts_with("[") || s.is_empty())
        {
            for l in line.split_ascii_whitespace() {
                moves.push(l.to_owned());
            }
        }

        // delete the result from the end
        let result = moves.pop().expect("failed to load empty PGN");

        let mut chunks = moves.array_chunks::<3>();
        let mut moves = Vec::new();
        while let Some([_, w, b]) = chunks.next() {
            moves.extend([
                Move::new(w, Color::White),
                Move::new(b, Color::Black),
            ]);
        }
        match chunks.remainder() {
            [_] => {}
            [_, w] => moves.push(Move::new(w, Color::White)),
            // if there were 3, chunks would have gotten it, and I already
            // covered 1 and 2
            _ => unreachable!(),
        }
        Ok(Self { moves, result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        dbg!(Pgn::load("testfiles/sample.pgn").unwrap());
    }
}
