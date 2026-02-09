use crate::Api;

#[derive(Clone)]
pub struct State {
    is_running: bool,
    title: String,
    directory_name: String,
    directory_content: Vec<(String, String)>,
    selected_item_index: usize,
    parent_directory_list: Vec<String>,
    text_input: String
}

impl State {
    pub fn new(
        title: String,
        path_name: String,
    ) -> Self {
        let directory_name = Api::get_root_directory_name(path_name.clone())
            .expect(&format!("Unable to initialize State with '{}' path name", path_name));
        let directory_content = Api::get_directory_content(directory_name.clone())
            .expect(&format!("Unable to initialize State with '{}' directory name", directory_name));

        Self {
            is_running: false,
            title,
            directory_content,
            directory_name,
            selected_item_index: 0,
            parent_directory_list: Vec::new(),
            text_input: String::new()
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running.clone()
    }
    pub fn title(&self) -> String {
        self.title.clone()
    }
    pub fn directory_name(&self) -> String {
        self.directory_name.clone()
    }
    pub fn directory_content(&self) -> Vec<(String, String)> {
        self.directory_content.clone()
    }
    pub fn selected_item_index(&self) -> usize {
        self.selected_item_index.clone()
    }
    pub fn parent_directory_list(&self) -> Vec<String> {
        self.parent_directory_list.clone()
    }
    pub fn text_input(&self) -> String {
        self.text_input.clone()
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn select_next_item(&mut self) {
        if self.selected_item_index < self.directory_content.len() - 1 {
            self.selected_item_index +=1;
        }
    }
    pub fn select_previous_item(&mut self) {
        if self.selected_item_index > 0 {
            self.selected_item_index -= 1;
        }
    }
    pub fn select_first_item(&mut self) {
        self.selected_item_index = 0;
    }
    pub fn select_last_item(&mut self) {
        if !self.directory_content.is_empty() {
            self.selected_item_index = self.directory_content.len() - 1;
        } else {
            self.selected_item_index = 0;
        }
    }

    pub fn load_next_directory(&mut self) {
        let State {
            directory_content,
            selected_item_index,
            directory_name,
            ..
        } = self.clone();
        let selected_item = directory_content[selected_item_index].clone();

        if "directory" == selected_item.1 {
            let next_directory_name = selected_item.0.clone();
            let next_directory_content = Api::get_directory_content(next_directory_name.to_string());

            match next_directory_content {
                Ok(directory_content) => {
                    self.parent_directory_list.push(directory_name);
                    self.directory_name = next_directory_name;
                    self.directory_content = directory_content;
                    self.selected_item_index = 0;
                },
                Err(error) => panic!("Unable to retrieve content for '{}' directory.\n{}", next_directory_name, error)
            };
        } 
    }
    pub fn load_previous_directory(&mut self) {
        let State {
            directory_name,
            parent_directory_list,
            ..
        } = self.clone();

        if parent_directory_list.is_empty() {
            return;
        }

        let current_directory_name = directory_name;
        let previous_directory_name = parent_directory_list.last().unwrap().to_string();
        let previous_directory_content = Api::get_directory_content(previous_directory_name.to_string());

        match previous_directory_content {
            Ok(directory_content) => {
                self.parent_directory_list.pop();
                self.directory_name = previous_directory_name;
                self.directory_content = directory_content.clone();
                self.selected_item_index = directory_content
                        .iter()
                        .position(|directory_name| current_directory_name == directory_name.0)
                        .unwrap();
            },
            Err(error) => panic!(
                "Unable to retrieve previous directory content for '{}' directory.\n{}",
                previous_directory_name,
                error
            )
        };
    }

    pub fn type_text(&mut self, char: char) {
        self.text_input.push(char);
    }
}
