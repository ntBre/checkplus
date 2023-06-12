use super::{piece::Piece, Board};
use std::ops::{Index, IndexMut};

impl Index<(usize, usize)> for Board {
    type Output = Piece;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.board[i][j]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.board[i][j]
    }
}
