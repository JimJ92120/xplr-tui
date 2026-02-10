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
        Block,
        Paragraph
    },
};

use super::super::components::{
    list::{ List, ListData }
};

#[derive(Clone)]
pub struct ContentData {
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize,
    pub selected_item: Option<(String, String)>,
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

        Block::new()
            .title(self.get_directory_list())
            .render(title_container, buffer);
        List::new(ListData {
            list: self.data.directory_content.clone(),
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

                if "file" == selected_item.1 {
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

    fn get_directory_list(&self) -> String {
        let ContentData {
            directory_name,
            parent_directory_list,
            ..
        } = self.data.clone();

        if parent_directory_list.is_empty() {
            directory_name
        } else {
            let mut directory_list = parent_directory_list;
            directory_list.push(directory_name);

            directory_list
                .iter()
                .enumerate()
                .map(|(index, directory_name)| {
                    if 0 < index {
                        let (_, formatted_directory_name) = directory_name
                            .split_once(
                                format!("{}/", directory_list[index - 1].clone()).as_str()
                            )
                            .unwrap();

                        format!(" > {}", formatted_directory_name).to_string()
                    } else {
                        directory_name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
                .to_string()
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
                    selected_item.0
                )),
                Line::from(format!(
                    "- type: {}",
                    selected_item.1
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
