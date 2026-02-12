use std::{
    fs,
    io::{ Result },
    path::{ Path }
};

pub struct CommandController {}

impl CommandController {
    pub fn copy_file(source_path_name: String, target_path_name: String) -> Result<()> {
        let source_path = Path::new(&source_path_name);
        if !source_path.is_file() {
            panic!("'{}' is not a file.", source_path_name);
        }

        let target_path = Path::new(&target_path_name);
        if target_path.exists() {
            panic!("'{}' already exists.", target_path_name);
        }

        match fs::copy(&source_path_name, &target_path_name) {
            Ok(_) => Ok(()),
            Err(error) => panic!("Unable to copy '{}' to '{}'\n{}", source_path_name, target_path_name, error), 
        }
    }
}
