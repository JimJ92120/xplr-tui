use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Paragraph }
};

use crate::{
    types::{ DirectoryItem, DirectoryItemType }
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
        } else if "" == self.data.preview {
            self.render_no_preview(area, buffer);
        } else {
            self.render_preview(area, buffer);
        }
    }
}

impl DirectoryItemPreview {
    pub fn new(data: DirectoryItemPreviewData) -> Self {
        Self {
            data
        }
    }

    fn render_no_preview(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new("\nNo preview available.")
            .render(area, buffer);
    }

    fn render_preview(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(format!("\nPreview:\n{}", self.data.preview))
            .render(area, buffer);
    }
}
