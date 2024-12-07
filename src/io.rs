/// This function lists files and folders from a given path.
pub fn list_files_and_folders(path: String) -> Result<Vec<crate::DirectoryItems>, std::io::Error> {
    let entries = std::fs::read_dir(path)?;
    let mut items = Vec::new();
    for entry in entries {
        let path = entry?.path();
        if path.is_file() {
            let size = if let Ok(file) = std::fs::File::open(path.clone()) {
                if let Ok(file) = file.metadata() {
                    file.len() as f32 / 1_000_000.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            items.push(crate::DirectoryItems::File(
                path.to_str().unwrap().to_string(),
                size,
            ));
        } else if path.is_dir() {
            items.push(crate::DirectoryItems::Folder(
                path.to_str().unwrap().to_string(),
            ));
        }
    }
    Ok(items)
}

/// Open the file/folder
pub fn open_file_or_folder_in_os(path: String) {
    #[cfg(target_os = "linux")]
    let prog = "xdg-open";

    #[cfg(target_os = "windows")]
    let prog = "start";

    #[cfg(target_os = "macos")]
    let prog = "open";

    let _ = std::process::Command::new(prog).arg(path.clone()).spawn();
}

/// Get the home directory.
/// Usually `/home/Username` on linux and `C:/Users/Username` on windows
pub fn get_home_dir() -> String {
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var_os("HOME")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    #[cfg(target_os = "windows")]
    {
        std::env::var_os("USERPROFILE")
            .or_else(|_| std::env::var_os("HOMEPATH"))
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// Exit the app
pub fn exit() {
    std::process::exit(0);
}
