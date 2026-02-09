use std::{
    env,
    io::{Result},
    path::{Path}
};

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
