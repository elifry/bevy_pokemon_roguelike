use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn list_files_in_folder(
    folder_path: &Path,
    file_ext: Option<&str>,
) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();

    // Read the directory specified by folder_path
    let entries = fs::read_dir(folder_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the entry is a file and its extension is .png
        if path.is_file()
            && (file_ext.is_none() || path.extension().and_then(|ext| ext.to_str()) == file_ext)
        {
            // Convert the path to a string and add it to the vector
            if let Some(path_str) = path.to_str() {
                files.push(path_str.to_string());
            }
        }
    }

    Ok(files)
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
