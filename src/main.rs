#![feature(array_chunks)]

use crate::board::{Board, Color};
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

    stockfish.set_position(dbg!(board.fen(0)));
    let score = stockfish.get_score(20, Color::White);
    println!("0 {score}");

    let mut to_move = [Color::Black, Color::White].iter().cycle();

    for (i, m) in pgn.moves.iter().enumerate() {
        let i = i + 1;
        board.make_move(m);
        let fen = board.fen(i);
        stockfish.set_position(dbg!(fen));
        let score = stockfish.get_score(20, *to_move.next().unwrap());
        println!("{i} {score}");
    }

    // let fen = "8/7p/4p3/8/3k4/2p5/4R1KP/8; w - - 0 43";
}
