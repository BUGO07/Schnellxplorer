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
    last_items: &mut [DirectoryItems],
    refresh_directory: &mut bool,
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
                            *last_path = current_path.clone();
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

                        let btn = ui.add(
                            Button::image_and_text(
                                icon.clone().fit_to_exact_size(Vec2::new(32.0, 32.0)),
                                RichText::new(name),
                            )
                            .min_size(Vec2::new(ui.available_width() - 10.0, 32.0)),
                        );
                        btn.context_menu(|ui| {
                            if ui.add(Button::new("Open")).clicked() {
                                if is_dir {
                                    // *last_path = current_path.clone();
                                    // *current_path = path.to_string();
                                    // *current_written_path = path.to_string();
                                } else {
                                    crate::io::open_file_or_folder_in_os(path.to_string());
                                }
                                ui.close_menu();
                            }
                            if ui.add(Button::new("Copy")).clicked() {
                                if is_dir {
                                    // *last_path = current_path.clone();
                                    // *current_path = path.to_string();
                                    // *current_written_path = path.to_string();
                                } else {
                                    crate::io::copy_file_to_clipboard(path.to_string());
                                }
                                ui.close_menu();
                            }
                            if ui.add(Button::new("Copy Location")).clicked() {
                                crate::io::copy_text_to_clipboard(path.to_string());
                                ui.close_menu();
                            }
                            if ui.add(Button::new("Delete")).clicked() {
                                crate::io::delete_path(path.to_string());
                                *refresh_directory = true;
                                ui.close_menu();
                            }
                        });
                        btn.clicked()
                    })
                    .inner
                };

                if current_path != last_path {
                    *refresh_directory = true;
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
