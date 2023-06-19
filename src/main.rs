#![feature(iter_array_chunks, array_chunks, let_chains, lazy_cell)]

use std::sync::LazyLock;
use std::time::Instant;

use clap::{arg, value_parser, Command};

use crate::board::{Board, Color};
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

pub mod board;
mod gui;
mod pgn;
mod stockfish;

struct Args {
    depth: usize,
    gui: bool,
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
            .arg(
                arg!(-g --gui "Run the GUI")
                    .value_parser(value_parser!(bool))
                    .default_value("false"),
            )
            .arg(arg!([input] "PGN file to score"))
            .get_matches();
        let depth = *args.get_one::<usize>("depth").unwrap();
        let gui = *args.get_one::<bool>("gui").unwrap();
        let input = args.get_one::<String>("input");
        let input = match input {
            Some(f) => Pgn::load(f).unwrap(),
            None => Pgn::read(&mut std::io::stdin()).unwrap(),
        };
        if input.games.is_empty() {
            eprintln!("no games in input");
            std::process::exit(0);
        }
        Self { depth, gui, input }
    }
}

static DEBUG: LazyLock<bool> =
    LazyLock::new(|| std::env::var("CHECK_PLUS_DEBUG").is_ok());

const PROGRAM_TITLE: &str = "checkplus";

fn score_game(
    stockfish: &mut Stockfish,
    game: &pgn::Game,
    depth: usize,
) -> Vec<f64> {
    let mut ret = Vec::with_capacity(game.moves.len());
    let mut board = Board::new();
    stockfish.new_game();
    stockfish.start_position();
    let mut cur = &Color::White;
    let score = stockfish.get_score(depth, *cur);
    println!("0 {score}");
    let mut to_move = [Color::Black, Color::White].iter().cycle();
    for (i, m) in game.moves.iter().enumerate() {
        let i = i + 1;
        board.make_move(m, *cur);
        cur = to_move.next().unwrap();
        let fen = board.fen(i);
        stockfish.set_position(&fen);
        let score = stockfish.get_score(depth, *cur);
        ret.push(score);
        print!("{i} {score:.2}");
        if *DEBUG {
            println!(" {fen}");
        } else {
            println!();
        }
    }
    ret
}

fn main() {
    let args = Args::new();

    if args.gui {
        let game = args.input.games[0].clone();
        // let mut stockfish = Stockfish::new();
        // let scores = score_game(&mut stockfish, &game, args.depth);
        let scores = vec![
            0.37, 0.35, 0.37, 0.26, 0.37, 0.28, 0.41, 0.45, 0.49, 0.50, 0.55,
            0.52, 0.48, 0.62, 0.55, 0.50, 0.47, 0.45, 0.50, 0.39, 0.39, 0.41,
            0.40, 0.01, 0.13, 0.13, 0.44, 0.26, 0.26, 0.26, 0.18, -0.07, -0.16,
            -0.13, -0.15, -0.18, -0.19, -0.52, -0.58, -0.67, -0.75, -1.16,
            -1.03, -1.10, -0.66, -0.67, -0.60, -0.71, -0.78, -0.65, -0.56,
            -0.77, -0.75, -1.10, -1.00, -0.95, -0.97, -0.96, -1.04, -1.22,
            -1.20, -1.11, -1.33, -1.31, -1.30, -1.16, -0.57, -0.67, -0.34,
            -0.61, -0.63, -1.49, -1.48, -1.39, -0.42, -0.36, -0.03, -0.00,
            0.00, -0.00, 0.00, -0.00, -0.05, -0.11, 0.00, -0.09,
        ];
        eframe::run_native(
            PROGRAM_TITLE,
            eframe::NativeOptions::default(),
            Box::new(|_cc| {
                Box::new(gui::MyApp::new(Board::new(), game, scores))
            }),
        )
        .unwrap();
        return;
    }

    let mut stockfish = Stockfish::new();

    for (g, pgn) in args.input.games.iter().enumerate() {
        let (w, b) = pgn.players();
        eprintln!("starting game {}: {} - {}", g + 1, w, b);
        let now = Instant::now();

        score_game(&mut stockfish, pgn, args.depth);

        eprintln!(
            "finished game {} after {:.1} sec\n",
            g + 1,
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }
}
