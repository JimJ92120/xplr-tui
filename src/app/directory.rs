use std::{
    io::{Result},
    fs,
    path::{Path}
};

pub type DirectoryContent = Vec<String>;

pub fn fetch_content(directory_name: String) -> Result<DirectoryContent> {
    let directory_path = Path::new(&directory_name);

    if !directory_path.is_dir() {
        panic!("'{}' not a directory.", directory_name);
    }

    let result: Vec<String> = fs::read_dir(directory_path)?
        .map(|entry| {
            let entry_path = entry.unwrap().path();

            entry_path.display().to_string()
        })
        .collect();

    Ok(result)
}
