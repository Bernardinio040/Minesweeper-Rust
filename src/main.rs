#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //hide console window on Windows in release
use crate::egui::ViewportCommand;
use eframe::egui;
use eframe::egui::Vec2;
use eframe::egui::{Color32, RichText};
use std::time::{Duration, Instant};
mod game_logic;
use game_logic::Game;

enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Minesweeper {
    wizard: Difficulty,
    game: Game,
    bombs: u32,
    time: Instant,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            wizard: Difficulty::Easy,
            game: Game::new(9, 9, 10),
            bombs: 10,
            time: Instant::now(),
        }
    }
}

impl Minesweeper {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn select_difficulty(&mut self, difficulty: Difficulty, ctx: &egui::Context) {
        match difficulty {
            Difficulty::Easy => {
                self.wizard = Difficulty::Easy;
                self.game = Game::new(9, 9, 10);
                self.bombs = 10;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(330.0, 380.0)));
            }
            Difficulty::Medium => {
                self.wizard = Difficulty::Medium;
                self.game = Game::new(16, 16, 40);
                self.bombs = 40;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(570.0, 605.0)));
            }
            Difficulty::Hard => {
                self.wizard = Difficulty::Hard;
                self.game = Game::new(30, 16, 99);
                self.bombs = 99;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(1060.0, 605.0)));
            }
        }

        self.time = Instant::now();
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        //topdown menu with difficulty selection and exit
        egui::TopBottomPanel::top("top_down_menu").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Game", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Easy").clicked() {
                        //9x9 minefield - 10 bombs
                        self.select_difficulty(Difficulty::Easy, ctx);
                    }
                    if ui.button("Medium").clicked() {
                        //16x16 minefield - 40 bombs
                        self.select_difficulty(Difficulty::Medium, ctx);
                    }
                    if ui.button("Hard").clicked() {
                        //30x16 minefield - 99 bombs
                        self.select_difficulty(Difficulty::Hard, ctx);
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = Vec2::new(5.0, 5.0);

            //minefield size gen
            let (rows, cols) = match self.wizard {
                Difficulty::Easy => (9, 9),
                Difficulty::Medium => (16, 16),
                Difficulty::Hard => (16, 30),
            };

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                //bomb count
                ui.label(
                    RichText::new(format!("Bombs: {}", self.bombs))
                        .size(15.0)
                        .color(Color32::WHITE)
                        .monospace(),
                );

                //timer
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let elapsed = self.time.elapsed().as_secs();
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
                for y in 0..rows {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(5.0, 5.0);

                        for x in 0..cols {
                            //let cell = &self.game.board[x][y];
                            let cell = egui::Button::new("");
                            let response = ui.add_sized([30.0, 30.0], cell);

                            if response.clicked() {
                                if !self.game.is_initialized {
                                    Game::generate_bombs(&mut self.game, x, y);
                                    self.game.is_initialized = true;
                                    Game::debug_print_board(&self.game);
                                } else if !self.game.is_game_over {
                                    Game::reveal_cell(&mut self.game, x, y);
                                    Game::debug_print_board(&self.game);
                                } // else {
                                //     Game::reveal_all_cells(&mut self.game);
                                // }
                            }

                            if response.secondary_clicked() {
                                // flag handler
                            }
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
