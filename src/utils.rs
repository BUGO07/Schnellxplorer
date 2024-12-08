use std::path::{Path, PathBuf};

/// This function takes in a file size and returns it with an appropriate unit.
pub fn size_units(size: f32) -> (String, String) {
    let (display_size, unit) = if size > 1_000_000_000.0 {
        (size / 1_000_000_000.0, "GB")
    } else if size > 1_000_000.0 {
        (size / 1_000_000.0, "MB")
    } else if size > 1_000.0 {
        (size / 1_000.0, "KB")
    } else {
        (size, "B")
    };

    let size_string = if unit == "B" {
        format!("{:.0}", display_size)
    } else {
        format!("{:.2}", display_size)
    };

    (size_string, unit.to_string())
}

/// This function normalizes/formats the path.
pub fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();

    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                if normalized != PathBuf::new() {
                    normalized.pop();
                }
            }
            std::path::Component::Normal(normal_part) => {
                normalized.push(normal_part);
            }
            std::path::Component::RootDir => normalized.push(component),
            _ => {}
        }
    }

    normalized
}
