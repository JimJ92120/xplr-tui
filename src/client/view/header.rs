use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

#[derive(Debug)]
pub struct HeaderData {
    pub title: String
}

#[derive(Debug)]
pub struct Header {}

impl Header {
    pub fn render(area: Rect, buffer: &mut Buffer, data: HeaderData) {
        Paragraph::new(data.title)
            .render(area, buffer);
    }
}
