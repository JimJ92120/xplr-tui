use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Paragraph }
};

pub struct HeaderData {
    pub title: String,
}

pub struct Header {
    data: HeaderData,
}

impl Widget for Header {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        self.render_title(area, buffer);
    }
}

impl Header {
    pub fn new(data: HeaderData) -> Self {
        Self {
            data
        }
    }

    fn render_title(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(format!("{}", self.data.title))
            .render(area, buffer);
    }
}
