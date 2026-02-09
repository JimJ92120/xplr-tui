use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
};

mod components;
pub mod header;
pub mod content;
pub mod footer;

use header::{
    Header,
    HeaderData
};
use content::{
    Content,
    ContentData
};
use footer::{
    Footer,
    FooterData
};

pub struct ViewModel {
    pub header: HeaderData,
    pub content: ContentData,
    pub footer: FooterData
}

pub struct View {}

impl View {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ViewModel) {
        let [header_container, container_container, footer_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        Header::render(header_container, buffer, data.header);
        Content::render(container_container, buffer, data.content);
        Footer::render(footer_container, buffer, data.footer);
    }
}
