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

        self.render_command_input(input_container, buffer);
        self.render_confirmation_text(confirmation_container, buffer);
    }
}

impl CommandInput {
    pub fn new(data: CommandInputData) -> Self {
        Self {
            data
        }
    }

    fn render_command_input(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(Line::from(vec![
            self.get_input_label(),
            self.get_input_text(),
        ]))
            .render(area, buffer);
    }

    fn render_confirmation_text(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new("Press [Enter] to confirm")
            .render(area, buffer);
    }

    fn get_input_text(&self) -> Span<'_> {
        Span::styled(self.data.input.clone(), Style::new().fg(Color::Green))
    }

    fn get_input_label(&self) -> Span<'_> {
        let text = match self.data.current_command.clone() {
            Some(Command::Copy) => String::from("Copy to: "),
            Some(Command::Move) => String::from("Move to: "),
            Some(Command::Delete) => String::from("Confirm: "),

            _ => String::new(),
        };

        Span::from(text)
    }
}
