#![feature(array_chunks, let_chains, lazy_cell)]

use std::sync::LazyLock;
use std::time::Instant;

use clap::{arg, value_parser, Command};

use crate::board::{Board, Color};
use crate::pgn::Pgn;
use crate::stockfish::Stockfish;

pub mod board;
mod pgn;
mod stockfish;

struct Args {
    depth: usize,
    input: Pgn,
}

impl Args {
    fn new() -> Self {
        let args = Command::new("checkplus")
            .arg(
                arg!(-d --depth <DEPTH> "Set the search depth")
                    .value_parser(value_parser!(usize))
                    .default_value("20"),
            )
            .arg(arg!([input] "PGN file to score"))
            .get_matches();
        let depth = *args.get_one::<usize>("depth").unwrap();
        let input = args.get_one::<String>("input");
        let input = match input {
            Some(f) => Pgn::load(f).unwrap(),
            None => Pgn::read(&mut std::io::stdin()).unwrap(),
        };
        if input.games.is_empty() {
            eprintln!("no games in input");
            std::process::exit(0);
        }
        Self { depth, input }
    }
}

static DEBUG: LazyLock<bool> =
    LazyLock::new(|| std::env::var("CHECK_PLUS_DEBUG").is_ok());

const PROGRAM_TITLE: &'static str = "checkplus";

mod gui {
    use fltk::{
        enums::{Color, Shortcut},
        prelude::*,
        window::Window,
        *,
    };

    use crate::PROGRAM_TITLE;

    pub(crate) struct MyApp {
        app: app::App,
    }

    fn menu_cb(m: &mut impl MenuExt) {
        if let Some(choice) = m.choice() {
            match choice.as_str() {
                "New\t" => println!("New"),
                "Open\t" => println!("Open"),
                "Third" => println!("Third"),
                "Quit\t" => {
                    println!("Quitting");
                    app::quit();
                }
                _ => println!("{}", choice),
            }
        }
    }

    impl MyApp {
        #[allow(unused)]
        fn menubar() {
            let mut menubar = menu::SysMenuBar::new(0, 0, 40, 40, "rew");
            menubar.global();
            menubar.add(
                "File/New\t",
                Shortcut::None,
                menu::MenuFlag::Normal,
                menu_cb,
            );
        }

        pub fn new() -> Self {
            let app = app::App::default();

            let mut win = Window::new(100, 100, 800, 600, PROGRAM_TITLE);
            win.make_resizable(false);

            let mut draw_window =
                Window::default().with_size(400, 400).center_of(&win);

            draw_window.draw(|f| {
                use draw::*;

                let width = f.w();
                let height = f.h();
                draw_rect_fill(0, 0, width, height, enums::Color::White);

                let square_height = height as usize / 8;
                let square_width = width as usize / 8;
                let mut colors = [Color::White, Color::Black].iter().cycle();
                let mut color = colors.next().unwrap();
                for row in (0..height).step_by(square_height) {
                    for col in (0..width).step_by(square_width) {
                        draw_rect_fill(
                            col,
                            row,
                            square_width as i32,
                            square_height as i32,
                            *color,
                        );
                        color = colors.next().unwrap();
                    }
                    color = colors.next().unwrap();
                }
            });

            win.end();
            win.show();

            Self { app }
        }

        pub fn run(self) {
            self.app.run().unwrap();
        }
    }
}

#[allow(unused)]
fn main() {
    let app = gui::MyApp::new();
    app.run();
    return;
    let args = Args::new();
    let mut stockfish = Stockfish::new();

    for (g, pgn) in args.input.games.iter().enumerate() {
        let (w, b) = pgn.players();
        eprintln!("starting game {}: {} - {}", g + 1, w, b);
        let now = Instant::now();

        let mut board = Board::new();
        stockfish.new_game();
        stockfish.start_position();

        let mut cur = &Color::White;
        let score = stockfish.get_score(args.depth, *cur);
        println!("0 {score}");

        let mut to_move = [Color::Black, Color::White].iter().cycle();
        for (i, m) in pgn.moves.iter().enumerate() {
            let i = i + 1;
            board.make_move(m, *cur);
            cur = to_move.next().unwrap();
            let fen = board.fen(i);
            stockfish.set_position(&fen);
            let score = stockfish.get_score(args.depth, *cur);
            print!("{i} {score:.2}");
            if *DEBUG {
                println!(" {fen}");
            } else {
                println!();
            }
        }

        eprintln!(
            "finished game {} after {:.1} sec\n",
            g + 1,
            now.elapsed().as_millis() as f64 / 1000.0
        );
    }
}
