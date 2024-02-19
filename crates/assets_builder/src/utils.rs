use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn list_png_files_in_folder(folder_path: &str) -> std::io::Result<Vec<String>> {
    let mut png_files = Vec::new();

    // Read the directory specified by folder_path
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and its extension is .png
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("png") {
            // Convert the path to a string and add it to the vector
            if let Some(path_str) = path.to_str() {
                png_files.push(path_str.to_string());
            }
        }
    }

    Ok(png_files)
}

pub fn list_directories(path: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    // Read the directory specified by `path`
    let entries = fs::read_dir(path)?;

    Ok(entries.filter_map(|entry| {
        let entry = entry.ok()?;
        let path = entry.path();

        // Check if the entry is a directory
        if path.is_dir() {
            return Some(path);
        }
        None
    }))
}
