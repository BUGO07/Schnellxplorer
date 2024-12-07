/// This function lists files and folders from a given path.
pub fn list_files_and_folders(path: String) -> Result<Vec<crate::DirectoryItems>, std::io::Error> {
    let entries = std::fs::read_dir(path)?;
    let mut items = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            items.push(crate::DirectoryItems::File(
                path.to_str().unwrap().to_string(),
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
    //TODO: make this for windows and mac aswell
    let _ = std::process::Command::new("xdg-open")
        .arg(path.clone())
        .spawn();
}
