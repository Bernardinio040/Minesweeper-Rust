#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //hide console window on Windows in release
use crate::egui::ViewportCommand;
use eframe::egui;
use eframe::egui::Vec2;
use eframe::egui::{Color32, RichText};
use egui::TextureHandle;
use std::time::{Duration, Instant};
mod game_logic;
use game_logic::Game;

const EASY_ROWS: usize = 9;
const EASY_COLS: usize = 9;
const EASY_MINES: u32 = 10;
const EASY_WINDOW: [f32; 2] = [285.0, 350.0];

const MEDIUM_ROWS: usize = 16;
const MEDIUM_COLS: usize = 16;
const MEDIUM_MINES: u32 = 40;
const MEDIUM_WINDOW: [f32; 2] = [495.0, 560.0];

const HARD_ROWS: usize = 16;
const HARD_COLS: usize = 30;
const HARD_MINES: u32 = 99;
const HARD_WINDOW: [f32; 2] = [915.0, 560.0];

struct GameAssets {
    hidden: TextureHandle,
    revealed: TextureHandle,
    flag: TextureHandle,
    mine: TextureHandle,
    numbers: Vec<TextureHandle>,
}

impl GameAssets {
    fn new(ctx: &egui::Context) -> Self {
        let load = |name: &str, bytes: &[u8]| -> TextureHandle {
            let image = image::load_from_memory(bytes).expect("Failed to load image");
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            ctx.load_texture(name, color_image, Default::default())
        };

        Self {
            hidden: load("hidden", include_bytes!("../assets/hidden_cell.png")),
            revealed: load("revealed", include_bytes!("../assets/revealed_cell.png")),
            flag: load("flag", include_bytes!("../assets/flag.png")),
            mine: load("mine", include_bytes!("../assets/mine.png")),
            numbers: vec![
                load("1", include_bytes!("../assets/1.png")),
                load("2", include_bytes!("../assets/2.png")),
                load("3", include_bytes!("../assets/3.png")),
                load("4", include_bytes!("../assets/4.png")),
                load("5", include_bytes!("../assets/5.png")),
                load("6", include_bytes!("../assets/6.png")),
                load("7", include_bytes!("../assets/7.png")),
                load("8", include_bytes!("../assets/8.png")),
            ],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub struct Minesweeper {
    assets: Option<GameAssets>,
    wizard: Difficulty,
    game: Game,
    mines: u32,
    time: Instant,
    final_time: Option<u64>,
}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {
            assets: None,
            wizard: Difficulty::Easy,
            game: Game::new(EASY_COLS, EASY_ROWS, EASY_MINES),
            mines: EASY_MINES,
            time: Instant::now(),
            final_time: None,
        }
    }
}

impl Minesweeper {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();
        app.assets = Some(GameAssets::new(&_cc.egui_ctx));
        app
    }

