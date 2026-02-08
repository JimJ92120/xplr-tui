use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

pub mod header;
pub mod content;
pub mod footer;

use header::{Header, HeaderData};
use content::{Content, ContentData};
use footer::{Footer, FooterData};

#[derive(Debug)]
pub struct ViewModel {
    pub header: HeaderData,
    pub content: ContentData,
    pub footer: FooterData
}

#[derive(Debug)]
pub struct View {}

impl View {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ViewModel) {
        let [header, main, footer] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        Header::render(header, buffer, data.header);
        Content::render(main, buffer, data.content);
        Footer::render(footer, buffer, data.footer);
    }
}
