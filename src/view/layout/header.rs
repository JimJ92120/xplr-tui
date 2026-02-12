use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Paragraph }
};

use crate::{
    components::{
        base
    }
};

pub struct HeaderData {
    pub title: String,
}

pub struct Header {
    data: HeaderData,
}

impl Widget for Header {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let container = base::box_container(self.data.title);

        Paragraph::new("Use ▲ ▼ to select an item and ◄ ► to navigate directories")
            .block(container)
            .render(area, buffer);
    }
}

impl Header {
    pub fn new(data: HeaderData) -> Self {
        Self {
            data
        }
    }
}
