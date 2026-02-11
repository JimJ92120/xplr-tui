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
    Store,
    types::{
        Command,
        Directory,
        DirectoryItem,
        DirectoryList,
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
    store: Store,
}

impl View {
    pub fn new(store: Store) -> Self {
        Self {
            store,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal: &mut DefaultTerminal| -> Result<()> {
            self.store.action("client", "start");

            while self.store.get::<bool>("client", "is_running")
            {
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
            store,
            ..
        } = self;

        ViewModel {
            header: HeaderData {
                title: String::from("XPLR"),
            },
            content: ContentData {
                directory: store.get::<Directory>("directory", "directory"),
                selected_item_index: store.get::<usize>("directory", "selected_item_index"),
                selected_item: store.get::<Option<DirectoryItem>>("directory", "selected_item"),
                parent_directory_list: store.get::<DirectoryList>("directory", "parent_directory_list"),
                preview: store.get::<String>("directory", "preview"),
            },
            footer: FooterData {
                current_command: store.get::<Option<Command>>("command", "current_command"),
                input: store.get::<String>("command", "input"),
            }
        }
    }

    fn event_callback(&mut self) -> Result<()> {
        if let Event::Key(key_event) = event::read()? {
            let View {
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
                        KeyCode::Esc => store.action("client", "stop"),

                        KeyCode::Up => store.action("directory", "select_previous_item"),
                        KeyCode::Down => store.action("directory", "select_next_item"),
                        KeyCode::PageUp => store.action("directory", "select_first_item"),
                        KeyCode::PageDown => store.action("directory", "select_last_item"),

                        KeyCode::Right => store.action("directory", "load_next_directory"),
                        KeyCode::Left => store.action("directory", "load_previous_directory"),

                        KeyCode::Char(char) => store.dispatch("command", "type_input", Box::new(char)),
                        KeyCode::Backspace => store.action("command", "delete_input_last_char"),
                    
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
