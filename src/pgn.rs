use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    mem::take,
    path::Path,
    str::FromStr,
};

use self::mov::Move;

pub mod mov;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub moves: Vec<mov::Move>,
    pub result: String,
    pub tags: HashMap<String, String>,
}

impl Game {
    pub fn players(&self) -> (String, String) {
        let white = self
            .tags
            .get("White")
            .cloned()
            .unwrap_or_else(|| String::from("NN"));
        let black = self
            .tags
            .get("Black")
            .cloned()
            .unwrap_or_else(|| String::from("NN"));
        (white, black)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Pgn {
    pub games: Vec<Game>,
}

enum State {
    Tags,
    Moves,
}

impl State {
    fn is_tags(&self) -> bool {
        matches!(self, Self::Tags)
    }

    fn is_moves(&self) -> bool {
        matches!(self, Self::Moves)
    }
}

/// parse the movetext section of a game into a series of Moves and the game's
/// result
fn parse_movetext(game: String) -> (Vec<Move>, String) {
    let mut in_brackets = false;
    let mut in_parens = false;
    let mut ret = String::new();
    for c in game.chars() {
        if c == '{' {
            in_brackets = true;
        } else if in_brackets {
            if c == '}' {
                in_brackets = false;
            }
            continue;
        } else if c == '(' {
            in_parens = true;
        } else if in_parens {
            if c == ')' {
                in_parens = false;
            }
            continue;
        } else {
            ret.push(c);
        }
    }

    let mut chunks: Vec<_> = ret.split_ascii_whitespace().collect();

    // delete the result from the end
    let result = chunks.pop().expect("failed to load empty PGN");

    let mut moves = Vec::new();
    for m in chunks {
        if !m.starts_with(char::is_numeric) {
            moves.push(mov::Move::from_str(m).unwrap());
        }
    }

    (moves, result.to_owned())
}

impl Pgn {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let f = File::open(path)?;
        Self::read(f)
    }

    pub fn read(r: impl Read) -> Result<Pgn, io::Error> {
        let r = BufReader::new(r);
        let mut games = Vec::new();
        let mut game = String::new();
        let mut tags = HashMap::new();
        use State::*;
        let mut state = Tags;
        for line in r.lines().flatten() {
            if line.starts_with('[') {
                let line = line.replace(['[', ']'], "");
                let sp: Vec<_> = line.split_ascii_whitespace().collect();
                tags.insert(sp[0].to_owned(), sp[1..].join(" ").to_owned());
            } else if state.is_tags() && line.is_empty() {
                state = Moves;
            } else if state.is_moves() && line.is_empty() {
                state = Tags;
                let (moves, result) = parse_movetext(take(&mut game));
                games.push(Game {
                    moves,
                    result,
                    tags: take(&mut tags),
                });
            } else {
                game.push_str(&line);
                game.push(' '); // keep separation from newlines
            }
        }
        let (moves, result) = parse_movetext(game);
        games.push(Game {
            moves,
            result,
            tags,
        });
        Ok(Self { games })
    }
}
