use ratatui::{
    buffer:: { Buffer },
    layout::{ Rect },
    text::{ Span, Line },
    widgets::{ Widget, Paragraph },
    style::{ Style, Color }
};

use crate::{
    types::{ Command }
};

#[derive(Clone)]
pub struct CommandListData {
    pub current_command: Option<Command>
}

pub struct CommandList {
    data: CommandListData
}

impl Widget for CommandList {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        self.render_command_list(area, buffer);
    }
}

impl CommandList {
    pub fn new(data: CommandListData) -> Self {
        Self {
            data
        }
    }

    fn render_command_list(&self, area: Rect, buffer: &mut Buffer) {
        let command_list: Vec<(Command, String)> = vec![
            (Command::Copy, String::from("[ ALT+1: copy ]")),
            (Command::Move, String::from("[ ALT+2: move ]")),
            (Command::Delete, String::from("[ ALT+3: delete ]")),
        ];

        Paragraph::new(Line::from(
            command_list
                .iter()
                .map(|(command, text)| self.get_list_item(command.clone(), text.clone()))
                .collect::<Vec<_>>()
        ))
            .render(area, buffer);
    }

    fn get_list_item(&self, command: Command, text: String) -> Span<'_> {
        let CommandListData {
            current_command,
            ..
        } = self.data.clone();

        if !current_command.is_none()
            && current_command.unwrap() == command
        {
            return Span::styled(text, Style::new().fg(Color::Green));
        }

        Span::from(text)
    }
}
