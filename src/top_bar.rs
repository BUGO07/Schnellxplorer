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
                let exists = PathBuf::from(current_written_path.clone()).exists();
                if current_written_path != current_path && exists {
                    *current_path = current_written_path.clone();
                }
                ui.text_edit_singleline(current_written_path)
                    .labelled_by(path_label);

                let search_label = ui.label("Search:").id;
                ui.text_edit_singleline(search).labelled_by(search_label);
            });
            ui.add_space(1.0);
        });
    });
}
