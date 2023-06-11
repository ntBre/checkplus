#![feature(array_chunks)]

use std::fmt::Display;
use std::io::{BufRead, BufReader, Write};
use std::process::{ChildStdin, ChildStdout, Command, Stdio};

mod pgn;

struct Stockfish {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl Stockfish {
    fn new() -> Self {
        let mut cmd = Command::new("stockfish");
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self { stdin, stdout }
    }

    /// write `cmd` to stockfish's stdin
    fn send<D>(&mut self, cmd: D)
    where
        D: Display,
    {
        writeln!(self.stdin, "{}", cmd).unwrap();
    }

    /// loop over self.output until the pattern `until` is encountered and
    /// return the resulting output
    fn receive(&mut self, until: &'static str) -> String {
        let mut s = String::new();
        let mut buf = String::new();
        while let Ok(_) = self.stdout.read_line(&mut buf) {
            s.push_str(&buf);
            s.push('\n');
            if buf.starts_with(until) {
                break;
            }
            buf.clear();
        }
        return s;
    }

    /// set stockfish's position
    fn set_position(&mut self, fen: &'static str) {
        self.send(format!("position fen {fen}"));
    }

    /// score the current position to `depth`
    fn get_score(&mut self, depth: usize) -> f64 {
        self.send(format!("go depth {}", depth));
        let output = self.receive("bestmove");
        let mut score = 0.0;
        for line in output.split('\n') {
            if line.starts_with("info") {
                let mut sp = line.split_ascii_whitespace();
                // not found on line saying NNUE is enabled
                if let Some(_) = sp.position(|s| s == "cp") {
                    let text = sp.next().unwrap();
                    score = text.parse::<f64>().unwrap();
                }
            }
        }
        // stockfish reports the score as an integer in units of centipawns
        score / 100.0
    }
}

fn main() {
    let mut stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive("readyok");

    let fen = "8/7p/4p3/8/3k4/2p5/4R1KP/8; w - - 0 43";

    stockfish.set_position(fen);

    let score = stockfish.get_score(20);
    println!("score = {score}");
}
