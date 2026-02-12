use ratatui::{
    buffer:: { Buffer },
    layout::{ Constraint, Layout, Rect },
    text::{ Line, Span },
    widgets::{ Widget, Paragraph },
    style::{ Style, Color }
};

use crate::{
    types::{ Command }
};

#[derive(Clone)]
pub struct CommandInputData {
    pub current_command: Option<Command>,
    pub input: String,
}

pub struct CommandInput {
    data: CommandInputData
}

impl Widget for CommandInput {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let [input_container, confirmation_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        self.get_input()
            .render(input_container, buffer);
        self.get_confirmation()
            .render(confirmation_container, buffer);
    }
}

impl CommandInput {
    pub fn new(data: CommandInputData) -> Self {
        Self {
            data
        }
    }

    fn get_input(&self) -> Paragraph<'_> {
        Paragraph::new(Line::from(vec![
            self.input_label(),
            self.input_text(),
        ]))
    }

    fn get_confirmation(&self) -> Paragraph<'_> {
        Paragraph::new("Press [Enter] to execute")
    }

    fn input_text(&self) -> Span<'_> {
        Span::styled(self.data.input.clone(), Style::new().fg(Color::Green))
    }

    fn input_label(&self) -> Span<'_> {
        let text = match self.data.current_command.clone() {
            Some(Command::Copy) => String::from("Enter path: "),
            Some(Command::Move) => String::from("Enter path: "),
            Some(Command::Delete) => String::from("Type 'yes' to confirm: "),

            _ => String::new(),
        };

        Span::from(text)
    }
}
