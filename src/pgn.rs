use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
struct Move(String);

#[derive(Debug)]
struct Pgn {
    moves: Vec<Move>,
    result: String,
}

impl Pgn {
    fn load(path: impl AsRef<Path>) -> io::Result<Self> {
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
            moves.extend([Move(w.clone()), Move(b.clone())]);
        }
        match chunks.remainder() {
            [_] => {}
            [_, w] => moves.push(Move(w.clone())),
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
