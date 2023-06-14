use super::{piece::Piece, Board};
use std::ops::{Index, IndexMut};

pub enum Coord {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

impl From<Coord> for (usize, usize) {
    fn from(value: Coord) -> Self {
        match value {
            Coord::A1 => (0, 0),
            Coord::A2 => (1, 0),
            Coord::A3 => (2, 0),
            Coord::A4 => (3, 0),
            Coord::A5 => (4, 0),
            Coord::A6 => (5, 0),
            Coord::A7 => (6, 0),
            Coord::A8 => (7, 0),
            Coord::B1 => (0, 1),
            Coord::B2 => (1, 1),
            Coord::B3 => (2, 1),
            Coord::B4 => (3, 1),
            Coord::B5 => (4, 1),
            Coord::B6 => (5, 1),
            Coord::B7 => (6, 1),
            Coord::B8 => (7, 1),
            Coord::C1 => (0, 2),
            Coord::C2 => (1, 2),
            Coord::C3 => (2, 2),
            Coord::C4 => (3, 2),
            Coord::C5 => (4, 2),
            Coord::C6 => (5, 2),
            Coord::C7 => (6, 2),
            Coord::C8 => (7, 2),
            Coord::D1 => (0, 3),
            Coord::D2 => (1, 3),
            Coord::D3 => (2, 3),
            Coord::D4 => (3, 3),
            Coord::D5 => (4, 3),
            Coord::D6 => (5, 3),
            Coord::D7 => (6, 3),
            Coord::D8 => (7, 3),
            Coord::E1 => (0, 4),
            Coord::E2 => (1, 4),
            Coord::E3 => (2, 4),
            Coord::E4 => (3, 4),
            Coord::E5 => (4, 4),
            Coord::E6 => (5, 4),
            Coord::E7 => (6, 4),
            Coord::E8 => (7, 4),
            Coord::F1 => (0, 5),
            Coord::F2 => (1, 5),
            Coord::F3 => (2, 5),
            Coord::F4 => (3, 5),
            Coord::F5 => (4, 5),
            Coord::F6 => (5, 5),
            Coord::F7 => (6, 5),
            Coord::F8 => (7, 5),
            Coord::G1 => (0, 6),
            Coord::G2 => (1, 6),
            Coord::G3 => (2, 6),
            Coord::G4 => (3, 6),
            Coord::G5 => (4, 6),
            Coord::G6 => (5, 6),
            Coord::G7 => (6, 6),
            Coord::G8 => (7, 6),
            Coord::H1 => (0, 7),
            Coord::H2 => (1, 7),
            Coord::H3 => (2, 7),
            Coord::H4 => (3, 7),
            Coord::H5 => (4, 7),
            Coord::H6 => (5, 7),
            Coord::H7 => (6, 7),
            Coord::H8 => (7, 7),
        }
    }
}

impl<T> Index<T> for Board
where
    T: Into<(usize, usize)>,
{
    type Output = Piece;

    fn index(&self, t: T) -> &Self::Output {
        let (i, j) = t.into();
        let i = 7 - i;
        &self.board[i][j]
    }
}

impl<T> IndexMut<T> for Board
where
    T: Into<(usize, usize)>,
{
    fn index_mut(&mut self, t: T) -> &mut Self::Output {
        let (i, j) = t.into();
        let i = 7 - i;
        &mut self.board[i][j]
    }
}
