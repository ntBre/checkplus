use std::str::FromStr;

use crate::pgn::mov::Move;

use super::*;

#[test]
fn fen_starting_position() {
    let board = Board::new();
    let got = board.fen(0);
    let want = String::from(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    );
    assert_eq!(got, want);
}

#[test]
fn fen_e4() {
    let mut board = Board::new();
    board.make_move(&"e4".parse().unwrap(), Color::White);
    let got = board.fen(1);
    let want = String::from(
        "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
    );
    assert_eq!(got, want);
}

#[test]
fn fen_black_c5() {
    let mut board = Board::new();
    board.make_move(&Move::from_str("e4").unwrap(), Color::White);
    board.make_move(&Move::from_str("c5").unwrap(), Color::Black);
    let got = board.fen(2);
    let want = String::from(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
    );
    assert_eq!(got, want);
}

#[test]
fn fen_2_nf3() {
    let mut board = Board::new();
    board.make_move(&Move::from_str("e4").unwrap(), Color::White);
    board.make_move(&Move::from_str("c5").unwrap(), Color::Black);
    board.make_move(&Move::from_str("Nf3").unwrap(), Color::White);
    let got = board.fen(3);
    let want = String::from(
        "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
    );
    assert_eq!(got, want);
}
