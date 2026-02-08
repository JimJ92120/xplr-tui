use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

#[derive(Debug)]
pub struct ContentData {
    pub text: String,
    pub count: isize,
}

#[derive(Debug)]
pub struct Content {}

impl Content {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ContentData) {
        Paragraph::new(format!("{}\ncount: {}", data.text, data.count))
            .render(area, buffer);
    }
}
