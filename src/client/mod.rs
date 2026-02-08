use std::io::{Result};

use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::Buffer,
    widgets::{Widget},
    layout::{Rect}
};

mod view;
use view::{
    View,
    ViewModel,
    header::{HeaderData},
    content::{ContentData},
    footer::{FooterData}
};

type EventCallback = fn(state: &mut ClientState, data: &mut ClientData) -> Result<()>;

#[derive(Debug)]
pub struct ClientState {
    pub is_running: bool,
    pub frame: usize,
}

#[derive(Debug)]
pub struct ClientData {
    pub count: isize
}

#[derive(Debug)]
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
                title: "XPLR".to_string()
            },
            content: ContentData {
                text: "Hello World".to_string(),
                count: self.data.count
            },
            footer: FooterData {
                text: "(Footer)".to_string(),
                frame: self.state.frame
            }
        }
    }
}

impl Widget for &mut Client {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        View::render(area, buffer, self.get_view_data());
    }
}
