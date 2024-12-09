use crate::DirectoryItems;

/// This function lists files and folders from a given path.
pub fn list_files_and_folders(path: String) -> Result<Vec<DirectoryItems>, std::io::Error> {
    let entries = std::fs::read_dir(path.clone())?;
    let mut items = Vec::new();

    println!("LISTING FILES AND FOLDERS at {}", path.clone());

    for entry in entries {
        let path = entry?.path();
        if path.is_file() {
            let size = std::fs::File::open(&path)
                .and_then(|file| file.metadata())
                .map(|metadata| metadata.len() as f32)
                .unwrap_or(0.0);

            items.push(DirectoryItems::File(
                path.to_string_lossy().to_string(),
                size,
            ));
        } else if path.is_dir() {
            items.push(DirectoryItems::Folder(path.to_string_lossy().to_string()));
        }
    }

    items.sort_by(|a, b| {
        match a {
            DirectoryItems::Folder(_) => 0,

            DirectoryItems::File(_, _) => 1,
        }
        .cmp(match b {
            DirectoryItems::Folder(_) => &0,

            DirectoryItems::File(_, _) => &1,
        })
    });

    Ok(items)
}

/// Open the file/folder in the system's default application.
pub fn open_file_or_folder_in_os(path: String) {
    #[cfg(target_os = "linux")]
    let prog = "xdg-open";

    #[cfg(target_os = "windows")]
    let prog = "start";

    #[cfg(target_os = "macos")]
    let prog = "open";

    let _ = std::process::Command::new(prog).arg(path).spawn();
}

/// Get the home directory.
pub fn get_home_dir() -> String {
    #[cfg(not(target_os = "windows"))]
    return std::env::var_os("HOME")
        .unwrap()
        .to_string_lossy()
        .to_string();

    #[cfg(target_os = "windows")]
    return std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOMEPATH"))
        .unwrap()
        .to_string_lossy()
        .to_string();
}

/// Exit the app.
pub fn exit() {
    std::process::exit(0);
}
