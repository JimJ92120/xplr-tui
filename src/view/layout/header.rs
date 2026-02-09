use ratatui::{
    buffer::Buffer,
    layout::{Rect},
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
        Paragraph::new(Self::get_header(data))
            .render(area, buffer);
    }

    fn get_header(data: HeaderData) -> String {
        format!("{}", data.title)
    }
}
