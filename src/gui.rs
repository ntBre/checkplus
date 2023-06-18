use eframe::App;
use egui::{vec2, Color32, Frame, Pos2, Rect, Rounding, Style};

use crate::board::{piece::Piece, Board};

pub(crate) struct MyApp {
    board: Board,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::canvas(&Style::default()).fill(Color32::WHITE).show(
                ui,
                |ui| {
                    let desired_width = 0.5 * ui.available_width();
                    let (_id, rect) =
                        ui.allocate_space(vec2(desired_width, desired_width));
                    let ymin = rect.top();
                    let ymax = rect.bottom();
                    let xmin = rect.left();
                    let xmax = rect.right();
                    let square_width = (xmax - xmin) / 8.0;
                    let square_height = (ymax - ymin) / 8.0;

                    let mut colors =
                        [Color32::WHITE, Color32::BROWN].into_iter().cycle();
                    let mut color = colors.next().unwrap();

                    for row in 0..8 {
                        for col in 0..8 {
                            let x = col as f32 * square_width + xmin;
                            let y = row as f32 * square_height + ymin;
                            ui.painter().rect_filled(
                                Rect {
                                    min: Pos2::new(x, y),
                                    max: Pos2::new(
                                        x + square_width,
                                        y + square_height,
                                    ),
                                },
                                Rounding::none(),
                                color,
                            );
                            color = colors.next().unwrap();
                        }
                        color = colors.next().unwrap();
                    }
                    // let to_screen = emath::RectTransform::from_to(
                    //     Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
                    //     rect,
                    // );
                },
            );
        });
    }
}

#[allow(unused)]
impl MyApp {
    pub(crate) fn new(board: Board) -> Self {
        Self { board }
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
