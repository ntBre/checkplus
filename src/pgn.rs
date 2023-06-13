use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub mod mov;

#[derive(Debug)]
pub struct Pgn {
    pub moves: Vec<mov::Move>,
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
                mov::Move::from_str(w).unwrap(),
                mov::Move::from_str(b).unwrap(),
            ]);
        }
        match chunks.remainder() {
            [_] => {}
            [_, w] => moves.push(mov::Move::from_str(w).unwrap()),
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
        Pgn::load("testfiles/sample.pgn").unwrap();
    }
}
