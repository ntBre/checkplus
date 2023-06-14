use std::fmt::Display;
use std::io::BufReader;
use std::io::{BufRead, Write};
use std::process::ChildStdout;
use std::process::Command;
use std::process::Stdio;
use std::process::{Child, ChildStdin};

use crate::board::Color;

pub(crate) struct Stockfish {
    child: Child,
    pub(crate) stdin: ChildStdin,
    pub(crate) stdout: BufReader<ChildStdout>,
}

impl Stockfish {
    pub(crate) fn new() -> Self {
        let mut cmd = Command::new("stockfish");
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            child,
            stdin,
            stdout,
        }
    }

    /// write `cmd` to stockfish's stdin
    pub(crate) fn send<D>(&mut self, cmd: D)
    where
        D: Display,
    {
        writeln!(self.stdin, "{}", cmd).unwrap();
    }

    /// loop over self.output until the pattern `until` is encountered and
    /// return the resulting output
    pub(crate) fn receive(&mut self, until: &'static str) -> String {
        let mut s = String::new();
        let mut buf = String::new();
        while self.stdout.read_line(&mut buf).is_ok() {
            s.push_str(&buf);
            s.push('\n');
            if buf.starts_with(until) {
                break;
            }
            buf.clear();
            match self.child.try_wait() {
                Ok(None) => {}
                Ok(Some(status)) => {
                    panic!("stockfish exited with {status}");
                }
                Err(e) => panic!("failed to get status with `{e}`"),
            }
        }
        s
    }

    /// set stockfish's position
    pub(crate) fn set_position(&mut self, fen: impl Display) {
        self.send(format!("position fen {fen}"));
    }

    /// score the current position to `depth` for the the player `to_move`
    pub(crate) fn get_score(&mut self, depth: usize, to_move: Color) -> f64 {
        self.send(format!("go depth {}", depth));
        let output = self.receive("bestmove");
        let mut score = 0.0;
        for line in output.split('\n') {
            if line.starts_with("info") {
                let mut sp = line.split_ascii_whitespace();
                // not found on line saying NNUE is enabled
                if sp.any(|s| s == "cp") {
                    let text = sp.next().unwrap();
                    score = text.parse::<f64>().unwrap();
                }
            }
        }

        if to_move.is_black() {
            score *= -1.0;
        }

        // stockfish reports the score as an integer in units of centipawns
        score / 100.0
    }
}
