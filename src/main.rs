#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //hide console window on Windows in release
use crate::egui::ViewportCommand;
use eframe::egui;
use eframe::egui::Vec2;
use eframe::egui::{Color32, RichText};
use std::time::{Duration, Instant};

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Minesweeper {
    wizard: Difficulty,
    bombs: u32,
    time: Option<Instant>,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            wizard: Difficulty::Easy,
            bombs: 10,
            time: Some(Instant::now()),
        }
    }
}

impl Minesweeper {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("top_down_menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Game", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Easy").clicked() {
                        //9x9 minefield - 10 bombs
                        self.wizard = Difficulty::Easy;
                        self.bombs = 10;
                        self.time = Some(Instant::now());
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            330.0, 380.0,
                        )));
                    }
                    if ui.button("Medium").clicked() {
                        //16x16 minefield - 40 bombs
                        self.wizard = Difficulty::Medium;
                        self.bombs = 40;
                        self.time = Some(Instant::now());
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            570.0, 605.0,
                        )))
                    }
                    if ui.button("Hard").clicked() {
                        //30x16 minefield - 99 bombs
                        self.wizard = Difficulty::Hard;
                        self.bombs = 99;
                        self.time = Some(Instant::now());
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            1060.0, 605.0,
                        )))
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = Vec2::new(5.0, 5.0);

            let (rows, cols) = match self.wizard {
                Difficulty::Easy => (9, 9),
                Difficulty::Medium => (16, 16),
                Difficulty::Hard => (16, 30),
            };

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("Bombs: {}", self.bombs))
                        .size(15.0)
                        .color(Color32::WHITE)
                        .monospace(),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let elapsed = match self.time {
                        Some(time) => time.elapsed().as_secs(),
                        None => 0,
                    };
                    ui.label(
                        RichText::new(format!("Time: {}", elapsed))
                            .size(15.0)
                            .color(Color32::WHITE)
                            .monospace(),
                    );
                });
            });

            ui.add_space(15.0);

            ui.vertical(|ui| {
                for _y in 0..rows {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(5.0, 5.0);

                        for _x in 0..cols {
                            if ui.add_sized([30.0, 30.0], egui::Button::new("")).clicked() {}
                        }
                    });
                }
            });
        });

        //repaint every second for timer
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([330.0, 380.0])
            .with_min_inner_size([330.0, 380.0])
            .with_max_inner_size([1060.0, 605.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "Minesweeper!",
        options,
        Box::new(|cc| Ok(Box::new(Minesweeper::new(cc)))),
    )
}
