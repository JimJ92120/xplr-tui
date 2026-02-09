use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint,
        Layout,
        Rect
    },
    widgets::{
        Widget,
        Block,
        Paragraph
    },
};

use super::components::list::{ListComponent, ListData};

#[derive(Clone)]
pub struct ContentData {
    pub directory_name: String,
    pub directory_content: Vec<(String, String)>,
    pub selected_item_index: usize,
    pub parent_directory_list: Vec<String>
}

pub struct Content {}

impl Content {
    pub fn render(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [left_container, right_container] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ]).areas(area);

        Content::render_left_container(left_container, buffer, data.clone());
        Content::render_right_container(right_container, buffer, data.clone());
    }

    fn render_left_container(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [title_container, list_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
        ]).areas(area);

        let directory_list_string = if data.parent_directory_list.is_empty() {
            data.directory_name.clone()
        } else {
            let mut directory_list = data.parent_directory_list.clone();
            directory_list.push(data.directory_name.clone());

            directory_list
                .iter()
                .enumerate()
                .map(|(index, directory_name)| {
                    if 0 < index {
                        let (_, formatted_directory_name) = directory_name
                            .split_once(
                                format!("{}/", directory_list[index - 1].clone()).as_str()
                            )
                            .unwrap();

                        format!(" > {}", formatted_directory_name).to_string()
                    } else {
                        directory_name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        };

        Block::new()
            .title(directory_list_string.to_string())
            .render(title_container, buffer);
        ListComponent::render(
            list_container,
            buffer,
            ListData {
                list: data.directory_content.clone(),
                selected_item_index: data.selected_item_index
            }
        );
    }

    fn render_right_container(area: Rect, buffer: &mut Buffer, data: ContentData) {
        let [title_container, details_container] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1)
        ]).areas(area);

        let parent_directory_list_string = if data.parent_directory_list.is_empty() {
            String::from("No parents found.")
        } else {
            format!("- {}", data.parent_directory_list.join("\n- "))
        };

        Block::new()
            .title(String::from("Details"))
            .render(title_container, buffer);
        Paragraph::new(
            format!(
                "Item:\n- name: {}\n- type: {}\n\nParents:\n{}",
                data.directory_content[data.selected_item_index].0,
                data.directory_content[data.selected_item_index].1,
                parent_directory_list_string
            )
        )
            .render(details_container, buffer);
    }
}
