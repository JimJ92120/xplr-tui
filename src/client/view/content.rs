use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    widgets::{
        Widget,
        Block,
        Paragraph
    },
};

use super::components::list::{ListComponent, ListData};

#[derive(Clone)]
pub struct ContentData {
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize
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

        Content::render_title(
            title_container,
            buffer,
            format!("> {}", data.directory_name.clone())
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

    fn render_right_container(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [title_container, details_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]).areas(area);

        Content::render_title(
            title_container,
            buffer,
            String::from("Details")
        );
        Paragraph::new(
            format!(
                "name: {}\ntype: {}",
                data.directory_content[data.selected_item_index].0,
                data.directory_content[data.selected_item_index].1
            )
        )
            .render(details_container, buffer);
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
