use std::{collections::HashMap, io::Read};

use eframe::App;
use egui::{
    plot::{Line, Plot, PlotBounds, PlotPoints},
    pos2, vec2, Color32, ColorImage, Frame, Pos2, Rect, Rounding, Style,
    TextureHandle,
};
use egui_extras::{image::load_svg_bytes_with_size, Column, TableBuilder};

use crate::{
    board::{self, piece::Piece, Board, Color, PieceType},
    pgn::Game,
};

pub(crate) struct MyApp {
    board: Board,

    game: Game,

    /// the current move in `game`, set to `None` once the game is finished
    cur_move: Option<usize>,

    cur_color: Color,

    /// cache of previous boards for going backwards through a game
    boards: Vec<Board>,

    scores: Vec<[f64; 2]>,

    /// maximum absolute score in `scores`
    score_max: f64,

    /// map of piece SVGs, initialized when `self` is created.
    piece_images: HashMap<Piece, ColorImage>,

    /// map of piece textures, created as needed by [Self::draw_board].
    pieces: HashMap<Piece, TextureHandle>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // key handling
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowRight)) {
            if let Some(m) = self.cur_move {
                self.make_move(m);
            }
        }
        if ctx.input(|i| i.key_pressed(egui::Key::ArrowLeft)) {
            if let Some(m) = self.cur_move {
                if m != 0 {
                    self.board = self.boards[m - 1].clone();
                    self.cur_move = Some(m - 1);
                    self.cur_color = self.cur_color.other();
                }
            } else {
                let m = self.game.moves.len() - 1;
                self.board = self.boards[m].clone();
                self.cur_move = Some(m);
            }
        }

        // top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                Frame::canvas(&Style::default()).fill(Color32::WHITE).show(
                    ui,
                    |ui| {
                        let desired_width = 0.5 * ui.available_width();
                        let (_id, rect) = ui
                            .allocate_space(vec2(desired_width, desired_width));
                        self.draw_board(rect, ui);
                    },
                );
                TableBuilder::new(ui)
                    .column(Column::auto())
                    .column(Column::auto())
                    .column(Column::auto())
                    .body(|mut body| {
                        let mut moves =
                            self.game.moves.clone().into_iter().array_chunks();
                        let mut i = 1;
                        for [w, b] in moves.by_ref() {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{i}"));
                                });
                                row.col(|ui| {
                                    if ui.button(format!("{w}")).clicked() {
                                        let n = 2 * (i - 1) + 1;
                                        self.get_board(n);
                                        self.board = self.boards[n].clone();
                                        self.cur_move = Some(n);
                                        self.cur_color = Color::Black;
                                    }
                                });
                                row.col(|ui| {
                                    if ui.button(format!("{b}")).clicked() {
                                        let n = 2 * i;
                                        self.get_board(n);
                                        self.board = self.boards[n].clone();
                                        self.cur_move = Some(n);
                                        self.cur_color = Color::White;
                                    }
                                });
                                i += 1;
                            });
                        }
                        match moves.into_remainder() {
                            Some(mut n) => match n.next() {
                                Some(w) => {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("{i}"));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{w}"));
                                        });
                                    });
                                }
                                None => (),
                            },
                            None => (),
                        }
                    });
            });
            Plot::new("game scores").show(ui, |plot_ui| {
                let min = (1.2 * self.score_max).min(10.0);
                plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                    [0.0, -min],
                    [self.scores.len() as f64, min],
                ));
                plot_ui.line(
                    Line::new(PlotPoints::new(self.scores.clone()))
                        .color(Color32::from_rgb(200, 100, 100)),
                );
            });
        });
    }
}

impl MyApp {
    pub(crate) fn new(board: Board, game: Game, scores: Vec<f64>) -> Self {
        let mut piece_images = HashMap::new();

        for c in ['b', 'w'] {
            let color = match c {
                'b' => board::Color::Black,
                'w' => board::Color::White,
                _ => unreachable!(),
            };
            for piece in ['B', 'K', 'N', 'P', 'Q', 'R'] {
                let typ = PieceType::from(piece);
                let filename = format!("assets/{c}{piece}.svg");
                let mut f = std::fs::File::open(filename).unwrap();
                let mut buf = Vec::new();
                f.read_to_end(&mut buf).unwrap();
                let p = Piece::Some { typ, color };
                let data = load_svg_bytes_with_size(
                    &buf,
                    egui_extras::image::FitTo::Zoom(3.0),
                )
                .unwrap();
                piece_images.insert(p, data);
            }
        }

        let mut out = Vec::with_capacity(scores.len());
        let mut score_max = scores[0];
        for (i, s) in scores.into_iter().enumerate() {
            out.push([i as f64, s]);
            if s.abs() > score_max {
                score_max = s.abs();
            }
        }

        Self {
            board: board.clone(),
            piece_images,
            pieces: HashMap::new(),
            game,
            scores: out,
            score_max,
            cur_move: Some(0),
            cur_color: Color::White,
            boards: vec![board],
        }
    }

    fn draw_board(&mut self, Rect { min, max }: Rect, ui: &mut egui::Ui) {
        let Pos2 { x: xmin, y: ymin } = min;
        let Pos2 { x: xmax, y: ymax } = max;
        let square_width = (xmax - xmin) / 8.0;
        let square_height = (ymax - ymin) / 8.0;
        let mut colors = [Color32::WHITE, Color32::BROWN].into_iter().cycle();
        let mut color = colors.next().unwrap();
        for rank in 0..8 {
            let rank = 7 - rank;
            for file in 0..8 {
                let x = file as f32 * square_width + xmin;
                let y = rank as f32 * square_height + ymin;
                let rect = Rect::from_min_max(
                    Pos2::new(x, y),
                    Pos2::new(x + square_width, y + square_height),
                );
                ui.painter().rect_filled(rect, Rounding::none(), color);
                color = colors.next().unwrap();

                match self.board[(7 - rank, file)] {
                    p @ Piece::Some { .. } => {
                        let texture =
                            self.pieces.entry(p).or_insert_with(|| {
                                ui.ctx().load_texture(
                                    "black rook",
                                    self.piece_images.get(&p).unwrap().clone(),
                                    Default::default(),
                                )
                            });
                        ui.painter().image(
                            texture.into(),
                            rect,
                            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                            Color32::WHITE,
                        );
                    }
                    Piece::None => (),
                }
            }
            color = colors.next().unwrap();
        }
    }

    /// make the `m`th move on `self.board`, keeping `cur_color`, `cur_move`,
    /// and `boards` up to date
    fn make_move(&mut self, m: usize) {
        self.board.make_move(&self.game.moves[m], self.cur_color);
        if self.boards.len() <= m + 2 {
            self.boards.resize(m + 2, Board::default());
        }
        self.boards[m + 1] = self.board.clone();
        if m + 1 < self.game.moves.len() {
            self.cur_move = Some(m + 1);
            self.cur_color = self.cur_color.other();
        } else {
            self.cur_move = None;
        }
    }

    /// get the `n`th board from self.boards or make moves until it can be
    /// gotten
    fn get_board(&mut self, n: usize) {
        if self.boards.get(n).is_none() {
            let mut cur = self.cur_move.unwrap();
            while cur <= n + 1 {
                self.make_move(cur);
                cur += 1;
            }
        }
    }
}
