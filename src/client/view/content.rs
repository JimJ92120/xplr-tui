use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{
        Widget,
        Block
    },
};

use super::components::list::{ListComponent, ListData};

#[derive(Debug)]
pub struct ContentData {
    pub directory_name: String,
    pub directory_content: Vec<String>,
    pub selected_item_index: usize
}

#[derive(Debug)]
pub struct Content {}

impl Content {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [title_container, list_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]).areas(area);

        Content::render_title(
            title_container,
            buffer,
            format!("Reading {}", data.directory_name.clone())
        );
        Content::render_list(
            list_container,
            buffer,
            ListData {
                list: data.directory_content.clone(),
                selected_item_index: data.selected_item_index
            }
        );
    }

    fn render_title(area: Rect, buffer: &mut Buffer, title: String) {
        Block::new()
            .title(title)
            .render(area, buffer);
    }

    fn render_list(area: Rect, buffer: &mut Buffer, data: ListData) {
        ListComponent::render(area, buffer, data);
    }
}
