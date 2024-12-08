use std::path::PathBuf;

use eframe::egui::{self, Context};

/// Top bar
pub fn draw(
    ctx: &Context,
    current_path: &mut String,
    current_written_path: &mut String,
    search: &mut String,
) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let path_label = ui.label("Current path:").id;
                let path_exists = PathBuf::from(current_written_path.clone()).exists();

                if current_written_path != current_path && path_exists {
                    *current_path = current_written_path.clone();
                }
                if current_written_path.is_empty() {
                    *current_path = "/".to_string();
                }

                let text_edit = ui
                    .text_edit_singleline(current_written_path)
                    .labelled_by(path_label);

                if text_edit.lost_focus()
                    || (text_edit.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
                {
                    if !path_exists {
                        *current_written_path = current_path.clone();
                    }

                    *current_written_path =
                        crate::utils::normalize_path(&PathBuf::from(current_written_path.clone()))
                            .to_str()
                            .unwrap()
                            .to_string();
                }

                let search_label = ui.label("Search:").id;
                ui.text_edit_singleline(search).labelled_by(search_label);
            });
            ui.add_space(1.0);
        });
    });
}
