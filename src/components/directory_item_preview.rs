use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Paragraph }
};

use crate::{
    types::{ DirectoryItem, DirectoryItemType },
};

use super::{
    base::{ 
        box_container::{ BoxContainer }
    }
};

#[derive(Clone)]
pub struct DirectoryItemPreviewData {
    pub selected_item: Option<DirectoryItem>,
    pub preview: String,
}

pub struct DirectoryItemPreview {
    data: DirectoryItemPreviewData,
}

impl Widget for DirectoryItemPreview {
    fn render(self, area: Rect, buffer: &mut Buffer) {       
        let content = if self.data.selected_item.is_none()
            ||  "" == self.data.preview
        {
            "No preview available."
        } else {
            &self.data.preview
        };

        Paragraph::new(content)
            .block(BoxContainer::new(String::from("Preview")))
            .render(area, buffer);
    }
}

impl DirectoryItemPreview {
    pub fn new(data: DirectoryItemPreviewData) -> Self {
        Self {
            data
        }
    }
}
