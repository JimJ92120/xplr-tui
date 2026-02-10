use std::{
    env,
    fs,
    io::{ Result },
    path::{ Path }
};

use crate::types::{
    DirectoryItemType,
    DirectoryItem,
    DirectoryContent,
    Directory,
};

pub struct Api {}

impl Api {
    pub fn get_root_directory_path_name(path_name: String) -> Result<String> {
        let current_directory = env::current_dir().unwrap();
        let mut directory_path_name = current_directory.display().to_string();

        if "" != path_name && "./" != path_name {
            if path_name.chars().nth(0).unwrap().is_alphanumeric() {
                directory_path_name = format!("{}/{}", directory_path_name, path_name);
            } else if path_name.starts_with("./") {
                let (_, split) = path_name.split_at(2);

                directory_path_name = format!("{}/{}", directory_path_name, split);
            } else if path_name.starts_with("/") {
                directory_path_name = path_name.clone();
            } else {
                panic!("'{}' path not accepted.", path_name);
            }
        }

        let directory_path = Path::new(&directory_path_name);

        if !directory_path.is_dir() {
            panic!("'{}' not a directory.", directory_path_name);
        }

        Ok(directory_path_name)
    }

    pub fn get_directory(directory_path_name: String) -> Result<Directory> {
        let directory_path = Path::new(&directory_path_name);

        if !directory_path.is_dir() {
            panic!("'{}' not a directory.", directory_path_name);
        }

        Ok(Directory {
            path_name: directory_path_name.clone(),
            content: Self::get_directory_content(directory_path_name.clone())?,
        })
    }

    fn get_directory_content(directory_path_name: String) -> Result<DirectoryContent> {
        let directory_path = Path::new(&directory_path_name);
        let directory_content: DirectoryContent = fs::read_dir(directory_path)?
            .map(|entry| {
                let path_name = entry.unwrap().path().display().to_string();

                DirectoryItem {
                    name: String::from("item"),
                    path_name: path_name.clone(),
                    item_type: Self::get_content_type(Path::new(&path_name))
                        .expect(&format!("Unable to fetch content type for '{}'.", path_name))
                }
            })
            .collect();

        Ok(directory_content)
    }

    pub fn get_file_content(file_name: String) -> Result<String> {
        let file_path = Path::new(&file_name);

        if !file_path.is_file() {
            panic!("'{}' is not a file.", file_name);
        }

        fs::read_to_string(file_path)
    }

    fn get_content_type(path: &Path) -> Result<DirectoryItemType> {
        if path.is_dir() {
            return Ok(DirectoryItemType::Directory);
        } else if path.is_file() {
            return Ok(DirectoryItemType::File);
        }

        panic!("'{}' not found.", path.display().to_string());
    }
}
