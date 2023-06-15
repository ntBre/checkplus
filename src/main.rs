#![feature(array_chunks, let_chains, lazy_cell)]

use std::sync::LazyLock;
use std::time::Instant;

use clap::{arg, value_parser, Command};

use crate::board::{Board, Color};
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

pub mod board;
mod pgn;
mod stockfish;

struct Args {
    depth: usize,
    input: Pgn,
}

impl Args {
    fn new() -> Self {
        let args = Command::new("checkplus")
            .arg(
                arg!(-d --depth <DEPTH> "Set the search depth")
                    .value_parser(value_parser!(usize))
                    .default_value("20"),
            )
            .arg(arg!([input] "PGN file to score"))
            .get_matches();
        let depth = *args.get_one::<usize>("depth").unwrap();
        let input = args.get_one::<String>("input");
        let input = match input {
            Some(f) => Pgn::load(f).unwrap(),
            None => Pgn::read(&mut std::io::stdin()).unwrap(),
        };
        if input.games.is_empty() {
            eprintln!("no games in input");
            std::process::exit(0);
        }
        Self { depth, input }
    }
}

static DEBUG: LazyLock<bool> =
    LazyLock::new(|| std::env::var("CHECK_PLUS_DEBUG").is_ok());

fn main() {
    let args = Args::new();
    let mut stockfish = Stockfish::new();

    for (g, pgn) in args.input.games.iter().enumerate() {
        let (w, b) = pgn.players();
        eprintln!("starting game {}: {} - {}", g + 1, w, b);
        let now = Instant::now();

        let mut board = Board::new();
        stockfish.new_game();
        stockfish.start_position();

        let mut cur = &Color::White;
        let score = stockfish.get_score(args.depth, *cur);
        println!("0 {score}");

        let mut to_move = [Color::Black, Color::White].iter().cycle();
        for (i, m) in pgn.moves.iter().enumerate() {
            let i = i + 1;
            board.make_move(m, *cur);
            cur = to_move.next().unwrap();
            let fen = board.fen(i);
            stockfish.set_position(&fen);
            let score = stockfish.get_score(args.depth, *cur);
            print!("{i} {score:.2}");
            if *DEBUG {
                println!(" {fen}");
            } else {
                println!();
            }
        }

        eprintln!(
            "finished game {} after {:.1} sec\n",
            g + 1,
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }
}
