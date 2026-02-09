use std::{
    env,
    io::{Result},
    thread::sleep,
    time::Duration
};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod helpers;
mod app;
mod client;

use app::App;
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

    let root_directory_name = helpers::get_root_directory_name(path_name)?;
    let app = App::new(root_directory_name);

    let state = ClientState {
        is_running: false,
        frame: 0,
    };
    let data = ClientData {
        directory_name: app.get_current_directory_name(),
        directory_content: app.get_current_directory_content(),
        selected_item_index: 0,
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
                KeyCode::Esc => { state.is_running = false}
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
                KeyCode::Char(char) => { data.text_input.push(char) },
                _ => {}
            };
        } 
    };

    Ok(())
}
