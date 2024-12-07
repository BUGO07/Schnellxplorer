//! This is my attempt at making a blazingly fast file explorer, using rust.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: add more stuff and split them into modules

use std::path::PathBuf;

use eframe::egui::{self, Label, Sense, Vec2};

/// This module handles stuff that has to do with the OS.
pub mod io;

/// Enum housing directory items
#[derive(Clone, Debug)]
pub enum DirectoryItems {
    /// File with its path
    File(String),
    /// Folder with its path
    Folder(String),
}

fn main() -> eframe::Result {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let home_path = std::env::var_os("HOME")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut current_path = if args.is_empty() {
        println!("No arguments, start at ~");
        home_path
    } else {
        println!("{:?}", args);
        if PathBuf::from(&args[0]).exists() {
            args[0].clone()
        } else {
            home_path
        }
    };

    eframe::run_simple_native(
        "Schnellxplorer",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        move |ctx, _frame| {
            ctx.send_viewport_cmd(egui::ViewportCommand::Title(format!(
                "Schnellxplorer - {}",
                current_path
            )));
            egui_extras::install_image_loaders(ctx);
            egui::CentralPanel::default().show(ctx, |ui| {
                let file_icon = egui::Image::new(egui::include_image!("../assets/file_icon.png"));
                let folder_icon =
                    egui::Image::new(egui::include_image!("../assets/folder_icon.png"));
                let path_label = ui.label("Current path:").id;
                ui.text_edit_singleline(&mut current_path)
                    .labelled_by(path_label);
                ui.vertical(|ui| {
                    if PathBuf::from(&current_path).parent().is_some() {
                        ui.horizontal(|ui| {
                            ui.add(folder_icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)));
                            let name = ui.add(Label::new("..").wrap_mode(egui::TextWrapMode::Wrap));
                            if ui
                                .allocate_response(ui.available_size(), Sense::click())
                                .clicked()
                                || ui
                                    .allocate_response(Vec2::new(32.0, 32.0), Sense::click())
                                    .clicked()
                                || name.clicked()
                            {
                                current_path = PathBuf::from(current_path.clone())
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                            }
                        });
                    }
                    if let Ok(items) = io::list_files_and_folders(current_path.clone()) {
                        for item in items {
                            match item {
                                DirectoryItems::File(path) => {
                                    ui.horizontal(|ui| {
                                        ui.add(
                                            file_icon
                                                .clone()
                                                .fit_to_exact_size(Vec2::new(32.0, 32.0)),
                                        );
                                        let name = ui.add(
                                            Label::new(
                                                PathBuf::from(path.clone())
                                                    .file_name()
                                                    .unwrap()
                                                    .to_str()
                                                    .unwrap(),
                                            )
                                            .wrap_mode(egui::TextWrapMode::Wrap),
                                        );
                                        if ui
                                            .allocate_response(ui.available_size(), Sense::click())
                                            .clicked()
                                            || ui
                                                .allocate_response(
                                                    Vec2::new(32.0, 32.0),
                                                    Sense::click(),
                                                )
                                                .clicked()
                                            || name.clicked()
                                        {
                                            io::open_file_or_folder_in_os(path.clone());
                                        }
                                    });
                                }
                                DirectoryItems::Folder(path) => {
                                    if ui
                                        .horizontal(|ui| {
                                            ui.add(
                                                folder_icon
                                                    .clone()
                                                    .fit_to_exact_size(Vec2::new(32.0, 32.0)),
                                            );
                                            let name = ui.add(
                                                Label::new(
                                                    PathBuf::from(path.clone())
                                                        .file_name()
                                                        .unwrap()
                                                        .to_str()
                                                        .unwrap(),
                                                )
                                                .wrap_mode(egui::TextWrapMode::Wrap),
                                            );
                                            ui.allocate_response(
                                                ui.available_size(),
                                                Sense::click(),
                                            )
                                            .clicked()
                                                || ui
                                                    .allocate_response(
                                                        Vec2::new(32.0, 32.0),
                                                        Sense::click(),
                                                    )
                                                    .clicked()
                                                || name.clicked()
                                        })
                                        .inner
                                    {
                                        current_path = path;
                                    }
                                }
                            }
                        }
                    } else {
                        ui.label("Error reading directory");
                    }
                })
            });
        },
    )
}
