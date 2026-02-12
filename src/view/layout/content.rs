use ratatui::{
    buffer::{ Buffer },
    layout::{ Constraint, Layout, Rect },
    widgets::{ Widget, Paragraph }
};

use crate::{
    types::{ DirectoryItem, Directory, DirectoryList },
    components::{
        directory_content::{ DirectoryContent, DirectoryContentData },
        directory_item_details::{ DirectoryItemDetails, DirectoryItemDetailsData },
        directory_item_preview::{ DirectoryItemPreview, DirectoryItemPreviewData }
    }
};

#[derive(Clone)]
pub struct ContentData {
    pub current_directory: Directory,
    pub selected_item_index: usize,
    pub selected_item: Option<DirectoryItem>,
    pub parent_directory_list: DirectoryList,
    pub preview: String,
}

pub struct Content {
    data: ContentData,
}

impl Widget for Content {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let [left_container, right_container] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        self.render_left_container(left_container, buffer);
        self.render_right_container(right_container, buffer);
    }
}

impl Content {
    pub fn new(data: ContentData) -> Self {
        Self {
            data
        }
    }

    fn render_left_container(&self, area: Rect, buffer: &mut Buffer) {
        let ContentData {
            selected_item_index,
            current_directory,
            parent_directory_list,
            ..
        } = self.data.clone();

        DirectoryContent::new(DirectoryContentData {
            selected_item_index: selected_item_index.clone(),
            current_directory: current_directory.clone(),
            parent_directory_list: parent_directory_list.clone(),
        })
            .render(area, buffer);
    }

    fn render_right_container(&self, area: Rect, buffer: &mut Buffer) {
        let ContentData {
            selected_item,
            ..
        } = self.data.clone();

        match selected_item {
            Some(selected_item) => {
                let ContentData {
                    preview,
                    ..
                } = self.data.clone();
                let [details_container, preview_container] = Layout::vertical([
                    Constraint::Length(5),
                    Constraint::Fill(1),
                ]).areas(area);

                DirectoryItemDetails::new(DirectoryItemDetailsData {
                    selected_item: selected_item.clone()
                })
                    .render(details_container, buffer);
                DirectoryItemPreview::new(DirectoryItemPreviewData {
                    preview: preview.clone(),
                    selected_item: selected_item.clone(),
                })
                    .render(preview_container, buffer);
            },
            None => {
                Paragraph::new("No item selected.")
                    .render(area, buffer);
            }
        }
    }
}
