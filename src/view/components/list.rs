use ratatui::{
    buffer::Buffer,
    layout::{ Rect },
    widgets::{
        Widget,
        Paragraph,
    },
    text::{ Line },
    style::{
        Color,
        Stylize
    }
};

pub struct ListData {
    pub list: Vec<(String, String)>,
    pub selected_item_index: usize,
}

pub struct List {
    data: ListData,
}

impl Widget for List {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        if self.data.list.is_empty() {
            self.render_no_list(area, buffer);
        } else {
            self.render_list(area, buffer);                
        }
    }
}

impl List {
    pub fn new(data: ListData) -> Self {
        Self {
            data
        }
    }

    fn render_no_list(self, area: Rect, buffer: &mut Buffer) {
        Paragraph::new("No item found.")
            .render(area, buffer);
    }

    fn render_list(self, area: Rect, buffer: &mut Buffer) {
        let ListData {
            selected_item_index,
            list
        } = self.data;
        let scroll = if (selected_item_index as u16) < area.height {
            0
        } else {
            (selected_item_index as u16) - area.height + 1
        };

        Paragraph::new(
            list
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    let line = Line::from(format!("{}.{}", index, item.0));

                    if selected_item_index == index {
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
