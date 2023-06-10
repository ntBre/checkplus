use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::{self, JoinHandle};

#[allow(unused)]
struct Stockfish {
    sender: SyncSender<&'static str>,
    output: Receiver<String>,
    child: Child,
    to_stockfish: JoinHandle<()>,
    from_stockfish: JoinHandle<()>,
}

impl Stockfish {
    fn new() -> Self {
        let mut cmd = Command::new("stockfish");
        let mut child = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stdout = BufReader::new(stdout);

        let (sender, receiver) = sync_channel(1);

        // this thread sends things to stockfish
        let to_stockfish = thread::spawn(move || {
            for r in receiver {
                writeln!(stdin, "{r}").unwrap();
            }
        });

        let (so, output) = sync_channel(1);
        let from_stockfish = thread::spawn(move || {
            let mut s = String::new();
            for line in stdout.lines().flatten() {
                s.push_str(dbg!(&line));
                s.push('\n');
                if line == "readyok" {
                    so.send(std::mem::take(&mut s)).unwrap();
                }
            }
        });

        Self {
            sender,
            output,
            child,
            to_stockfish,
            from_stockfish,
        }
    }

    fn send(&self, cmd: &'static str) {
        self.sender.send(cmd).unwrap();
        self.sender.send("isready").unwrap();
    }

    fn receive(&self) -> String {
        self.output.recv().unwrap()
    }

    fn _quit(self) {
        self.send("quit");
        self.child.wait_with_output().unwrap();
        self.to_stockfish.join().unwrap();
        self.from_stockfish.join().unwrap();
    }
}

fn main() {
    let stockfish = Stockfish::new();

    stockfish.send("isready");
    stockfish.receive();

    stockfish.send("position fen 8/7p/4p3/8/3k4/2p5/4R1KP/8; w - - 0 43");
    stockfish.receive();

    stockfish.send("go depth 5");
    stockfish.receive();
}
