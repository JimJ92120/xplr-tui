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

type EventCallback = fn(state: &mut ClientState, data: &mut ClientData) -> Result<()>;

pub struct ViewModel {
    pub header: HeaderData,
    pub content: ContentData,
    pub footer: FooterData
}

pub struct ClientState {
    pub is_running: bool,
    pub frame: usize,
}

pub struct ClientData {
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize,
    pub parent_directory_list: Vec<String>,
    pub text_input: String
}

pub struct Client {
    state: ClientState,
    data: ClientData,
    event_callback: EventCallback
}

impl Client {
    pub fn new(state: ClientState, data: ClientData, event_callback: EventCallback) -> Self {
        Self {
            state,
            data,
            event_callback
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal: &mut DefaultTerminal| -> Result<()> {
            self.state.is_running = true;

            while self.state.is_running {
                self.state.frame += 1;

                terminal.draw(|frame| self.render(frame))?;

                (self.event_callback)(&mut self.state, &mut self.data)?;
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
        ViewModel {
            header: HeaderData {
                title: String::from("XPLR"),
                frame: self.state.frame,
                selected_item_index: self.data.selected_item_index
            },
            content: ContentData {
                directory_name: self.data.directory_name.clone(),
                directory_content: self.data.directory_content.clone(),
                selected_item_index: self.data.selected_item_index,
                parent_directory_list: self.data.parent_directory_list.clone()
            },
            footer: FooterData {
                text_input: self.data.text_input.clone(),
            }
        }
    }
}

impl Widget for &mut Client {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let data = self.get_view_data();
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
