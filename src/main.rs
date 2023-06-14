#![feature(array_chunks, let_chains)]

use clap::{arg, value_parser, Command};

use crate::board::{Board, Color};
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

pub mod board;
mod pgn;
mod stockfish;

struct Args {
    depth: usize,
    input: String,
}

impl Args {
    fn new() -> Self {
        let args = Command::new("checkplus")
            .arg(
                arg!(-d --depth <DEPTH> "Set the search depth")
                    .value_parser(value_parser!(usize))
                    .default_value("20"),
            )
            .arg(arg!(<input> "PGN file to score"))
            .get_matches();
        let depth = *args.get_one::<usize>("depth").unwrap();
        let input = args.get_one::<String>("input").unwrap().to_owned();
        Self { depth, input }
    }
}

fn main() {
    let args = Args::new();

    let pgn = Pgn::load(args.input).unwrap();
    let mut board = Board::new();

    let mut stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive("readyok");

    stockfish.set_position(board.fen(0));

    let mut cur = &Color::White;
    let score = stockfish.get_score(args.depth, *cur);
    println!("0 {score}");

    let mut to_move = [Color::Black, Color::White].iter().cycle();

    for (i, m) in pgn.moves.iter().enumerate() {
        let i = i + 1;
        board.make_move(m, *cur);
        cur = to_move.next().unwrap();
        let fen = board.fen(i);
        stockfish.set_position(fen);
        let score = stockfish.get_score(args.depth, *cur);
        println!("{i} {score:.2}");
    }
}
