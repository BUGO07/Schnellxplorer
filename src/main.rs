//! This is my attempt at making a blazingly fast file explorer, using rust.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use eframe::egui::Vec2;
use io::get_home_dir;

/// This module handles stuff that has to do with the OS.
pub mod io;

/// This module handles utility functions.
pub mod utils;

/// This module handles the menu bar.
pub mod menu_bar;

/// This module handles the top bar.
pub mod top_bar;

/// This module handles the side bar.
pub mod side_bar;

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
        home_path.clone()
    } else {
        println!("{:?}", args);
        if PathBuf::from(&args[0]).exists() {
            args[0].clone()
        } else {
            home_path.clone()
        }
    };
    let mut current_written_path = current_path.clone();
    let mut last_path = "".to_string();
    let mut last_items: Vec<DirectoryItems> = Vec::new();
    let home_items = io::list_files_and_folders(home_path).unwrap();
    let mut search = "".to_string();
    let mut eq = false;

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
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::MinInnerSize(Vec2::new(
                740.0, 460.0,
            )));
            egui_extras::install_image_loaders(ctx);
            menu_bar::draw(ctx);
            top_bar::draw(
                ctx,
                &mut current_path,
                &mut current_written_path,
                &mut search,
            );
            side_bar::draw(
                ctx,
                &mut current_path,
                &mut current_written_path,
                &home_items,
            );
            center_panel::draw(
                ctx,
                &mut current_path,
                &mut current_written_path,
                &search,
                &mut last_path,
                &mut last_items,
            );
            if eq {
                last_path = current_path.clone();
                eq = false;
            }
            if last_path != current_path {
                eq = true;
            }
        },
    )
}
