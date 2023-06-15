use super::*;
use crate::board::file::File::*;
use crate::board::PieceType::*;
use crate::pgn::mov::Move::KingCastle;
use crate::pgn::mov::Move::Normal;

#[test]
fn load_single() {
    let got = Pgn::load("testfiles/sample.pgn").unwrap();
    // std::fs::write("testfiles/sample.want", format!("{got:#?}")).unwrap();
    let want = include!("../../testfiles/sample.want");
    assert_eq!(got.moves.len(), want.moves.len());
    for (i, (g, w)) in got.moves.iter().zip(&want.moves).enumerate() {
        if g != w {
            panic!("mismatch at {i}: got\n{g:#?}, want\n{w:#?}");
        }
    }
    assert_eq!(got.result, want.result);
    assert_eq!(got.tags, want.tags);
    assert_eq!(got, want);
}

#[test]
fn load_multi() {
    let got = Pgn::load("testfiles/multi.pgn").unwrap();
    println!("{got:#?}");
}
