use eframe::egui::{self, Color32, Context};

/// Menu bar
pub fn draw(ctx: &Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        crate::io::exit();
                    }
                });

                // TODO: add uses for these
                ui.menu_button("Edit", |ui| {});
                ui.menu_button("Window", |ui| {});
                ui.menu_button("Help", |ui| {});
            });
            ui.add_space(1.0);
        });
    });
}
