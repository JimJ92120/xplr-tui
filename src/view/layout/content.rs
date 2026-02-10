use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    text::{ Line },
    widgets::{
        Widget,
        Paragraph
    },
};

use crate::types::{
    DirectoryItemType,
    DirectoryItem,
    Directory,
};
use super::super::components::{
    directory_list::{
        DirectoryContent,
        DirectoryContentData
    },
    parent_directory_list::{
        ParentDirectoryList,
        ParentDirectoryListData,
    }
};

#[derive(Clone)]
pub struct ContentData {
    pub directory: Directory,
    pub selected_item_index: usize,
    pub selected_item: Option<DirectoryItem>,
    pub parent_directory_list: Vec<String>,
    pub preview: String,
}

pub struct Content {
    data: ContentData,
}

impl Widget for Content {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let [left_container, right_container] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        self.render_left_container(left_container, buffer);
        self.render_right_container(right_container, buffer);
    }
}

impl Content {
    pub fn new(data: ContentData) -> Self {
        Self {
            data
        }
    }

    fn render_left_container(&self, area: Rect, buffer: &mut Buffer) {
        let [title_container, list_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]).areas(area);

        ParentDirectoryList::new(ParentDirectoryListData {
            directory: self.data.directory.clone(),
            parent_directory_list: self.data.parent_directory_list.clone(),
        })
            .render(title_container, buffer);
        DirectoryContent::new(DirectoryContentData {
            directory: self.data.directory.clone(),
            selected_item_index: self.data.selected_item_index.clone(),
        })
            .render(list_container, buffer);
    }

    fn render_right_container(&self, area: Rect, buffer: &mut Buffer) {
        let ContentData {
            selected_item,
            ..
        } = self.data.clone();

        match selected_item {
            Some(selected_item) => {
                let [details_container, preview_container] = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ]).areas(area);

                Paragraph::new(self.get_details())
                    .render(details_container, buffer);

                if DirectoryItemType::File == selected_item.item_type {
                    Paragraph::new(self.get_preview())
                        .render(preview_container, buffer);
                }
            },
            None => {
                Paragraph::new("No item selected")
                    .render(area, buffer);
            }
        }
    }

    fn get_details(&self) -> Vec<Line<'static>> {
        let ContentData {
            selected_item,
            ..
        } = self.data.clone();

        let details: Vec<Line> = if !selected_item.is_none() {
            let selected_item = selected_item.unwrap();

            vec![
                Line::from("Details:"),
                Line::from(format!(
                    "- name: {}",
                    selected_item.name
                )),
                Line::from(format!(
                    "- type: {:?}",
                    selected_item.item_type
                )),
                Line::from(""),
            ]
        } else {
            vec![
                Line::from("No item selected.")
            ]
        };

        details
    }

    fn get_preview(&self) -> String {
        let ContentData {
            selected_item,
            preview,
            ..
        } = self.data.clone();

        if selected_item.is_none() {
            return String::from("\nNo item selected.");
        } else if "" == preview {
            return String::from("\nNo preview available.");
        }

        format!("\nPreview:\n{}", preview)
    }
}
