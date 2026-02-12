use std::{
    io::{ Result }
};
use ratatui::{
    DefaultTerminal,
    Frame,
    buffer::{ Buffer },
    widgets::{ Widget },
    layout::{ Constraint, Layout, Rect },
    crossterm::{
        event::{
            self,
            Event,
            KeyCode,
            KeyEventKind,
            KeyModifiers,
        }
    }
};

mod layout;

use crate::{
    store::{ Store, StoreType },
    types::{ Command, Directory, DirectoryItem, DirectoryList }
};

use layout::{
    header::{ Header, HeaderData },
    content::{ Content, ContentData },
    footer::{ Footer, FooterData }
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
            self.store.action(StoreType::Client, "start");

            while self.store.get::<bool>(StoreType::Client, "is_running")
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

    fn data(&self) -> ViewModel {
        let View {
            store,
            ..
        } = self;

        ViewModel {
            header: HeaderData {
                title: String::from("XPLR"),
            },
            content: ContentData {
                directory: store.get::<Directory>(StoreType::Directory, "directory"),
                selected_item_index: store.get::<usize>(StoreType::Directory, "selected_item_index"),
                selected_item: store.get::<Option<DirectoryItem>>(StoreType::Directory, "selected_item"),
                parent_directory_list: store.get::<DirectoryList>(StoreType::Directory, "parent_directory_list"),
                preview: store.get::<String>(StoreType::Directory, "preview"),
            },
            footer: FooterData {
                current_command: store.get::<Option<Command>>(StoreType::Command, "current_command"),
                input: store.get::<String>(StoreType::Command, "input"),
                prompt: store.get::<String>(StoreType::Command, "prompt"),
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
                if !store.get::<String>(StoreType::Command, "prompt").is_empty() {
                    store.action(StoreType::Command, "clear_prompt");
                }                

                if key_event.modifiers.contains(KeyModifiers::ALT) {
                    match key_event.code {
                        KeyCode::Char('1') => store.dispatch(StoreType::Command, "run_command", Box::new(Command::Copy)),
                        KeyCode::Char('2') => store.dispatch(StoreType::Command, "run_command", Box::new(Command::Move)),
                        KeyCode::Char('3') => store.dispatch(StoreType::Command, "run_command", Box::new(Command::Rename)),
                        KeyCode::Char('4') => store.dispatch(StoreType::Command, "run_command", Box::new(Command::Delete)),
                    
                        _ => {}
                    };
                } else {
                    match key_event.code {
                        KeyCode::Esc => store.action(StoreType::Client, "stop"),

                        KeyCode::Up => store.action(StoreType::Directory, "select_previous_item"),
                        KeyCode::Down => store.action(StoreType::Directory, "select_next_item"),
                        KeyCode::PageUp => store.action(StoreType::Directory, "select_first_item"),
                        KeyCode::PageDown => store.action(StoreType::Directory, "select_last_item"),

                        KeyCode::Right => store.action(StoreType::Directory, "load_next_directory"),
                        KeyCode::Left => store.action(StoreType::Directory, "load_previous_directory"),

                        KeyCode::Char(char) => store.dispatch(StoreType::Command, "type_input", Box::new(char)),
                        KeyCode::Backspace => store.action(StoreType::Command, "delete_input_last_char"),

                        KeyCode::Enter => store.dispatch(
                            StoreType::Command,
                            "copy_file",
                            Box::new(
                                store
                                    .get::<Option<DirectoryItem>>(StoreType::Directory, "selected_item")
                                    .expect("No item selected to copy.")
                                    .path_name
                            )
                        ),
                    
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
        let ViewModel { header, content, footer } = self.data();
        
        Header::new(header)
            .render(header_container, buffer);
        Content::new(content)
            .render(container_container, buffer);
        Footer::new(footer)
            .render(footer_container, buffer);
    }
}
