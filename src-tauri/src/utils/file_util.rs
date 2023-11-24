use std::{fs, io};

pub fn get_file_name(file_path: &str) -> &str {
    std::path::Path::new(file_path)
        .file_stem()
        .and_then(|file_stem| file_stem.to_str())
        .unwrap_or("")
}

pub fn get_filename_from_url(url: &str) -> Option<String> {
    if let Some(file_name_start) = url.rfind('/') {
        let file_name = &url[file_name_start + 1..];

        // Remove any query parameters or fragments if they exist
        let file_name_end = file_name.find(|c| c == '?' || c == '#').unwrap_or_else(|| file_name.len());

        Some(file_name[..file_name_end].to_string())
    } else {
        None
    }
}

pub fn try_list_files_in_directory(dir_path: &str) -> Vec<String> {
    if let Ok(value) = list_files_in_directory(dir_path) {
        return value;
    }
    return vec![];
}

pub fn list_files_in_directory(dir_path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(dir_path)?;

    let mut file_names = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    file_names.push(file_name_str.to_string());
                }
            }
        }
    }

    Ok(file_names)
}

pub fn try_create_dir(path: &str) {
    if !dir_exists(&path) {
        create_dir(&path);
    }
}

pub fn dir_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        #[cfg(debug_assertions)]
        {
            println!("Error creating directory: {}", err);
        }
    } else {
        #[cfg(debug_assertions)]
        {
            println!("Directory created: {}", path);
        }
    }
}

pub fn delete_dir(path: &str) {
    if let Err(err) = fs::remove_dir_all(path) {
        #[cfg(debug_assertions)]
        {
            println!("Error deleting directory: {}", err);
        }
    } else {
        #[cfg(debug_assertions)]
        {
            println!("Directory deleted: {}", path);
        }
    }
}