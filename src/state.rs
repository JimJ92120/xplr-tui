use crate::types::{
    DirectoryItemType,
    DirectoryItem,
    Directory,
    DirectoryList,
};
use crate::Api;

#[derive(Clone)]
pub struct State {
    is_running: bool,
    title: String,
    directory: Directory,
    selected_item_index: usize,
    parent_directory_list: DirectoryList,
    text_input: String,
    preview: String,
}

impl State {
    pub fn new(
        title: String,
        path_name: String,
    ) -> Self {
        let directory_path_name = Api::get_root_directory_path_name(path_name.clone())
            .expect(&format!("Unable to initialize State with '{}' path name", path_name));
        let directory = Api::get_directory(directory_path_name.clone())
            .expect(&format!("Unable to initialize State with '{}' directory name", directory_path_name));

        let mut state = Self {
            is_running: false,
            title,
            directory,
            selected_item_index: 0,
            parent_directory_list: Vec::new(),
            text_input: String::new(),
            preview: String::new()
        };
        state.load_preview();

        state
    }

    pub fn is_running(&self) -> bool {
        self.is_running.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn directory(&self) -> Directory {
        self.directory.clone()
    }
    pub fn selected_item_index(&self) -> usize {
        self.selected_item_index.clone()
    }
    pub fn parent_directory_list(&self) -> DirectoryList {
        self.parent_directory_list.clone()
    }
    pub fn text_input(&self) -> String {
        self.text_input.clone()
    }
    pub fn preview(&self) -> String {
        self.preview.clone()
    }
    pub fn selected_item(&self) -> Option<DirectoryItem> {
        let State {
            directory,
            selected_item_index,
            ..
        } = self;

        if directory.content.is_empty() {
            return None;
        }

        Some(directory.content[*selected_item_index].clone())
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn select_next_item(&mut self) {
        if self.selected_item_index < self.directory.content.len() - 1 {
            self.update_selected_item_index(self.selected_item_index + 1);
        }
    }
    pub fn select_previous_item(&mut self) {
        if self.selected_item_index > 0 {
            self.update_selected_item_index(self.selected_item_index - 1);
        }
    }
    pub fn select_first_item(&mut self) {
        self.update_selected_item_index(0);
    }
    pub fn select_last_item(&mut self) {
        if !self.directory.content.is_empty() {
            self.update_selected_item_index(self.directory.content.len() - 1);
        } else {
            self.selected_item_index = 0;
        }
    }

    pub fn load_next_directory(&mut self) {
        let State {
            directory,
            ..
        } = self.clone();

        match self.selected_item() {
            Some(selected_item) => {
                if DirectoryItemType::Directory == selected_item.item_type {
                    let next_directory_path_name = selected_item.path_name.clone();
                    let next_directory = Api::get_directory(next_directory_path_name.clone());

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
    pub fn load_previous_directory(&mut self) {
        let State {
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

    pub fn type_text(&mut self, char: char) {
        self.text_input.push(char);
    }
    pub fn delete_text_last_char(&mut self) {
        if !self.text_input.is_empty() {
            self.text_input.pop();
        }
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
            self.preview = Api::get_file_content(selected_item.path_name.clone()).expect(
                &format!("'{}' is not a file.", selected_item.path_name)
            );
        }
    }
}
