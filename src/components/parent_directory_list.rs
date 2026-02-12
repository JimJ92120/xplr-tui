use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Paragraph }
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
        if self.data.parent_directory_list.is_empty() {
            self.render_no_list(area, buffer);
        } else {
            self.render_parent_directory_list(area, buffer);
        }
    }
}

impl ParentDirectoryList {
    pub fn new(data: ParentDirectoryListData) -> Self {
        Self {
            data
        }
    }

    fn render_no_list(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.data.current_directory.path_name.clone())
            .render(area, buffer);
    }

    fn render_parent_directory_list(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_parent_directory_list())
            .render(area, buffer);
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
