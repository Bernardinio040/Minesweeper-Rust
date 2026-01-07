#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use crate::egui::ViewportCommand;
use eframe::egui;
use eframe::egui::Vec2;

pub struct Minesweeper {}

impl Default for Minesweeper {
    fn default() -> Self {
        Self {}
    }
}

impl Minesweeper {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Minesweeper {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("top_down_menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Game", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Easy").clicked() {
                        //9x9 minefield
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            330.0, 400.0,
                        )));
                    }
                    if ui.button("Medium").clicked() {
                        //16x16 minefield
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            550.0, 500.0,
                        )))
                    }
                    if ui.button("Hard").clicked() {
                        //30x16 minefield
                        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(Vec2::new(
                            900.0, 500.0,
                        )))
                    }
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                    //https://github.com/Seebass22/harptabber/blob/main/harptabber-gui/src/app.rs
                });
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.button_padding = Vec2::new(5.0, 5.0);

            ui.vertical(|ui| {
                for _y in 0..9 {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(5.0, 5.0);

                        for _x in 0..9 {
                            if ui.add_sized([30.0, 30.0], egui::Button::new("")).clicked() {}
                        }
                    });
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([330.0, 400.0])
            .with_min_inner_size([330.0, 400.0])
            .with_max_inner_size([900.0, 500.0])
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
