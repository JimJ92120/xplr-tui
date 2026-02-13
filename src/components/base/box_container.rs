use ratatui::{
    buffer::{ Buffer },
    layout::{ Rect },
    widgets::{ Widget, Block, BorderType, Padding }
};

pub struct BoxContainer {}

impl Widget for BoxContainer {
    fn render(self, _area: Rect, _buffer: &mut Buffer) {}
}

impl BoxContainer {
    pub fn new(title: String) -> Block<'static> {
        let box_container = Block::bordered()
            .border_type(BorderType::Rounded)
            .padding(Padding::new(1, 1, 0, 0));

        if !title.is_empty() {
            return box_container.title(format!(" {} ", title));
        }

        box_container
    }
}
