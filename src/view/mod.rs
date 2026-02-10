use std::io::{ Result };
use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::Buffer,
    widgets::{ Widget },
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

mod components;
mod layout;

use crate::{ State };
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

struct ViewModel {
    pub header: HeaderData,
    pub content: ContentData,
    pub footer: FooterData
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
            self.state.start();

            while self.state.is_running() {
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
        let View {
            state,
            ..
        } = self;

        ViewModel {
            header: HeaderData {
                title: state.title(),
            },
            content: ContentData {
                directory_name: state.directory_name(),
                directory_content: state.directory_content(),
                selected_item_index: state.selected_item_index(),
                selected_item: state.selected_item(),
                parent_directory_list: state.parent_directory_list(),
                preview: state.preview(),
            },
            footer: FooterData {
                text_input: state.text_input(),
            }
        }
    }

    fn event_callback(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            let View {
                state,
                ..
            } = self;

            if KeyEventKind::Press == key.kind {
                match key.code {
                    KeyCode::Esc => state.stop(),

                    KeyCode::Up => state.select_previous_item(),
                    KeyCode::Down => state.select_next_item(),
                    KeyCode::PageUp => state.select_first_item(),
                    KeyCode::PageDown => state.select_last_item(),

                    KeyCode::Right => state.load_next_directory(),
                    KeyCode::Left => state.load_previous_directory(),

                    KeyCode::Char(char) => state.type_text(char),
                    KeyCode::Backspace => state.delete_text_last_char(),
                
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
            Constraint::Length(2),
        ]).areas(area);
        let ViewModel { header, content, footer } = self.get_view_data();
        
        Header::new(header)
            .render(header_container, buffer);
        Content::new(content)
            .render(container_container, buffer);
        Footer::new(footer)
            .render(footer_container, buffer);
    }
}
