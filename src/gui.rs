use std::{collections::HashMap, io::Read};

use eframe::App;
use egui::{
    plot::{Line, Plot, PlotBounds, PlotPoints},
    pos2, vec2, Color32, ColorImage, Frame, Pos2, Rect, Rounding, Style,
    TextureHandle,
};
use egui_extras::{image::load_svg_bytes_with_size, Column, TableBuilder};

use crate::{
    board::{self, piece::Piece, Board, PieceType},
    pgn::Game,
};

pub(crate) struct MyApp {
    board: Board,

    game: Game,

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
                        let mut moves = self.game.moves.iter().array_chunks();
                        let mut i = 1;
                        for [w, b] in moves.by_ref() {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{i}"));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{w}"));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{b}"));
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
                    [(self.scores.len() / 2) as f64, min],
                ));
                plot_ui.line(
                    Line::new(PlotPoints::new(self.scores.clone()))
                        .color(Color32::from_rgb(200, 100, 100))
                        .name("wave"),
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
            board,
            piece_images,
            pieces: HashMap::new(),
            game,
            scores: out,
            score_max,
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
}
