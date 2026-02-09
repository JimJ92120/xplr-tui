use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

pub struct HeaderData {
    pub title: String,
    pub frame: usize,
    pub selected_item_index: usize
}

pub struct Header {}

impl Header {
    pub fn render(area: Rect, buffer: &mut Buffer, data: HeaderData) {
        Paragraph::new(format!("{} | frame: {} | index: {}", data.title, data.frame, data.selected_item_index))
            .render(area, buffer);
    }
}
