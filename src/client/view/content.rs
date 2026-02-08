use ratatui::{
    buffer::Buffer,
    layout::{ Rect},
    widgets::{Widget, Paragraph}
};

#[derive(Debug)]
pub struct ContentData {
    pub directory_name: String,
    pub directory_content: Vec<String>
}

#[derive(Debug)]
pub struct Content {}

impl Content {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ContentData) {
        if 0 == data.directory_content.len() {
            Paragraph::new(format!("Reading {}\n\nNo item found.", data.directory_name))
                .render(area, buffer);
        } else {
            Paragraph::new(format!(
                "Reading {}\n\n- {}",
                data.directory_name,
                data.directory_content.join("\n- ")
            ))
                .render(area, buffer);
        }
    }
}