    pub fn select_difficulty(&mut self, difficulty: Difficulty, ctx: &egui::Context) {
        match difficulty {
            Difficulty::Easy => {
                self.wizard = Difficulty::Easy;
                self.game = Game::new(EASY_COLS, EASY_ROWS, EASY_MINES);
                self.mines = EASY_MINES;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                    EASY_WINDOW[0],
                    EASY_WINDOW[1],
                )));
            }
            Difficulty::Medium => {
                self.wizard = Difficulty::Medium;
                self.game = Game::new(MEDIUM_COLS, MEDIUM_ROWS, MEDIUM_MINES);
                self.mines = MEDIUM_MINES;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                    MEDIUM_WINDOW[0],
                    MEDIUM_WINDOW[1],
                )));
            }
            Difficulty::Hard => {
                self.wizard = Difficulty::Hard;
                self.game = Game::new(HARD_COLS, HARD_ROWS, HARD_MINES);
                self.mines = HARD_MINES;
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                    HARD_WINDOW[0],
                    HARD_WINDOW[1],
                )));
            }
        }

        self.time = Instant::now();
        self.final_time = None;
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
                        //9x9 minefield - 10 mines
                        self.select_difficulty(Difficulty::Easy, ctx);
                    }
                    if ui.button("Medium").clicked() {
                        //16x16 minefield - 40 mines
                        self.select_difficulty(Difficulty::Medium, ctx);
                    }
                    if ui.button("Hard").clicked() {
                        //30x16 minefield - 99 mines
                        self.select_difficulty(Difficulty::Hard, ctx);
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //minefield size gen
            let (rows, cols) = match self.wizard {
                Difficulty::Easy => (EASY_ROWS, EASY_COLS),
                Difficulty::Medium => (MEDIUM_ROWS, MEDIUM_COLS),
                Difficulty::Hard => (HARD_ROWS, HARD_COLS),
            };

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                let flags_count = self
                    .game
                    .board
                    .iter()
                    .flatten()
                    .filter(|c| c.is_flagged)
                    .count();
                let mines_left = self.mines as i32 - flags_count as i32;

                //mines count
                ui.label(
                    RichText::new(format!("Mines: {}", mines_left))
                        .size(15.0)
                        .color(if mines_left < 0 {
                            Color32::RED
                        } else {
                            Color32::WHITE
                        })
                        .monospace(),
                );

                //timer
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let elapsed = if let Some(final_t) = self.final_time {
                        final_t
                    } else {
                        self.time.elapsed().as_secs()
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

            ui.vertical_centered(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.spacing_mut().button_padding = Vec2::ZERO;
                for y in 0..rows {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        for x in 0..cols {
                            let cell = &self.game.board[x][y];
                            let assets = self.assets.as_ref().unwrap();

                            let texture = if !cell.is_revealed {
                                if cell.is_flagged {
                                    &assets.flag
                                } else {
                                    &assets.hidden
                                }
                            } else {
                                if cell.is_mine {
                                    &assets.mine
                                } else if cell.neighbour_mines == 0 {
                                    &assets.revealed
                                } else {
                                    //neighbour_mines to 1..8, a tablica ma indeksy 0..7
                                    &assets.numbers[(cell.neighbour_mines - 1) as usize]
                                }
                            };

                            let image_btn = egui::Button::image(texture).frame(false);
                            let response = ui.add(image_btn);

                            //left click handler
                            if response.clicked() {
                                if !cell.is_flagged {
                                    if !self.game.is_initialized {
                                        Game::generate_mines(&mut self.game, x, y);
                                        self.game.is_initialized = true;
                                    } else if !self.game.is_game_over {
                                        if cell.is_revealed {
                                            //when we click on revealed cell -> chord
                                            self.game.chord_cell(x, y);
                                        } else {
                                            //when we click on unrevealed cell -> reveal
                                            Game::reveal_cell(&mut self.game, x, y);
                                        }
                                    }
                                }
                            }

                            //right click handler
                            if response.secondary_clicked() {
                                self.game.toggle_flag(x, y);
                            }
                        }
                        ui.end_row();
                    });
                }
            });
        });

        if self.game.is_game_over {
            if self.final_time.is_none() {
                self.final_time = Some(self.time.elapsed().as_secs());
            }

            egui::Window::new("Game Over")
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .max_width(280.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);

                        let (text, color) = if self.game.is_win {
                            ("WYGRANA!", Color32::GREEN)
                        } else {
                            ("GAME OVER", Color32::RED)
                        };

                        ui.label(egui::RichText::new(text).size(30.0).color(color).strong());

                        //your time label after winning
                        if self.game.is_win {
                            let elapsed = if let Some(final_t) = self.final_time {
                                final_t
                            } else {
                                self.time.elapsed().as_secs()
                            };
                            ui.label(format!("Your time: {} s", elapsed));
                        }

                        ui.add_space(10.0);

                        //restart handler
                        if ui.button("Spróbuj ponownie").clicked() {
                            self.select_difficulty(self.wizard, ctx);
                        }

                        ui.add_space(10.0);

                        if ui.button("Wyjdź").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }

                        ui.add_space(10.0);
                    });
                });
        }

        //repaint every second for timer
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([EASY_WINDOW[0], EASY_WINDOW[1]])
            .with_min_inner_size([EASY_WINDOW[0], EASY_WINDOW[1]])
            .with_max_inner_size([HARD_WINDOW[0], HARD_WINDOW[1]])
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
