use std::any::Any;

use crate::types::{
    DirectoryItemType,
    DirectoryItem,
    Directory,
    DirectoryList,
};

use super::{
    NestedStore,
};

mod controller;

use controller::{
    DirectoryController,
};

#[derive(Debug, Clone)]
pub struct DirectoryStore {
    directory: Directory,
    selected_item_index: usize,
    parent_directory_list: DirectoryList,
    preview: String,
}

impl NestedStore for DirectoryStore {
    fn get(&self, field: &str) -> Box<dyn Any> {
        match field {
            "directory" => Box::new(self.directory.clone()),
            "selected_item_index" => Box::new(self.selected_item_index.clone()),
            "selected_item" => Box::new(self.selected_item()),
            "parent_directory_list" => Box::new(self.parent_directory_list.clone()),
            "preview" => Box::new(self.preview.clone()),

            _ => panic!("{}", Self::no_field_found(field)),
        }
    }

    fn action(&mut self, action: &str) {
        match action {
            "select_previous_item" => self.select_previous_item(),
            "select_next_item" => self.select_next_item(),
            "select_first_item" => self.select_first_item(),
            "select_last_item" => self.select_last_item(),
            "load_next_directory" => self.load_next_directory(),
            "load_previous_directory" => self.load_previous_directory(),

            _ => panic!("{}", Self::no_action_found(action)),
        };
    }
}

impl DirectoryStore {
    pub fn new(path_name: String) -> Self {
        let root_directory_path_name = DirectoryController::get_root_directory_path_name(path_name.clone())
            .expect(&format!("Unable to initialize Store with '{}' path name", path_name));
        let root_directory = DirectoryController::get_directory(root_directory_path_name.clone())
            .expect(&format!("Unable to initialize Store with '{}' directory name", root_directory_path_name));

        let mut store = Self {
            directory: root_directory,
            selected_item_index: 0,
            parent_directory_list: Vec::new(),
            preview: String::new(),
        };
        store.load_preview();

        store
    }

    fn selected_item(&self) -> Option<DirectoryItem> {
        let Self {
            directory,
            selected_item_index,
            ..
        } = self;

        if directory.content.is_empty() {
            return None;
        }

        Some(directory.content[*selected_item_index].clone())
    }

    fn update_selected_item_index(&mut self, new_index: usize) {        
        // bound to check before calling to avoid unnecessary checks at runtime
        self.selected_item_index = new_index;
        self.clear_preview();

        match self.selected_item() {
            Some(item) => {
                if DirectoryItemType::File == item.item_type {
                    self.load_preview();
                }
            },
            None => ()
        }
    }
    fn select_next_item(&mut self) {
        if self.selected_item_index < self.directory.content.len() - 1 {
            self.update_selected_item_index(self.selected_item_index + 1);
        }
    }
    fn select_previous_item(&mut self) {
        if self.selected_item_index > 0 {
            self.update_selected_item_index(self.selected_item_index - 1);
        }
    }
    fn select_first_item(&mut self) {
        self.update_selected_item_index(0);
    }
    fn select_last_item(&mut self) {
        if !self.directory.content.is_empty() {
            self.update_selected_item_index(self.directory.content.len() - 1);
        } else {
            self.selected_item_index = 0;
        }
    }

    fn load_next_directory(&mut self) {
        let Self {
            directory,
            ..
        } = self.clone();

        match self.selected_item() {
            Some(selected_item) => {
                if DirectoryItemType::Directory == selected_item.item_type {
                    let next_directory_path_name = selected_item.path_name.clone();
                    let next_directory = DirectoryController::get_directory(next_directory_path_name.clone());

                    match next_directory {
                        Ok(next_directory) => {
                            self.parent_directory_list.push(directory.clone());
                            self.directory = next_directory.clone();

                            self.update_selected_item_index(0)
                        },
                        Err(error) => panic!("Unable to retrieve content for '{}' directory.\n{}", next_directory_path_name, error)
                    };
                } 
            },
            None => ()
        }
    }
    fn load_previous_directory(&mut self) {
        let Self {
            directory,
            parent_directory_list,
            ..
        } = self.clone();

        if parent_directory_list.is_empty() {
            return;
        }

        let current_directory_path_name = directory.path_name;

        match self.parent_directory_list.pop() {
            Some(previous_directory) => {
                self.directory = previous_directory.clone();
                
                let selected_item_index = previous_directory.content
                        .iter()
                        .position(|item| current_directory_path_name == item.path_name)
                        .unwrap();
                self.update_selected_item_index(selected_item_index);
            },
            None => panic!(
                "Unable to retrieve parent directory for '{}' directory.",
                current_directory_path_name
            )
        };
    }

    fn clear_preview(&mut self) {
        self.preview = String::new()
    }
    fn load_preview(&mut self) {
        let selected_item = self.selected_item();

        if selected_item.is_none() {
            return;
        }

        let selected_item = selected_item.unwrap();
        if DirectoryItemType::File == selected_item.item_type {
            self.preview = DirectoryController::get_file_content(selected_item.path_name.clone()).expect(
                &format!("'{}' is not a file.", selected_item.path_name)
            );
        }
    }
}
