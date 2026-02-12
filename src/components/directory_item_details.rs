use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    text::{ Line },
    widgets::{ Widget, Paragraph, Block, BorderType }
};

use crate::{
    types::{ DirectoryItem }
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
        let container = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Details");

        Paragraph::new(self.get_details())
            .block(container)
            .render(area, buffer);
    }
    
    fn get_details(&self) -> Vec<Line<'static>> {
        let DirectoryItemDetailsData {
            selected_item,
            ..
        } = self.data.clone();

        vec![
            Line::from(format!("path: {}", selected_item.path_name)),
            Line::from(format!("name: {}", selected_item.name)),
            Line::from(format!("type: {:?}", selected_item.item_type)),
        ]
    }
}
