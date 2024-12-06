//! This is my attempt at making a blazingly fast file explorer, using bevy and rust.
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

/// the app entrypoint
pub mod entrypoint;
/// main ui
pub mod ui;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_systems(
            Startup,
            (entrypoint::setup_ui, ui::directory::content::display_items)
                // .after(DirectoryUpdatedSet)
                .chain(),
        )
        .add_systems(
            Update,
            ui::directory::content::display_items
                .run_if(resource_changed::<ui::directory::content::CurrentData>),
        )
        .run();
}
