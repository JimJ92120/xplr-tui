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
    Command
};

use crate::components::{
    command_list::{
        CommandList,
        CommandListData,
    },
    command_input::{
        CommandInput,
        CommandInputData,
    },
    command_prompt::{
        CommandPrompt,
        CommandPromptData,
    }
};

#[derive(Clone)]
pub struct FooterData {
    pub current_command: Option<Command>,
    pub input: String,
    pub prompt: String,
}

pub struct Footer {
    data: FooterData
}

impl Widget for Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let FooterData {
            current_command,
            input,
            prompt,
            ..
        } = self.data.clone();
        let [command_list_container, command_content_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2),
        ]).areas(area);

        CommandList::new(CommandListData {
            current_command: current_command.clone(),
        })
            .render(command_list_container, buffer);

        if self.data.current_command.is_none() {
            CommandPrompt::new(CommandPromptData {
                prompt: prompt.clone(),
            })
                .render(command_content_container, buffer);
        } else {
            CommandInput::new(CommandInputData {
                current_command: current_command.clone(),
                input: input.clone(),
            })
            .render(command_content_container, buffer);
        }

    }
}

impl Footer {
    pub fn new(data: FooterData) -> Self {
        Self {
            data
        }
    }
}
