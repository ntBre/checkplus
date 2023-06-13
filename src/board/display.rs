use super::Board;
use std::fmt::Debug;
use std::fmt::Display;

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board {
            for p in row {
                write!(f, "{p}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{self}")
    }
}
