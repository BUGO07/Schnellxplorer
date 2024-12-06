use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

/// Enum housing directory items
#[derive(Clone, Debug)]
pub enum DirectoryItems {
    /// File with its path
    File(String),
    /// Folder with its path
    Folder(String),
}

/// Marks the directory items to despawn them quickly.
#[derive(Component)]
pub struct ItemMarker;

/// Current directory data
#[derive(Resource)]
pub struct CurrentData {
    /// Curent directory path
    pub path: String,
}

/// This system spawns icons and names of the items in the current directory.
pub fn display_items(
    mut commands: Commands,
    mut root_query: Query<Entity, With<crate::entrypoint::RootNode>>,
    current_data: Res<CurrentData>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<ItemMarker>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for root in root_query.iter_mut() {
        commands.spawn((
            Text::new(format!("Current directory: {}", &current_data.path)),
            ItemMarker,
        ));
        // commands.entity(root).with_children(|cb| {

        if let Some(path) = std::path::Path::new(&current_data.path).parent() {
            let path_string = path.to_str().unwrap().to_string();

            let base_node = spawn_base_node(&mut commands).id();
            let closure_path = path_string.clone();

            // Icon
            commands
                .spawn((
                    ImageNode::new(asset_server.load("folder_icon.png")),
                    Node {
                        height: Val::Px(50.0),
                        ..default()
                    },
                    ItemMarker,
                ))
                .observe(
                    move |trigger: Trigger<Pointer<Up>>, mut current_data: ResMut<CurrentData>| {
                        if trigger.event().button == PointerButton::Primary {
                            current_data.path = closure_path.clone();
                        }
                    },
                )
                .set_parent(base_node);
            // Folder Name
            commands
                .spawn((
                    Text::new(".."),
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    ItemMarker,
                ))
                .set_parent(base_node);

            commands.entity(base_node).set_parent(root);
        }

        if let Ok(items) = super::io::list_files_and_folders(&current_data.path) {
            for item in items {
                match item {
                    DirectoryItems::File(path) => {
                        let base_node = spawn_base_node(&mut commands).id();
                        let closure_path = path.clone();
                        // Icon
                        commands
                            .spawn((
                                ImageNode::new(asset_server.load("file_icon.png")),
                                Node {
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                ItemMarker,
                            ))
                            .observe(
                                move |trigger: Trigger<Pointer<Up>>, mut commands: Commands| {
                                    if trigger.event().button == PointerButton::Primary {
                                        commands.run_system_cached_with(
                                            super::io::open_file_or_folder_in_os,
                                            closure_path.clone(),
                                        );
                                    }
                                },
                            )
                            .set_parent(base_node);
                        // Folder Name
                        let mut text = std::path::Path::new(&path)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        text.truncate(13);
                        text.push_str("...");
                        commands
                            .spawn((
                                Text::new(
                                    text
                                ),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                ItemMarker,
                            ))
                            .set_parent(base_node);

                        commands.entity(base_node).set_parent(root);
                    }
                    DirectoryItems::Folder(path) => {
                        let base_node = spawn_base_node(&mut commands).id();

                        let closure_path = path.clone();

                        // Icon
                        commands
                        .spawn((
                            ImageNode::new(asset_server.load("folder_icon.png")),
                            Node {
                                height: Val::Px(50.0),
                                ..default()
                            },
                            ItemMarker
                        ))
                        .observe(
                            move |trigger: Trigger<Pointer<Up>>,
                                  mut current_data: ResMut<CurrentData>| {
                                if trigger.event().button == PointerButton::Primary {
                                    current_data.path = closure_path.clone();
                                }
                            },
                        )
                        .set_parent(base_node);
                        // Folder Name
                        commands
                            .spawn((
                                Text::new(
                                    std::path::Path::new(&path)
                                        .file_stem()
                                        .unwrap()
                                        .to_str()
                                        .unwrap(),
                                ),
                                TextFont {
                                    font_size: 10.0,
                                    ..default()
                                },
                                ItemMarker,
                            ))
                            .set_parent(base_node);

                        commands.entity(base_node).set_parent(root);
                    }
                }
            }
        } else {
            //TODO: improve this
            let base_node = spawn_base_node(&mut commands).set_parent(root).id();
            commands
                .spawn((
                    Text::new("Error reading directory"),
                    Node {
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ItemMarker,
                ))
                .set_parent(base_node);
        }
        // });
    }
}

/// "borrowed" from `bevy_editor`
fn spawn_base_node<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    let mut base_node_ec = commands.spawn((
        Button,
        Node {
            margin: UiRect {
                left: Val::Px(3.0),
                right: Val::Px(3.0),
                top: Val::Px(20.0),
                bottom: Val::Px(3.0),
            },
            padding: UiRect::all(Val::Px(5.0)),
            height: Val::Px(100.0),
            width: Val::Px(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(3.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ZIndex(1),
        BorderRadius::all(Val::Px(8.)),
        ItemMarker,
    ));

    // Hover effect
    base_node_ec
        .observe(
            move |_trigger: Trigger<Pointer<Move>>,
                  window_query: Query<Entity, With<Window>>,
                  mut commands: Commands| {
                let window = window_query.single();
                commands
                    .entity(window)
                    .insert(CursorIcon::System(SystemCursorIcon::Pointer));
            },
        )
        .observe(
            move |_trigger: Trigger<Pointer<Out>>,
                  window_query: Query<Entity, With<Window>>,
                  mut commands: Commands| {
                let window = window_query.single();
                commands
                    .entity(window)
                    .insert(CursorIcon::System(SystemCursorIcon::Default));
            },
        );

    base_node_ec
}
