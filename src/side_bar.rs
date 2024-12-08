use std::path::PathBuf;

use eframe::egui::{self, Button, Color32, Context, RichText, Vec2};

/// Side bar
pub fn draw(ctx: &Context, current_path: &mut String, current_written_path: &mut String) {
    egui::SidePanel::new(egui::panel::Side::Left, "left_bar")
        .resizable(false)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                    ui.add_space(8.0);
                    ui.label("Places");
                    ui.add_space(4.0);

                    let mut spawn_button =
                        |path: &str, name: &str, icon: egui::Image, size: f32| {
                            let clicked = ui
                                .horizontal(|ui| {
                                    ui.add(
                                        Button::image_and_text(
                                            icon.clone().fit_to_exact_size(Vec2::new(size, size)),
                                            RichText::new(name),
                                        )
                                        .min_size(Vec2::new(size * 5.0, size)),
                                    )
                                    .clicked()
                                })
                                .inner;

                            if clicked {
                                *current_path = path.to_string();
                                *current_written_path = current_path.clone();
                            }
                        };

                    let folder_icon =
                        egui::Image::new(egui::include_image!("../assets/folder_icon.png"));

                    spawn_button(
                        &crate::io::get_home_dir(),
                        "Home",
                        folder_icon.clone(),
                        32.0,
                    );

                    if let Ok(items) = crate::io::list_files_and_folders(crate::io::get_home_dir())
                    {
                        for item in items {
                            if let crate::DirectoryItems::Folder(path) = item {
                                let name = PathBuf::from(&path)
                                    .file_name()
                                    .and_then(|f| f.to_str())
                                    .unwrap_or_default()
                                    .to_string();

                                if name.starts_with('.')
                                    || name.chars().next().unwrap().is_lowercase()
                                {
                                    continue;
                                }

                                spawn_button(&path, &name, folder_icon.clone(), 32.0);
                            }
                        }
                    }
                });
            });
        });
}
