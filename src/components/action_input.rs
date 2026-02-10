use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    text::{
        Span,
        Line,
    },
    widgets::{
        Widget,
        Paragraph,
    },
    style::{
        Style,
        Color,
    }
};

use crate::types::{
    Action
};

#[derive(Clone)]
pub struct ActionInputData {
    pub current_action: Option<Action>,
    pub text_input: String,
}

pub struct ActionInput {
    data: ActionInputData
}

impl Widget for ActionInput {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if self.data.current_action.is_none() {
            return;
        }

        let [input_container, confirmation_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        self.render_action_input(input_container, buffer);
        self.render_confirmation_text(confirmation_container, buffer);
    }
}

impl ActionInput {
    pub fn new(data: ActionInputData) -> Self {
        Self {
            data
        }
    }

    fn render_action_input(&self, area: Rect, buffer: &mut Buffer) {
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
        Span::styled(self.data.text_input.clone(), Style::new().fg(Color::Green))
    }

    fn get_input_label(&self) -> Span<'_> {
        let text = match self.data.current_action.clone().unwrap() {
            Action::Copy => String::from("Copy to: "),
            Action::Move => String::from("Move to: "),
            Action::Rename => String::from("New name: "),
            Action::Delete => String::from("Confirm: "),
        };

        Span::from(text)
    }
}
