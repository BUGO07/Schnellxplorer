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
                    let mut spawn = |path: &String, name: &String, icon: egui::Image, size: f32| {
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
                    spawn(
                        &crate::io::get_home_dir(),
                        &"Home".to_string(),
                        folder_icon.clone(),
                        32.0,
                    );

                    for item in
                        crate::io::list_files_and_folders(crate::io::get_home_dir()).unwrap()
                    {
                        if let crate::DirectoryItems::Folder(path) = item {
                            let name = PathBuf::from(path.clone())
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string();

                            if name.starts_with(".") || name.chars().next().unwrap().is_lowercase()
                            {
                                continue;
                            }

                            spawn(&path, &name, folder_icon.clone(), 32.0);
                        }
                    }
                });
            });
        });
}
