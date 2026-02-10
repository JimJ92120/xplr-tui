use ratatui::{
    buffer::Buffer,
    layout::{ Rect },
    widgets::{
        Widget,
        Paragraph
    }
};

pub struct HeaderData {
    pub title: String,
}

pub struct Header {}

impl Header {
    pub fn render(area: Rect, buffer: &mut Buffer, data: HeaderData) {
        Paragraph::new(format!("{}", data.title))
            .render(area, buffer);
    }
}
