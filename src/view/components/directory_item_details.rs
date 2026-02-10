use ratatui::{
    buffer::Buffer,
    layout::{
        Rect
    },
    text::{
        Line
    },
    widgets::{
        Widget,
        Paragraph
    },
};

use crate::types::{
    DirectoryItem,
};

#[derive(Clone)]
pub struct DirectoryItemDetailsData {
    pub selected_item: DirectoryItem,
}

pub struct DirectoryItemDetails {
    data: DirectoryItemDetailsData,
}

impl Widget for DirectoryItemDetails {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        self.render_details(area, buffer)
    }
}

impl DirectoryItemDetails {
    pub fn new(data: DirectoryItemDetailsData) -> Self {
        Self {
            data
        }
    }

    fn render_details(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(self.get_details())
            .render(area, buffer);
    }
    
    fn get_details(&self) -> Vec<Line<'static>> {
        let DirectoryItemDetailsData {
            selected_item,
            ..
        } = self.data.clone();

        vec![
            Line::from("Details:"),
            Line::from(format!(
                "- name: {}",
                selected_item.name
            )),
            Line::from(format!(
                "- type: {:?}",
                selected_item.item_type
            )),
            Line::from(""),
        ]
    }
}
