use std::{
    env,
    fs::{ self, metadata },
    io::{ Result },
    path::{ Path },
    os::unix::fs::{ PermissionsExt }
};

use crate::{
    types::{ DirectoryItemType, DirectoryItem, Directory, DirectoryContent }
};

pub struct DirectoryController {}

impl DirectoryController {
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
            name: Self::get_formatted_path_name(directory_path_name.clone()),
            path_name: directory_path_name.clone(),
            content: Self::get_directory_content(directory_path_name.clone())?,
        })
    }

    fn get_directory_content(directory_path_name: String) -> Result<DirectoryContent> {
        let directory_path = Path::new(&directory_path_name);
        let directory_content: DirectoryContent = fs::read_dir(directory_path)?
            .map(|entry| {
                let entry = entry.expect(&format!("Invalid entry in {}.", directory_path_name));
                let entry_path = entry.path();
                let path_name = entry_path.display().to_string();
                let item_type = Self::get_content_type(Path::new(&path_name))
                    .expect(&format!("Unable to fetch content type for '{}'.", path_name));
                let metadata = metadata(&entry_path)
                    .expect(&format!("Unable to retrieve '{}' metadata.", entry_path.display().to_string()));
                let permissions_mode = metadata.permissions().mode();

                DirectoryItem {
                    name: Self::get_formatted_path_name(path_name.clone()),
                    path_name: path_name.clone(),
                    item_type: item_type.clone(),
                    permissions: Self::get_formatted_permissions(permissions_mode, item_type),
                    can_read: Self::can_read(permissions_mode),
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

        Ok(fs::read_to_string(file_path)
            .unwrap_or(String::new())
        )
    }

    fn get_content_type(path: &Path) -> Result<DirectoryItemType> {
        if path.is_dir() {
            return Ok(DirectoryItemType::Directory);
        } else if path.is_file() {
            return Ok(DirectoryItemType::File);
        }

        panic!("'{}' not found.", path.display().to_string());
    }

    fn get_formatted_path_name(path_name: String) -> String {
        match path_name.rsplit_once("/") {
            Some((_, name)) => name.to_string(),
            None => path_name
        }
    }

    fn get_formatted_permissions(permissions_mode: u32, item_type: DirectoryItemType) -> String {
        let user = [
            if permissions_mode & 0o400 != 0 { 'r' } else { '-' },
            if permissions_mode & 0o200 != 0 { 'w' } else { '-' },
            if permissions_mode & 0o100 != 0 { 'x' } else { '-' },
        ];
        let group = [
            if permissions_mode & 0o040 != 0 { 'r' } else { '-' },
            if permissions_mode & 0o020 != 0 { 'w' } else { '-' },
            if permissions_mode & 0o010 != 0 { 'x' } else { '-' },
        ];
        let other = [
            if permissions_mode & 0o004 != 0 { 'r' } else { '-' },
            if permissions_mode & 0o002 != 0 { 'w' } else { '-' },
            if permissions_mode & 0o001 != 0 { 'x' } else { '-' },
        ];

        let file_type = if DirectoryItemType::Directory == item_type {
            'd'
        } else {
            '-'
        };

        format!(
            "{}{}{}{}", 
            file_type, 
            user.iter().collect::<String>(),
            group.iter().collect::<String>(),
            other.iter().collect::<String>()
        )
    }

    fn can_read(permissions_mode: u32) -> bool {
        permissions_mode & 0o400 != 0
    }
}
