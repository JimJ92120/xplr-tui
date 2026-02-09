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
    crossterm::event::{
        self,
        Event,
        KeyCode,
        KeyEventKind
    }
};

use crate::Controller;

mod components;
mod layout;

use layout::{
    header::{
        Header,
        HeaderData
    },
    content::{
        Content,
        ContentData
    },
    footer::{
        Footer,
        FooterData
    }
};

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
}

impl View {
    pub fn new(state: State) -> Self {
        Self {
            state
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal: &mut DefaultTerminal| -> Result<()> {
            self.state.is_running = true;

            while self.state.is_running {
                terminal.draw(|frame| self.render(frame))?;

                self.event_callback()?;
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

    fn event_callback(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if KeyEventKind::Press == key.kind {
                match key.code {
                    KeyCode::Esc => Controller::stop(&mut self.state),

                    KeyCode::Up => Controller::select_previous_item(&mut self.state),
                    KeyCode::Down => Controller::select_next_item(&mut self.state),

                    KeyCode::Right => Controller::load_next_directory(&mut self.state),
                    KeyCode::Left => Controller::load_previous_directory(&mut self.state),

                    KeyCode::Char(char) => Controller::type_text(&mut self.state, char),
                
                    _ => {}
                };
            } 
        };

        Ok(())
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
