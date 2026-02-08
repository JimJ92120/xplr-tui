use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

#[derive(Debug)]
pub struct FooterData {
    pub text: String,
    pub frame: usize
}

#[derive(Debug)]
pub struct Footer {}

impl Footer {
    pub fn render(area: Rect, buffer: &mut Buffer, data: FooterData) {
        Paragraph::new(format!("{} | frame: {}", data.text, data.frame))
            .render(area, buffer);
    }
}
