use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

/// Enum housing directory items
#[derive(Debug)]
pub enum DirectoryItems {
    /// File with its path
    File(String),
    /// Folder with its path
    Folder(String),
}

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
) {
    for root in root_query.iter_mut() {
        // commands.entity(root).with_children(|cb| {
        let items = super::io::list_files_and_folders(&current_data.path).unwrap();
        for item in items {
            match item {
                DirectoryItems::File(path) => {
                    let base_node = spawn_base_node(&mut commands).id();

                    // Icon
                    commands
                        .spawn((
                            ImageNode::new(asset_server.load("file_icon.png")),
                            Node {
                                height: Val::Px(50.0),
                                ..default()
                            },
                        ))
                        .set_parent(base_node);
                    // Folder Name
                    commands
                        .spawn((
                            Text::new(
                                std::path::Path::new(&path)
                                    .file_name()
                                    .unwrap()
                                    .to_str()
                                    .unwrap(),
                            ),
                            TextFont {
                                font_size: 10.0,
                                ..default()
                            },
                        ))
                        .set_parent(base_node);

                    commands.entity(base_node).set_parent(root);
                }
                DirectoryItems::Folder(path) => {
                    let base_node = spawn_base_node(&mut commands).id();

                    // Icon
                    commands
                        .spawn((
                            ImageNode::new(asset_server.load("folder_icon.png")),
                            Node {
                                height: Val::Px(50.0),
                                ..default()
                            },
                        ))
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
                        ))
                        .set_parent(base_node);

                    commands.entity(base_node).set_parent(root);
                }
            }
        }
        // });
    }
}

/// "borrowed" from `bevy_editor`
fn spawn_base_node<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    let mut base_node_ec = commands.spawn((
        Button,
        Node {
            margin: UiRect::all(Val::Px(5.0)),
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
