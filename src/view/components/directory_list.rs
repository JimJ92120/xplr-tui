use ratatui::{
    buffer::Buffer,
    layout::{ Rect },
    widgets::{
        Widget,
        Paragraph,
    },
    text::{ Line },
    style::{
        Color,
        Stylize
    }
};

use crate::types::{
    Directory,
};

pub struct DirectoryContentData {
    pub directory: Directory,
    pub selected_item_index: usize,
}

pub struct DirectoryContent {
    data: DirectoryContentData,
}

impl Widget for DirectoryContent {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if self.data.directory.content.is_empty() {
            self.render_no_list(area, buffer);
        } else {
            self.render_list(area, buffer);                
        }
    }
}

impl DirectoryContent {
    pub fn new(data: DirectoryContentData) -> Self {
        Self {
            data
        }
    }

    fn render_no_list(self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new("No item found.")
            .render(area, buffer);
    }

    fn render_list(self, area: Rect, buffer: &mut Buffer) {
        let DirectoryContentData {
            selected_item_index,
            directory
        } = self.data;
        let scroll = if (selected_item_index as u16) < area.height {
            0
        } else {
            (selected_item_index as u16) - area.height + 1
        };

        Paragraph::new(
            directory.content
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let line = Line::from(format!("{}.{}", index, item.path_name));

                    if selected_item_index == index {
                        return line.bg(Color::Green);
                    }

                    line
                })
                .collect::<Vec<Line>>()
        )
            .scroll((scroll, 0))
            .render(area, buffer);
    }
}
