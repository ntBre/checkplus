use std::str::FromStr;

use crate::pgn::{mov::Move, Pgn};

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

#[test]
fn samples() {
    struct Test {
        take: usize,
        want: &'static str,
    }
    impl Test {
        fn new(take: usize, want: &'static str) -> Self {
            Self { take, want }
        }
    }
    let tests = [
        Test::new(
            1,
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ),
        Test::new(
            11,
            "r1bqk2r/1pppbppp/p1n2n2/4p3/B3P3/5N2/PPPP1PPP/RNBQR1K1 b kq - 5 6",
        ),
        Test::new(50, "6k1/1b3Np1/1n3q1p/2p5/1p6/7P/PP3PP1/R2Qr1K1 w - - 0 26"),
        Test::new(85, "8/8/4R1p1/2k3p1/1p4P1/1P1b1P2/3K1n2/8 b - - 2 43"),
    ];

    let game = &Pgn::load("testfiles/sample.pgn").unwrap().games[0];
    for test in tests {
        let mut board = Board::new();
        let mut to_move = [Color::White, Color::Black].into_iter().cycle();
        for m in game.moves.iter().take(test.take) {
            board.make_move(m, to_move.next().unwrap());
        }
        let got = board.fen(test.take);
        assert_eq!(got, test.want, "take = {}, board =\n{board}", test.take);
    }
}
