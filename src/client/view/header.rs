use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

#[derive(Debug)]
pub struct HeaderData {
    pub title: String,
    pub count: isize,
    pub frame: usize
}

#[derive(Debug)]
pub struct Header {}

impl Header {
    pub fn render(area: Rect, buffer: &mut Buffer, data: HeaderData) {
        Paragraph::new(format!("{} | count: {} | frame: {}", data.title, data.count, data.frame))
            .render(area, buffer);
    }
}
