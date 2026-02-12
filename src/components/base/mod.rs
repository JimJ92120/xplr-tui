use ratatui::{
    widgets::{ Block, BorderType, Padding }
};

pub fn box_container(title: String) -> Block<'static> {
    let box_container = Block::bordered()
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 0, 0));

    if !title.is_empty() {
        return box_container.title(format!(" {} ", title));
    }

    box_container
}
