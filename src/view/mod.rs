use std::io::{Result};

use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::Buffer,
    widgets::{Widget},
    layout::{
        Constraint,
        Layout,
        Rect
    },
};

mod components;
mod header;
mod content;
mod footer;

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

type EventCallback = fn(state: &mut State) -> Result<()>;

pub struct ViewModel {
    pub header: HeaderData,
    pub content: ContentData,
    pub footer: FooterData
}

#[derive(Clone)]
pub struct State {
    pub is_running: bool,
    pub title: String,
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize,
    pub parent_directory_list: Vec<String>,
    pub text_input: String
}

pub struct View {
    state: State,
    event_callback: EventCallback
}

impl View {
    pub fn new(state: State, event_callback: EventCallback) -> Self {
        Self {
            state,
            event_callback
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal: &mut DefaultTerminal| -> Result<()> {
            self.state.is_running = true;

            while self.state.is_running {
                terminal.draw(|frame| self.render(frame))?;

                (self.event_callback)(&mut self.state)?;
            };

            Ok(())
        })?;

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(
            self,
            frame.area()
        );
    }

    fn get_view_data(&self) -> ViewModel {
        let State {
            title,
            directory_name,
            directory_content,
            selected_item_index,
            parent_directory_list,
            text_input,
            ..
        } = self.state.clone();

        ViewModel {
            header: HeaderData {
                title,
            },
            content: ContentData {
                directory_name,
                directory_content,
                selected_item_index,
                parent_directory_list
            },
            footer: FooterData {
                text_input,
            }
        }
    }
}

impl Widget for &mut View {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let [header_container, container_container, footer_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ]).areas(area);

        let ViewModel { header, content, footer } = self.get_view_data();
        Header::render(header_container, buffer, header);
        Content::render(container_container, buffer, content);
        Footer::render(footer_container, buffer, footer);
    }
}
