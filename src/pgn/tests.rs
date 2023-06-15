use super::*;
use crate::board::file::File::*;
use crate::board::PieceType::*;
use crate::pgn::mov::Move::KingCastle;
use crate::pgn::mov::Move::Normal;

#[test]
fn load_single() {
    let got = Pgn::load("testfiles/sample.pgn").unwrap();
    let want = include!("../../testfiles/sample.want");
    assert_eq!(got, want);
}

#[test]
fn load_multi() {
    let got = Pgn::load("testfiles/multi.pgn").unwrap();
    // std::fs::write("testfiles/multi.want", format!("{got:#?}")).unwrap();
    let want = include!("../../testfiles/multi.want");
    assert_eq!(got, want);
}
