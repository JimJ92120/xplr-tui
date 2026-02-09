use crate::Api;

#[derive(Clone)]
pub struct State {
    pub is_running: bool,
    pub title: String,
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize,
    pub parent_directory_list: Vec<String>,
    pub text_input: String
}

impl State {
    pub fn new(state: State) -> Self {
        Self { ..state }
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
