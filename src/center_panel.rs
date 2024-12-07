use std::path::PathBuf;

use eframe::egui::{self, Context, Label, Sense, Vec2};

use crate::DirectoryItems;

/// Menu bar
pub fn panel(ctx: &Context, current_path: &mut String, search: &str) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let file_icon = egui::Image::new(egui::include_image!("../assets/file_icon.png"));
            let folder_icon = egui::Image::new(egui::include_image!("../assets/folder_icon.png"));
            ui.vertical(|ui| {
                if PathBuf::from(current_path.clone()).parent().is_some() {
                    ui.horizontal(|ui| {
                        ui.add(folder_icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)));
                        let name = ui.add(Label::new("..").wrap_mode(egui::TextWrapMode::Wrap));
                        if ui
                            .allocate_response(
                                ui.available_size() - Vec2::new(32.0, 0.0),
                                Sense::click(),
                            )
                            .clicked()
                            || name.clicked()
                        {
                            *current_path = PathBuf::from(current_path.clone())
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                        }
                    });
                }
                if let Ok(items) = crate::io::list_files_and_folders(current_path.clone()) {
                    let mut spawn = |path: &String, icon: egui::Image, size: f32, is_dir: bool| {
                        let created = ui
                            .horizontal(|ui| {
                                ui.add(icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)));
                                let binding = PathBuf::from(path.clone());
                                let name = if is_dir {
                                    binding.file_name().unwrap().to_str().unwrap().to_string()
                                } else {
                                    format!(
                                        "{} - {} MB",
                                        binding.file_name().unwrap().to_str().unwrap(),
                                        size
                                    )
                                };

                                let label =
                                    ui.add(Label::new(name).wrap_mode(egui::TextWrapMode::Wrap));
                                ui.allocate_response(ui.available_size(), Sense::click())
                                    .clicked()
                                    || ui
                                        .allocate_response(Vec2::new(32.0, 32.0), Sense::click())
                                        .clicked()
                                    || label.clicked()
                            })
                            .inner;

                        if created {
                            if is_dir {
                                *current_path = path.to_string();
                            } else {
                                crate::io::open_file_or_folder_in_os(path.clone());
                            }
                        }
                    };
                    let new_items = items
                        .iter()
                        .filter(|s| {
                            let binding = PathBuf::from(match s {
                                DirectoryItems::Folder(n) | DirectoryItems::File(n, _) => n,
                            });
                            let path = binding.file_name().unwrap().to_str().unwrap();
                            path.to_lowercase().contains(&search.to_lowercase())
                        })
                        .collect::<Vec<_>>();
                    for item in new_items {
                        match item {
                            DirectoryItems::File(path, size) => {
                                spawn(path, file_icon.clone(), *size, false);
                            }
                            DirectoryItems::Folder(path) => {
                                spawn(path, folder_icon.clone(), 0.0, true);
                            }
                        }
                    }
                } else {
                    ui.label("Error reading directory");
                }
            })
        });
    });
}
