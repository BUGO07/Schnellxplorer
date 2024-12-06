use std::path::PathBuf;

use bevy::prelude::*;

/// Root node component. This is what everything will be mounted on.
#[derive(Component)]
pub struct RootNode;

/// The ui setup system.
pub fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            grid_column: GridPlacement::start(1),
            grid_row: GridPlacement::start(1),
            position_type: PositionType::Absolute,
            min_width: Val::Percent(100.0),
            min_height: Val::Percent(100.0),

            // display: Display::Flex,
            // justify_content: JustifyContent::Center,
            // align_items: AlignItems::Center,
            ..default()
        },
        RootNode,
    ));

    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    let home_path = std::env::var_os("HOME")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut current_dir = if args.is_empty() {
        info!("No arguments, start at ~");
        PathBuf::from(&home_path)
    } else {
        info!("{:?}", args);
        PathBuf::from(&args[0])
    };
    if !current_dir.exists() {
        current_dir = PathBuf::from(&home_path);
    }

    commands.insert_resource(crate::ui::directory::content::CurrentData {
        path: current_dir.to_str().unwrap().to_string(),
    });
}
