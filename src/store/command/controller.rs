use std::{
    fs,
    io::{ Result },
    path::{ Path }
};

pub struct CommandController {}

impl CommandController {
    pub fn copy_file_or_directory(source_path_name: String, target_path_name: String) -> Result<()> {
        let source_path = Path::new(&source_path_name);
        if !source_path.exists() {
            panic!("'{}' doesn't exist.", source_path_name);
        }

        let target_path = Path::new(&target_path_name);
        if target_path.exists() {
            panic!("'{}' already exists.", target_path_name);
        }

        if let Some(target_parent_directory) = target_path.parent() {
            fs::create_dir_all(target_parent_directory)?;
        }

        if source_path.is_file() {
            Self::copy_file(source_path_name, target_path_name)
        } else {
            Self::copy_directory_recursively(source_path_name, target_path_name)
        }
    }

    fn copy_file(source_path_name: String, target_path_name: String) -> Result<()> {
        match fs::copy(&source_path_name, &target_path_name) {
            Ok(_) => Ok(()),
            Err(error) => panic!("Unable to copy '{}' to '{}'\n{}", source_path_name, target_path_name, error), 
        }
    }

    fn copy_directory_recursively(source_path_name: String, target_path_name: String) -> Result<()> {
        let source_path = Path::new(&source_path_name);
        let target_path = Path::new(&target_path_name);

        fs::create_dir_all(&target_path_name)?;

        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let entry_type = entry.file_type()?;
            let entry_path_name = entry.path().display().to_string();
            let entry_target_path = target_path.join(entry.file_name());
            let entry_target_path_name = entry_target_path.as_path().display().to_string();

            if entry_type.is_dir() {
                Self::copy_directory_recursively(entry_path_name, entry_target_path_name)?;
            } else if entry_type.is_file() {
                Self::copy_file(entry_path_name, entry_target_path_name)?;
            } else {
                panic!("Unable to copy '{}' to '{}'", entry_path_name, entry_target_path_name);
            }
        }

        Ok(())
    }

    pub fn move_file_or_directory(source_path_name: String, target_path_name: String) -> Result<()> {
        let source_path = Path::new(&source_path_name);
        if !source_path.exists() {
            panic!("'{}' doesn't exist.", source_path_name);
        }

        let target_path = Path::new(&target_path_name);
        if target_path.exists() {
            panic!("'{}' already exists.", target_path_name);
        }

        if let Some(target_parent_directory) = target_path.parent() {
            fs::create_dir_all(target_parent_directory)?;
        }

        match fs::rename(&source_path_name, &target_path_name) {
            Ok(_) => Ok(()),
            Err(error) => panic!("Unable to move '{}' to '{}'.\n{}", source_path_name, target_path_name, error),
        }
    }

    pub fn delete_file_or_directory(source_path_name: String) -> Result<()> {
        let source_path = Path::new(&source_path_name);
        if !source_path.exists() {
            panic!("'{}' doesn't exist.", source_path_name);
        }

        if source_path.is_file() {
            match fs::remove_file(&source_path_name) {
                Ok(_) => Ok(()),
                Err(error) => panic!("Unable to delete '{}' file.\n{}", source_path_name, error),
            }
        } else {
            match fs::remove_dir_all(&source_path_name) {
                Ok(_) => Ok(()),
                Err(error) => panic!("Unable to delete '{}' directory.\n{}", source_path_name, error),
            }
        }
    }
}
