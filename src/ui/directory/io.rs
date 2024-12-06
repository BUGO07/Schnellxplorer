/// This function lists files and folders from a given path.
pub fn list_files_and_folders(
    path: &str,
) -> Result<Vec<super::content::DirectoryItems>, std::io::Error> {
    let entries = std::fs::read_dir(path)?;
    let mut items = Vec::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            items.push(super::content::DirectoryItems::File(
                path.file_name().unwrap().to_str().unwrap().to_string(),
            ));
        } else if path.is_dir() {
            items.push(super::content::DirectoryItems::Folder(
                path.to_str().unwrap().to_string(),
            ));
        }
    }
    println!("{:?}", items);
    Ok(items)
}
