use std::{
    io::{Result},
    fs,
    path::{Path}
};

pub type DirectoryContent = Vec<(String, String)>;

pub fn fetch_content(directory_name: String) -> Result<DirectoryContent> {
    let directory_path = Path::new(&directory_name);

    if !directory_path.is_dir() {
        panic!("'{}' not a directory.", directory_name);
    }

    let result: DirectoryContent = fs::read_dir(directory_path)?
        .map(|entry| {
            // let entry_path = entry.unwrap().path();
            let entry_name = entry.unwrap().path().display().to_string();
            let content_type = get_content_type(Path::new(&entry_name))
                .expect(&format!("Unable to fetch content type for '{}'.", entry_name));

            (entry_name, content_type)
        })
        .collect();

    Ok(result)
}

fn get_content_type(path: &Path) -> Result<String> {
    if path.is_dir() {
        return Ok(String::from("directory"));
    } else if path.is_file() {
        return Ok(String::from("file"));
    }

    panic!("'{}' not found.", path.display().to_string());
}
