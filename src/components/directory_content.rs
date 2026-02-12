use ratatui::{
    buffer:: { Buffer },
    layout::{ Rect },
    text::{ Line },
    widgets::{ Widget, Paragraph, Block, BorderType },
    style::{ Color, Stylize }
};

use crate::{
    types::{ Directory, DirectoryItem, DirectoryItemType, DirectoryList }
};

#[derive(Clone)]
pub struct DirectoryContentData {
    pub current_directory: Directory,
    pub selected_item_index: usize,
    pub parent_directory_list: DirectoryList,
}

pub struct DirectoryContent {
    data: DirectoryContentData,
}

impl Widget for DirectoryContent {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let list = if self.data.current_directory.content.is_empty() {
            self.get_no_list()
        } else {
            self.get_list()
        };

        let title = if self.data.parent_directory_list.is_empty() {
            self.no_parent_directory_list()
        } else {
            self.parent_directory_list()
        };

        list
            .block(self.get_container(title))
            .scroll((self.scroll_value(area.height), 0))
            .render(area, buffer);
    }
}

impl DirectoryContent {
    pub fn new(data: DirectoryContentData) -> Self {
        Self {
            data
        }
    }

    fn get_container(&self, title: String) -> Block<'_> {
        Block::bordered()
            .border_type(BorderType::Rounded)
            .title(format!(" {} ", title))
    }

    fn get_no_list(&self) -> Paragraph<'_> {
        Paragraph::new("No item found.")
    }

    fn get_list(&self) -> Paragraph<'_> {
        Paragraph::new(
            self.data.current_directory.content.clone()
                .iter()
                .enumerate()
                .map(|(index, item)| self.get_list_item(item.clone(), index))
                .collect::<Vec<Line>>()
        )
    }

    fn get_list_item(&self, item: DirectoryItem, index: usize) -> Line<'_> {
        let item_content = if DirectoryItemType::Directory == item.item_type {
            format!("{}/", item.name)
        } else {
            item.name
        };
        let line = Line::from(item_content);

        if self.data.selected_item_index == index {
            return line.bg(Color::Green);
        }

        line
    }

    fn scroll_value(&self, container_height: u16) -> u16 {
        let selected_item_index = self.data.selected_item_index.clone() as u16;

        if selected_item_index < container_height {
            0
        } else {
            selected_item_index - container_height + 3
        }
    }

    fn parent_directory_list(&self) -> String {
        let DirectoryContentData {
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

    fn no_parent_directory_list(&self) -> String {
        self.data.current_directory.path_name.clone()
    }
}
