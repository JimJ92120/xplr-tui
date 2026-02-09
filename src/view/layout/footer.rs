use ratatui::{
    buffer::Buffer,
    layout::{ Rect },
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
        Paragraph::new(Self::get_footer(data))
            .render(area, buffer);
    }

    fn get_footer(data: FooterData) -> String {
        format!("Input: {}", data.text_input)
    }
}
