use std::collections::HashMap;

mod directory;
use directory::{DirectoryContent};

type DirectoryList = HashMap<String, DirectoryContent>;

#[derive(Debug)]
pub struct App {
    root_directory_name: String,
    current_directory_name: String,
    directory_list: DirectoryList
}

impl App {
    pub fn new(root_directory_name: String) -> Self{
        let mut app = Self {
            root_directory_name,
            current_directory_name: String::new(),
            directory_list: HashMap::new()
        };
        app.set_current_directory(app.get_root_directory_name());

        app
    }

    pub fn get_root_directory_name(&self) -> String {
        self.root_directory_name.clone()
    }

    pub fn get_current_directory_name(&self) -> String {
        self.current_directory_name.clone()
    }

    pub fn get_current_directory_content(&self) -> DirectoryContent {
        self.directory_list
            .get(&self.current_directory_name).clone()
            .unwrap_or(&(Vec::new() as DirectoryContent))
            .clone()
    }

    fn set_current_directory(&mut self, directory_name: String) {
        self.current_directory_name = directory_name.clone();

        if self.directory_list.contains_key(&self.current_directory_name) {
            println!("'{}' already added.", directory_name);
        } else {
            println!("adding '{}'.", directory_name);
            self.add_directory(self.current_directory_name.clone());
        }
    }

    fn add_directory(&mut self, directory_name: String) {
        self.directory_list.insert(
            directory_name.clone(),
            directory::fetch_content(directory_name.clone()).expect(
                &format!("Unable to fetch '{}' directory content.", directory_name)
            )
        );
    }
}
