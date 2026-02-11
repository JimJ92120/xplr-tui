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
        KeyEventKind,
        KeyModifiers,
    }
};

mod layout;

use crate::{
    State,
    Store,
    types::{
        Command
    }
};

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
    store: Store,
}

impl View {
    pub fn new(state: State, store: Store) -> Self {
        Self {
            state,
            store,
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
            store,
            ..
        } = self;

        ViewModel {
            header: HeaderData {
                title: state.title(),
            },
            content: ContentData {
                directory: state.directory(),
                selected_item_index: state.selected_item_index(),
                selected_item: state.selected_item(),
                parent_directory_list: state.parent_directory_list(),
                preview: state.preview(),
            },
            footer: FooterData {
                current_command: store.get("command", "current_command")
                    .downcast_ref::<Option<Command>>()
                    .unwrap()
                    .clone(),
                input: store.get("command", "input").downcast_ref::<String>().unwrap().clone(),
            }
        }
    }

    fn event_callback(&mut self) -> Result<()> {
        if let Event::Key(key_event) = event::read()? {
            let View {
                state,
                store,
                ..
            } = self;

            if KeyEventKind::Press == key_event.kind {
                if key_event.modifiers.contains(KeyModifiers::ALT) {
                    match key_event.code {
                        KeyCode::Char('1') => store.dispatch("command", "run_command", Box::new(Command::Copy)),
                        KeyCode::Char('2') => store.dispatch("command", "run_command", Box::new(Command::Move)),
                        KeyCode::Char('3') => store.dispatch("command", "run_command", Box::new(Command::Rename)),
                        KeyCode::Char('4') => store.dispatch("command", "run_command", Box::new(Command::Delete)),
                    
                        _ => {}
                    };
                } else {
                    match key_event.code {
                        KeyCode::Esc => state.stop(),

                        KeyCode::Up => state.select_previous_item(),
                        KeyCode::Down => state.select_next_item(),
                        KeyCode::PageUp => state.select_first_item(),
                        KeyCode::PageDown => state.select_last_item(),

                        KeyCode::Right => state.load_next_directory(),
                        KeyCode::Left => state.load_previous_directory(),

                        KeyCode::Char(char) => store.dispatch("command", "type_input", Box::new(char)),
                        KeyCode::Backspace => store.dispatch("command", "delete_input_last_char", Box::new(())),
                    
                        _ => {}
                    };
                }
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
            Constraint::Length(3),
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
