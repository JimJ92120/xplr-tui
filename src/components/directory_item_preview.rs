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
    pub selected_item: DirectoryItem,
    pub preview: String,
}

pub struct DirectoryItemPreview {
    data: DirectoryItemPreviewData,
}

impl Widget for DirectoryItemPreview {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if DirectoryItemType::File != self.data.selected_item.item_type {
            return;
        }
        
        let preview = if "" == self.data.preview {
            self.get_no_preview()
        } else {
            self.get_preview()
        };

        preview
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

    fn get_no_preview(&self) -> Paragraph<'_> {
        Paragraph::new("No preview available.")
    }

    fn get_preview(&self) -> Paragraph<'_> {
        Paragraph::new(self.data.preview.clone())
    }
}
