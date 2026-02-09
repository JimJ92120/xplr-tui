use std::{
    env,
    io::{Result},
    thread::sleep,
    time::Duration
};
use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEventKind
};

mod api;
mod client;

use api::Api;
use client::{Client, ClientState};

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
        directory_name: root_directory_name.clone(),
        directory_content: Api::get_directory_content(root_directory_name)?,
        selected_item_index: 0,
        parent_directory_list: Vec::new(),
        text_input: String::new()
    };
    let mut client = Client::new(
        state,
        event_callback
    );

    println!("Starting XPLR...");
    sleep(Duration::from_secs(1));

    client.run()?;

    println!("Done!");

    Ok(())
}

fn event_callback(state: &mut ClientState) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if KeyEventKind::Press == key.kind {
            match key.code {
                KeyCode::Esc => { state.is_running = false },

                KeyCode::Up => {
                    if state.selected_item_index > 0 {
                        state.selected_item_index -= 1;
                    }
                },
                KeyCode::Down => {
                    if state.selected_item_index < state.directory_content.len() - 1 {
                        state.selected_item_index +=1;
                    }
                },

                KeyCode::Right => {
                    let selected_item = state.directory_content[state.selected_item_index].clone();

                    if "directory" == selected_item.1 {
                        let directory_name = selected_item.0.clone();
                        let directory_content = Api::get_directory_content(directory_name.clone());

                        match directory_content {
                            Ok(directory_content) => {
                                state.parent_directory_list.push(state.directory_name.clone());
                                state.directory_name = directory_name;
                                state.directory_content = directory_content;
                                state.selected_item_index = 0;
                            },
                            Err(error) => panic!("Unable to retrieve content for '{}' directory.\n{}", directory_name, error)
                        };
                    }
                },
                KeyCode::Left => {
                    if state.parent_directory_list.is_empty() {
                        return Ok(());
                    }

                    let current_directory_name = state.directory_name.clone();
                    let previous_directory_name = state.parent_directory_list.last().unwrap().to_string();
                    let previous_directory_content = Api::get_directory_content(previous_directory_name.clone());

                    match previous_directory_content {
                        Ok(directory_content) => {
                            state.parent_directory_list.pop();
                            state.directory_name = previous_directory_name.clone();
                            state.directory_content = directory_content.clone();
                            state.selected_item_index = directory_content
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

                KeyCode::Char(char) => { state.text_input.push(char) },
            
                _ => {}
            };
        } 
    };

    Ok(())
}
