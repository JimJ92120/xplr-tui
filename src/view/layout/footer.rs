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

pub struct FooterData {
    pub text_input: String
}

pub struct Footer {}

impl Footer {
    pub fn render(area: Rect, buffer: &mut Buffer, data: FooterData) {
        let [actions_container, input_container] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ]).areas(area);

        Self::render_actions_container(actions_container, buffer);
        Self::render_input_container(input_container, buffer, data);
    }

    fn render_actions_container(area: Rect, buffer: &mut Buffer) {
        Paragraph::new("1.copy | 2.move | 3.rename | 4.delete")
            .render(area, buffer);
    }

    fn render_input_container(area: Rect, buffer: &mut Buffer, data: FooterData) {
        Paragraph::new(format!("Input: {}", data.text_input))
            .render(area, buffer);
    }
}
