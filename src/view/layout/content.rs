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
    pub parent_directory_list: Vec<String>
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
        let [title_container, details_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1)
        ]).areas(area);

        Block::new()
            .title(String::from("Details"))
            .render(title_container, buffer);
        Paragraph::new(Self::get_details(data))
            .render(details_container, buffer);
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
            selected_item_index,
            parent_directory_list,
            directory_content,
            ..
        } = data;

        let mut details: Vec<Line> = if !directory_content.is_empty() {
            vec![
                Line::from("Item:"),
                Line::from(format!(
                    "- name: {}",
                    directory_content[selected_item_index].0
                )),
                Line::from(format!(
                    "- type: {}",
                    directory_content[selected_item_index].1
                )),
                Line::from(""),
            ]
        } else {
            vec![
                Line::from("No content found.")
            ]
        };

        if parent_directory_list.is_empty() {
            details.push(Line::from("No parents found."));
        } else {
            parent_directory_list
                .iter()
                .for_each(|item| {
                    details.push(Line::from(
                        format!("- {}", item)
                    ));
                })
        }

        details
    }
}
