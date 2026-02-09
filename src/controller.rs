use crate::ClientState;
use crate::Api;

pub struct Controller {}

impl Controller {
    pub fn stop(state: &mut ClientState) {
        state.is_running = false;
    }

    pub fn select_next_item(state: &mut ClientState) {
        if state.selected_item_index < state.directory_content.len() - 1 {
            state.selected_item_index +=1;
        }
    }
    pub fn select_previous_item(state: &mut ClientState) {
        if state.selected_item_index > 0 {
            state.selected_item_index -= 1;
        }
    }

    pub fn load_next_directory(state: &mut ClientState) {
        let ClientState {
            directory_content,
            selected_item_index,
            directory_name,
            ..
        } = state.clone();
        let selected_item = directory_content[selected_item_index].clone();

        if "directory" == selected_item.1 {
            let next_directory_name = selected_item.0.clone();
            let next_directory_content = Api::get_directory_content(next_directory_name.to_string());

            match next_directory_content {
                Ok(directory_content) => {
                    state.parent_directory_list.push(directory_name);
                    state.directory_name = next_directory_name;
                    state.directory_content = directory_content;
                    state.selected_item_index = 0;
                },
                Err(error) => panic!("Unable to retrieve content for '{}' directory.\n{}", next_directory_name, error)
            };
        } 
    }
    pub fn load_previous_directory(state: &mut ClientState) {
        let ClientState {
            directory_name,
            parent_directory_list,
            ..
        } = state.clone();

        if parent_directory_list.is_empty() {
            return;
        }

        let current_directory_name = directory_name;
        let previous_directory_name = parent_directory_list.last().unwrap().to_string();
        let previous_directory_content = Api::get_directory_content(previous_directory_name.to_string());

        match previous_directory_content {
            Ok(directory_content) => {
                state.parent_directory_list.pop();
                state.directory_name = previous_directory_name;
                state.directory_content = directory_content.clone();
                state.selected_item_index = directory_content
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

    pub fn type_text(state: &mut ClientState, char: char) {
        state.text_input.push(char);
    }
}
