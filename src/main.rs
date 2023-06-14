#![feature(array_chunks, let_chains)]

use crate::board::{Board, Color};
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

pub mod board;
mod pgn;
mod stockfish;

const DEPTH: usize = 20;

fn main() {
    let pgn = Pgn::load("testfiles/sample.pgn").unwrap();
    let mut board = Board::new();

    let mut stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive("readyok");

    stockfish.set_position(board.fen(0));

    let mut cur = &Color::White;
    let score = stockfish.get_score(DEPTH, *cur);
    println!("0 {score}");

    let mut to_move = [Color::Black, Color::White].iter().cycle();

    for (i, m) in pgn.moves.iter().enumerate() {
        let i = i + 1;
        board.make_move(m, *cur);
        cur = to_move.next().unwrap();
        let fen = board.fen(i);
        stockfish.set_position(fen);
        let score = stockfish.get_score(DEPTH, *cur);
        println!("{i} {score:.2}");
    }
}
