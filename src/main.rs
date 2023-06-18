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
    gui: bool,
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
            .arg(
                arg!(-g --gui "Run the GUI")
                    .value_parser(value_parser!(bool))
                    .default_value("false"),
            )
            .arg(arg!([input] "PGN file to score"))
            .get_matches();
        let depth = *args.get_one::<usize>("depth").unwrap();
        let gui = *args.get_one::<bool>("gui").unwrap();
        let input = args.get_one::<String>("input");
        let input = match input {
            Some(f) => {
                let pgn = Pgn::load(f).unwrap();
                if pgn.games.is_empty() {
                    eprintln!("no games in input");
                    std::process::exit(0);
                }
                pgn
            }
            None => {
                if gui {
                    Pgn::default()
                } else {
                    Pgn::read(&mut std::io::stdin()).unwrap()
                }
            }
        };
        Self { depth, gui, input }
    }
}

static DEBUG: LazyLock<bool> =
    LazyLock::new(|| std::env::var("CHECK_PLUS_DEBUG").is_ok());

const _PROGRAM_TITLE: &'static str = "checkplus";

mod gui {
    use eframe::App;

    use crate::board::{piece::Piece, Board};

    pub(crate) struct MyApp {
        label: String,
        value: f32,
        board: Board,
    }

    impl App for MyApp {
        fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
            let Self { label, value, .. } = self;

            // Examples of how to create different panels and windows.
            // Pick whichever suits you.
            // Tip: a good default choice is to just keep the `CentralPanel`.
            // For inspiration and more examples, go to https://emilk.github.io/egui

            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                });
            });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Side Panel");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(label);
                });

                ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    *value += 1.0;
                }

                ui.with_layout(
                    egui::Layout::bottom_up(egui::Align::LEFT),
                    |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("powered by ");
                            ui.hyperlink_to(
                                "egui",
                                "https://github.com/emilk/egui",
                            );
                            ui.label(" and ");
                            ui.hyperlink_to(
			    "eframe",
			    "https://github.com/emilk/egui/tree/master/crates/eframe",
			);
                            ui.label(".");
                        });
                    },
                );
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and
                // SidePanel's

                ui.heading("eframe template");
                ui.hyperlink("https://github.com/emilk/eframe_template");
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/master/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });

            if false {
                egui::Window::new("Window").show(ctx, |ui| {
                    ui.label("Windows can be moved by dragging them.");
                    ui.label("They are automatically sized based on contents.");
                    ui.label(
                        "You can turn on resizing and scrolling if you like.",
                    );
                    ui.label(
                        "You would normally choose either panels OR windows.",
                    );
                });
            }
        }
    }

    #[allow(unused)]
    impl MyApp {
        pub(crate) fn new(board: Board) -> Self {
            Self {
                board,
                label: String::from("Hello world"),
                value: 3.14,
            }
        }

        pub fn run(self) {}

        /// draw the pieces in `b` onto the current widget (?)
        fn draw_board(&self) {
            for rank in 0..8 {
                for file in 0..8 {
                    match self.board[(rank, file)] {
                        p @ Piece::Some { color, .. } => {
                            // let t = p.to_char().unwrap().to_uppercase();
                            // let c = match color {
                            //     crate::board::Color::Black => 'b',
                            //     crate::board::Color::White => 'w',
                            // };
                            // let filename = format!("assets/{c}{t}.svg");
                            // let mut img = SvgImage::load(filename).unwrap();
                            // img.scale(
                            //     square_width as i32,
                            //     square_height as i32,
                            //     true,
                            //     true,
                            // );
                            // let rank = 7 - rank;
                            // img.draw(
                            //     (file * square_width) as i32,
                            //     (rank * square_width) as i32,
                            //     square_width as i32,
                            //     square_height as i32,
                            // );
                        }
                        Piece::None => (),
                    }
                }
            }
        }
    }
}

#[allow(unused)]
fn main() {
    let args = Args::new();

    if args.gui {
        let native_options = eframe::NativeOptions::default();
        let board = Board::new();
        eframe::run_native(
            "eframe template",
            native_options,
            Box::new(|cc| Box::new(gui::MyApp::new(board))),
        );
        return;
    }

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
