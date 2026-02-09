use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    widgets::{
        Widget,
        Paragraph,
    },
    text::{Line},
    style::{
        Color,
        Stylize
    }
};

pub struct ListData {
    pub list: Vec<(String, String)>,
    pub selected_item_index: usize,
}

pub struct List {}

impl List {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ListData) {
        if data.list.is_empty() {
            Self::render_no_list(area, buffer);
        } else {
            Self::render_list(area, buffer, data);                
        }
    }

    fn render_no_list(area: Rect, buffer: &mut Buffer) {
        Paragraph::new("No item found.")
            .render(area, buffer);
    }

    fn render_list(area: Rect, buffer: &mut Buffer, data: ListData) {
        let selected_item_index = data.selected_item_index as u16;
        let scroll = if selected_item_index < area.height {
            0
        } else {
            selected_item_index - area.height + 1
        };

        Paragraph::new(
            data.list
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let line = Line::from(format!("{}.{}", index, item.0));

                    if data.selected_item_index == index {
                        return line.bg(Color::Green);
                    }

                    line
                })
                .collect::<Vec<Line>>()
        )
            .scroll((scroll, 0))
            .render(area, buffer);
    }
}
