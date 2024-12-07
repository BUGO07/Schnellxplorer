use eframe::egui::{self, Context};

/// Menu bar
pub fn panel(ctx: &Context, current_path: &mut String, search: &mut String) {
    egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let path_label = ui.label("Current path:").id;
                ui.text_edit_singleline(current_path)
                    .labelled_by(path_label);

                // ui.add_space(5.0);

                let search_label = ui.label("Search:").id;
                ui.text_edit_singleline(search).labelled_by(search_label);
            });
            ui.add_space(1.0);
        });
    });
}
