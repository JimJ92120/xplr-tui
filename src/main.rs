use std::{
    env,
    io::{Result},
    thread::sleep,
    time::Duration
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod api;
mod client;

use api::Api;
use client::{Client, ClientState, ClientData};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 2 < args.len() {
        panic!("Too many arguments.\nTry '$xplr' or '$xplr /path/to/directory'.");
    }

    let mut path_name = String::new();
    if 2 == args.len() {
        path_name = args[1].clone();
    }

    let root_directory_name = Api::get_root_directory_name(path_name)?;

    let state = ClientState {
        is_running: false,
        frame: 0,
    };
    let data = ClientData {
        directory_name: root_directory_name.clone(),
        directory_content: Api::get_directory_content(root_directory_name)?,
        selected_item_index: 0,
        parent_directory_list: Vec::new(),
        text_input: String::new()
    };
    let mut client = Client::new(
        state,
        data,
        event_callback
    );

    println!("Starting XPLR...");
    sleep(Duration::from_secs(1));

    client.run()?;

    println!("Done!");

    Ok(())
}

fn event_callback(state: &mut ClientState, data: &mut ClientData) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if KeyEventKind::Press == key.kind {
            match key.code {
                KeyCode::Esc => { state.is_running = false },

                KeyCode::Up => {
                    if data.selected_item_index > 0 {
                        data.selected_item_index -= 1;
                    }
                },
                KeyCode::Down => {
                    if data.selected_item_index < data.directory_content.len() - 1 {
                        data.selected_item_index +=1;
                    }
                },

                KeyCode::Right => {
                    let selected_item = data.directory_content[data.selected_item_index].clone();

                    if "directory" == selected_item.1 {
                        let directory_name = selected_item.0.clone();
                        let directory_content = Api::get_directory_content(directory_name.clone());

                        match directory_content {
                            Ok(directory_content) => {
                                data.parent_directory_list.push(data.directory_name.clone());
                                data.directory_name = directory_name;
                                data.directory_content = directory_content;
                                data.selected_item_index = 0;
                            },
                            Err(error) => panic!("Unable to retrieve content for '{}' directory.\n{}", directory_name, error)
                        };
                    }
                },
                KeyCode::Left => {
                    if data.parent_directory_list.is_empty() {
                        return Ok(());
                    }

                    let current_directory_name = data.directory_name.clone();
                    let previous_directory_name = data.parent_directory_list.last().unwrap().to_string();
                    let previous_directory_content = Api::get_directory_content(previous_directory_name.clone());

                    match previous_directory_content {
                        Ok(directory_content) => {
                            data.parent_directory_list.pop();
                            data.directory_name = previous_directory_name.clone();
                            data.directory_content = directory_content.clone();
                            data.selected_item_index = directory_content
                                    .iter()
                                    .position(|directory_name| current_directory_name == directory_name.0)
                                    .unwrap();
                        },
                        Err(error) => panic!(
                            "Unable to retrieve previous directory content for '{}' directory.\n{}",
                            previous_directory_name,
                            error
                        )
                    };       
                }

                KeyCode::Char(char) => { data.text_input.push(char) },
            
                _ => {}
            };
        } 
    };

    Ok(())
}
