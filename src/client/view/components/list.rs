use ratatui::{
    buffer::Buffer,
    layout::{Rect},
    widgets::{Widget, Paragraph, List, ListItem},
    style::{Color, Stylize}
};

#[derive(Debug)]
pub struct ListData {
    pub list: Vec<String>,
    pub selected_item_index: usize
}

#[derive(Debug)]
pub struct ListComponent {}

impl ListComponent {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ListData) {
        if 0 == data.list.len() {
            Paragraph::new("No item found.")
                .render(area, buffer);
        } else {
            ListComponent::render_list(
                data.list
                    .iter()
                    .enumerate()
                    .map(|(index, content)| {
                        let list_item = ListComponent::render_list_item(
                            content.to_string(),
                            index
                        );

                        if data.selected_item_index == index {
                            return ListComponent::highlight_list_item(list_item);
                        }

                        list_item
                    })
                    .collect()
            )
                .render(area, buffer);
                
        }
    }

    fn render_list(items: Vec<ListItem>) -> List {
        List::new(items)        
    }

    fn render_list_item(content: String, index: usize) -> ListItem<'static> {
        ListItem::from(format!("{}.{}", index + 1, content))
    }

    fn highlight_list_item(list_item: ListItem<'static>) -> ListItem<'static> {
        list_item
            .bg(Color::Green)
    }
}
