use ratatui::{
    buffer::Buffer,
    layout::{
        Rect
    },
    widgets::{
        Widget,
        Paragraph
    },
};

use crate::types::{
    Directory,
};

#[derive(Clone)]
pub struct ParentDirectoryListData {
    pub directory: Directory,
    pub parent_directory_list: Vec<String>,
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
        Paragraph::new(self.data.directory.path_name.clone())
            .render(area, buffer);
    }

    fn render_parent_directory_list(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_parent_directory_list())
            .render(area, buffer);
    }

    fn get_parent_directory_list(&self) -> String {
        let ParentDirectoryListData {
            directory,
            parent_directory_list,
            ..
        } = self.data.clone();

        if parent_directory_list.is_empty() {
            return directory.path_name;
        } else {
            let mut directory_list = parent_directory_list;
            directory_list.push(directory.path_name);

            directory_list
                .iter()
                .enumerate()
                .map(|(index, directory_path_name)| {
                    if 0 < index {
                        let (_, formatted_directory_name) = directory_path_name
                            .split_once(
                                format!("{}/", directory_list[index - 1].clone()).as_str()
                            )
                            .unwrap();

                        format!(" > {}", formatted_directory_name).to_string()
                    } else {
                        directory_path_name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
                .to_string()
        }
    }
}
