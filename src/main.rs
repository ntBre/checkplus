#![feature(array_chunks)]

use crate::board::Board;
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

mod board;
mod pgn;
mod stockfish;

fn main() {
    let pgn = Pgn::load("testfiles/sample.pgn").unwrap();
    println!("{pgn:?}");
    let board = Board::new();
    println!("{board}");

    let mut stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive("readyok");

    let fen = "8/7p/4p3/8/3k4/2p5/4R1KP/8; w - - 0 43";

    stockfish.set_position(fen);

    let score = stockfish.get_score(20);
    println!("score = {score}");
}
