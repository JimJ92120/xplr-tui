use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    widgets::{
        Widget,
    }
};

use crate::types::{
    Action
};

use crate::components::{
    action_list::{
        ActionList,
        ActionListData,
    },
    action_input::{
        ActionInput,
        ActionInputData,
    }
};

#[derive(Clone)]
pub struct FooterData {
    pub current_action: Option<Action>,
    pub text_input: String
}

pub struct Footer {
    data: FooterData
}

impl Widget for Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let FooterData {
            current_action,
            text_input,
        } = self.data.clone();
        let [action_list_container, action_input_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2),
        ]).areas(area);

        ActionList::new(ActionListData {
            current_action: current_action.clone(),
        })
            .render(action_list_container, buffer);
        ActionInput::new(ActionInputData {
            current_action: current_action.clone(),
            text_input: text_input.clone(),
        })
            .render(action_input_container, buffer);
    }
}

impl Footer {
    pub fn new(data: FooterData) -> Self {
        Self {
            data
        }
    }
}
