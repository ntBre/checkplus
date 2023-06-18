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

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
			    "eframe",
			    "https://github.com/emilk/egui/tree/master/crates/eframe",
			);
                    ui.label(".");
                });
            });
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
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
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
