//! This is my attempt at making a blazingly fast file explorer, using rust.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// TODO: add more stuff and split them into modules

use std::path::PathBuf;

use io::get_home_dir;

/// This module handles stuff that has to do with the OS.
pub mod io;

/// This module handles the top bar.
pub mod menu_bar;

/// This module handles the top bar.
pub mod top_bar;

/// This module handles the main area `CenterPanel`
pub mod center_panel;

/// Enum housing directory items
#[derive(Clone, Debug)]
pub enum DirectoryItems {
    /// File with its path
    File(String, f32),
    /// Folder with its path
    Folder(String),
}

fn main() -> eframe::Result {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let home_path = get_home_dir();

    let mut current_path = if args.is_empty() {
        println!("No arguments, start at ~");
        home_path
    } else {
        println!("{:?}", args);
        if PathBuf::from(&args[0]).exists() {
            args[0].clone()
        } else {
            home_path
        }
    };
    let mut search = "".to_string();

    eframe::run_simple_native(
        "Schnellxplorer",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
            ..Default::default()
        },
        move |ctx, _frame| {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Title(format!(
                "Schnellxplorer - {}",
                current_path
            )));
            egui_extras::install_image_loaders(ctx);
            menu_bar::panel(ctx);
            top_bar::panel(ctx, &mut current_path, &mut search);
            center_panel::panel(ctx, &mut current_path, &search);
        },
    )
}
