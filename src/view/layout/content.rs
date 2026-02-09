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

pub struct Content {}

impl Content {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [left_container, right_container] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        Content::render_left_container(left_container, buffer, data.clone());
        Content::render_right_container(right_container, buffer, data.clone());
    }

    fn render_left_container(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [title_container, list_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]).areas(area);

        let ContentData {
            directory_content,
            selected_item_index,
            ..
        } = data.clone();
        Block::new()
            .title(Self::get_directory_list(data.clone()))
            .render(title_container, buffer);
        List::render(
            list_container,
            buffer,
            ListData {
                list: directory_content,
                selected_item_index
            }
        );
    }

    fn render_right_container(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [details_container, preview_container] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).areas(area);

        Paragraph::new(Self::get_details(data.clone()))
            .render(details_container, buffer);
        Paragraph::new(Self::get_preview(data.clone()))
            .render(preview_container, buffer);
    }

    fn get_directory_list(data: ContentData) -> String {
        let ContentData {
            directory_name,
            parent_directory_list,
            ..
        } = data;

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

    fn get_details(data: ContentData) -> Vec<Line<'static>> {
        let ContentData {
            selected_item,
            ..
        } = data;

        let details: Vec<Line> = if !selected_item.is_none() {
            let selected_item = selected_item.unwrap();

            vec![
                Line::from("Item:"),
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

    fn get_preview(data: ContentData) -> String {
        let ContentData {
            selected_item,
            preview,
            ..
        } = data.clone();

        if selected_item.is_none() {
            return String::from("\nNo item selected.");
        } else if "" == preview {
            return String::from("\nNo preview available.");
        }

        format!("\nPreview:\n{}", data.preview)
    }
}
