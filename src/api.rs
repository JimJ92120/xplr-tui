use std::{
    env,
    fs,
    io::{ Result },
    path::{ Path }
};

type DirectoryContent = Vec<(String, String)>;

pub struct Api {}

impl Api {
    pub fn get_root_directory_name(path_name: String) -> Result<String> {
        let current_directory = env::current_dir().unwrap();
        let mut directory_name = current_directory.display().to_string();

        if "" != path_name && "./" != path_name {
            if path_name.chars().nth(0).unwrap().is_alphanumeric() {
                directory_name = format!("{}/{}", directory_name, path_name);
            } else if path_name.starts_with("./") {
                let (_, split) = path_name.split_at(2);

                directory_name = format!("{}/{}", directory_name, split);
            } else if path_name.starts_with("/") {
                directory_name = path_name.clone();
            } else {
                panic!("'{}' path not accepted.", path_name);
            }
        }

        let directory_path = Path::new(&directory_name);

        if !directory_path.is_dir() {
            panic!("'{}' not a directory.", directory_name);
        }

        Ok(directory_name)
    }

    pub fn get_directory_content(directory_name: String) -> Result<DirectoryContent> {
        let directory_path = Path::new(&directory_name);

        if !directory_path.is_dir() {
            panic!("'{}' not a directory.", directory_name);
        }

        let result: DirectoryContent = fs::read_dir(directory_path)?
            .map(|entry| {
                let entry_name = entry.unwrap().path().display().to_string();
                let content_type = Self::get_content_type(Path::new(&entry_name))
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
}
