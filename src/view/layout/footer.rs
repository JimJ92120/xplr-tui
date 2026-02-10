use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    widgets::{
        Widget,
        Paragraph
    }
};

use crate::types::{
    Action
};

use crate::components::{
    action_list::{
        ActionList,
        ActionListData,
    }
};

pub struct FooterData {
    pub current_action: Option<Action>,
    pub text_input: String
}

pub struct Footer {
    data: FooterData
}

impl Widget for Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let [actions_container, input_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        self.render_actions_container(actions_container, buffer);
        self.render_input_container(input_container, buffer);
    }
}

impl Footer {
    pub fn new(data: FooterData) -> Self {
        Self {
            data
        }
    }

    fn render_actions_container(&self, area: Rect, buffer: &mut Buffer) {
        ActionList::new(ActionListData {
            current_action: self.data.current_action.clone(),
        })
            .render(area, buffer);
    }

    fn render_input_container(&self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new(format!("Input: {}", self.data.text_input))
            .render(area, buffer);
    }
}
