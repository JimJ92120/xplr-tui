use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Block, BorderType }
};

use crate::{
    types::{ Directory, DirectoryList }
};

#[derive(Clone)]
pub struct ParentDirectoryListData {
    pub current_directory: Directory,
    pub parent_directory_list: DirectoryList,
}

pub struct ParentDirectoryList {
    data: ParentDirectoryListData,
}

impl Widget for ParentDirectoryList {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let content = if self.data.parent_directory_list.is_empty() {
            self.get_no_list()
        } else {
            self.get_parent_directory_list()
        };

        Block::new()
            .border_type(BorderType::Rounded)
            // .borders(Borders::TOP)
            .title(content)
            .render(area, buffer)
    }
}

impl ParentDirectoryList {
    pub fn new(data: ParentDirectoryListData) -> Self {
        Self {
            data
        }
    }

    fn get_no_list(&self) -> String {
        self.data.current_directory.path_name.clone()
    }

    fn get_parent_directory_list(&self) -> String {
        let ParentDirectoryListData {
            current_directory,
            parent_directory_list,
            ..
        } = self.data.clone();

        if parent_directory_list.is_empty() {
            return current_directory.path_name;
        } else {
            let mut directory_list = parent_directory_list;
            directory_list.push(current_directory);

            directory_list
                .iter()
                .enumerate()
                .map(|(index, directory)| {
                    if 0 < index {
                        return directory.name.to_string();
                    } else {
                        directory.path_name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(" > ")
                .to_string()
        }
    }
}
