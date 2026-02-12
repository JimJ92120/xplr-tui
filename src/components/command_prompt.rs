use ratatui::{
    buffer::Buffer,
    layout::{
        Rect
    },
    widgets::{
        Widget,
        Paragraph,
    },
};

#[derive(Clone)]
pub struct CommandPromptData {
    pub prompt: String,
}

pub struct CommandPrompt {
    data: CommandPromptData
}

impl Widget for CommandPrompt {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let CommandPromptData {
            prompt
        } = self.data;

        if prompt.is_empty() {
            return;
        }

        Paragraph::new(prompt)
            .render(area, buffer);
    }
}

impl CommandPrompt {
    pub fn new(data: CommandPromptData) -> Self {
        Self {
            data
        }
    }
}
