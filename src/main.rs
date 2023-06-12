#![feature(array_chunks)]

use crate::board::Board;
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

mod board;
mod pgn;
mod stockfish;

fn main() {
    let pgn = Pgn::load("testfiles/sample.pgn").unwrap();
    let mut board = Board::new();

    let mut stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive("readyok");

    for (i, m) in pgn.moves.iter().enumerate() {
        board.make_move(m);
        let fen = board.fen(i);
        stockfish.set_position(dbg!(fen));
        let score = stockfish.get_score(20);
        println!("{i} {score}");
    }

    // let fen = "8/7p/4p3/8/3k4/2p5/4R1KP/8; w - - 0 43";
}
