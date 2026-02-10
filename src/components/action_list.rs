use ratatui::{
    buffer::Buffer,
    layout::{
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
    }
};

use crate::types::{
    Action
};

#[derive(Clone)]
pub struct ActionListData {
    pub current_action: Option<Action>
}

pub struct ActionList {
    data: ActionListData
}

impl Widget for ActionList {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        self.render_action_list(area, buffer);
    }
}

impl ActionList {
    pub fn new(data: ActionListData) -> Self {
        Self {
            data
        }
    }

    fn render_action_list(&self, area: Rect, buffer: &mut Buffer) {
        let action_list: Vec<(Action, String)> = vec![
            (Action::Copy, String::from("1: copy | ")),
            (Action::Move, String::from("2: move | ")),
            (Action::Rename, String::from("3: rename | ")),
            (Action::Delete, String::from("4: delete")),
        ];

        Paragraph::new(
            Line::from(
                action_list
                    .iter()
                    .map(|(action, text)| self.get_list_item(action.clone(), text.clone()))
                    .collect::<Vec<_>>()
            )
        )
            .render(area, buffer);
    }

    fn get_list_item(&self, action: Action, text: String) -> Span<'_> {
        let ActionListData {
            current_action,
            ..
        } = self.data.clone();

        if !current_action.is_none()
            && current_action.unwrap() == action
        {
            return Span::styled(text, Style::new().green());
        }

        Span::from(text)
    }
}
