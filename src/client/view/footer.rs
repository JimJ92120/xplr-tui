use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    widgets::{
        Widget,
        Paragraph
    }
};

pub struct FooterData {
    pub text_input: String
}

pub struct Footer {}

impl Footer {
    pub fn render(area: Rect, buffer: &mut Buffer, data: FooterData) {
        Paragraph::new(format!("Input: {}", data.text_input))
            .render(area, buffer);
    }
}
