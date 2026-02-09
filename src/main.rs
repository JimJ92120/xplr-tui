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
mod view;
mod controller;

use api::Api;
use view::{View, State};
use controller::Controller;

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

    let state = State {
        is_running: false,
        title: String::from("XPLR"),
        directory_name: root_directory_name.clone(),
        directory_content: Api::get_directory_content(root_directory_name)?,
        selected_item_index: 0,
        parent_directory_list: Vec::new(),
        text_input: String::new()
    };
    let mut view = View::new(
        state,
        event_callback
    );

    println!("Starting...");
    sleep(Duration::from_secs(1));

    view.run()?;

    println!("Done!");

    Ok(())
}

fn event_callback(state: &mut State) -> Result<()> {
    if let Event::Key(key) = event::read()? {
        if KeyEventKind::Press == key.kind {
            match key.code {
                KeyCode::Esc => Controller::stop(state),

                KeyCode::Up => Controller::select_previous_item(state),
                KeyCode::Down => Controller::select_next_item(state),

                KeyCode::Right => Controller::load_next_directory(state),
                KeyCode::Left => Controller::load_previous_directory(state),

                KeyCode::Char(char) => Controller::type_text(state, char),
            
                _ => {}
            };
        } 
    };

    Ok(())
}
