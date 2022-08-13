#![cfg_attr(not(debug_assertions), windows_subsystem = "windows&")] // hide console window on Windows in release

use eframe::egui;
use egui::{text::LayoutJob, Vec2, *};

mod file_system;
mod hg_commands;

const ICON: &str = ".\\icon.png";

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2 {
            x: 400f32,
            y: 600f32,
        }),
        icon_data: Some(load_icon(ICON)), // an example
        ..Default::default()
    };
    eframe::run_native(
        "Rusty hg Manager",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<String>,
    picked_branch: Option<String>,
    repo_list: Option<Vec<String>>,

    n_items: usize,
    console_output: LayoutJob,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Select Project…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                    self.repo_list =
                        file_system::find_repo_list(path.display().to_string()).unwrap();

                    match &self.repo_list {
                        Some(repo_list) => {
                            self.picked_branch = Some(hg_commands::hg_branch(&repo_list[0]));
                        }
                        None => (),
                    };
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Current Project Folder:");
                    ui.monospace(picked_path);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("No Project Selected");
                });
            }

            if let Some(picked_branch) = &self.picked_branch {
                ui.horizontal(|ui| {
                    ui.label("Current Branch:");
                    ui.monospace(picked_branch);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("No Project Selected");
                });
            }

            ui.separator();

            ui.collapsing("Commands", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Status").clicked() {
                        hg_commands::hg_status(&self.repo_list, &mut self.console_output)
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Pull").clicked() {}
                    if ui.button("Push").clicked() {}
                });
                ui.horizontal(|ui| {
                    if ui.button("Switch").clicked() {}
                    if ui.button("Purge").clicked() {}
                });
            });

            ui.add_space(4.0);

            ui.label("Commands Output:");

            ui.separator();

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            ScrollArea::vertical().stick_to_bottom().show_rows(
                ui,
                row_height,
                self.n_items,
                |ui, _row_range| {
                    ui.label(self.console_output.clone());
                },
            );
        });
    }
}

fn load_icon(path: &str) -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
