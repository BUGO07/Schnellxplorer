use std::path::PathBuf;

use eframe::egui::{self, Button, Color32, Context, RichText, Vec2};

use crate::DirectoryItems;

/// Menu bar
pub fn draw(
    ctx: &Context,
    current_path: &mut String,
    current_written_path: &mut String,
    search: &str,
    last_path: &mut String,
    last_items: &mut Vec<DirectoryItems>,
) {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_space(Vec2::X * ui.available_width());

            let file_icon = egui::Image::new(egui::include_image!("../assets/file_icon.png"));
            let folder_icon = egui::Image::new(egui::include_image!("../assets/folder_icon.png"));

            ui.vertical(|ui| {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

                if PathBuf::from(current_path.clone()).parent().is_some() {
                    ui.horizontal(|ui| {
                        if ui
                            .add(
                                Button::image_and_text(
                                    folder_icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)),
                                    RichText::new(".."),
                                )
                                .min_size(Vec2::new(ui.available_width() - 10.0, 32.0)),
                            )
                            .clicked()
                        {
                            *current_path = PathBuf::from(current_path.clone())
                                .parent()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();
                            *current_written_path = current_path.clone();
                        }
                    });
                }
                let mut spawn_button = |path: &str, icon: egui::Image, size: f32, is_dir: bool| {
                    ui.horizontal(|ui| {
                        let binding = PathBuf::from(path);
                        let name = if is_dir {
                            binding.file_name().unwrap().to_str().unwrap().to_string()
                        } else {
                            let (size, unit) = crate::utils::size_units(size);
                            format!(
                                "{} - {} {}",
                                binding.file_name().unwrap().to_str().unwrap(),
                                size,
                                unit
                            )
                        };

                        ui.add(
                            Button::image_and_text(
                                icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)),
                                RichText::new(name),
                            )
                            .min_size(Vec2::new(ui.available_width() - 10.0, 32.0)),
                        )
                        .clicked()
                    })
                    .inner
                };

                if current_path != last_path {
                    *last_items = if let Ok(items) =
                        crate::io::list_files_and_folders(current_path.clone())
                    {
                        for item in items
                            .iter()
                            .filter(|item| {
                                let path = match item {
                                    DirectoryItems::Folder(p) | DirectoryItems::File(p, _) => p,
                                };
                                PathBuf::from(path)
                                    .file_name()
                                    .and_then(|f| f.to_str())
                                    .map_or(false, |name| {
                                        name.to_lowercase().contains(&search.to_lowercase())
                                    })
                            })
                            .collect::<Vec<_>>()
                        {
                            match item {
                                DirectoryItems::File(path, size) => {
                                    if spawn_button(path, file_icon.clone(), *size, false) {
                                        crate::io::open_file_or_folder_in_os(path.clone());
                                    }
                                }
                                DirectoryItems::Folder(path) => {
                                    if spawn_button(path, folder_icon.clone(), 0.0, true) {
                                        *last_path = current_path.clone();
                                        *current_path = path.clone();
                                        *current_written_path = path.clone();
                                    }
                                }
                            }
                        }
                        items
                    } else {
                        ui.label("Error reading directory");
                        Vec::new()
                    };
                } else {
                    for item in last_items
                        .iter()
                        .filter(|item| {
                            let path = match item {
                                DirectoryItems::Folder(p) | DirectoryItems::File(p, _) => p,
                            };
                            PathBuf::from(path)
                                .file_name()
                                .and_then(|f| f.to_str())
                                .map_or(false, |name| {
                                    name.to_lowercase().contains(&search.to_lowercase())
                                })
                        })
                        .collect::<Vec<_>>()
                    {
                        match item {
                            DirectoryItems::File(path, size) => {
                                if spawn_button(path, file_icon.clone(), *size, false) {
                                    crate::io::open_file_or_folder_in_os(path.clone());
                                }
                            }
                            DirectoryItems::Folder(path) => {
                                if spawn_button(path, folder_icon.clone(), 0.0, true) {
                                    *last_path = current_path.clone();
                                    *current_path = path.clone();
                                    *current_written_path = path.clone();
                                }
                            }
                        }
                    }
                }
            });
        });
    });
}
