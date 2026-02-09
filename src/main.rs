use std::{
    env,
    io::{Result},
    thread::sleep,
    time::Duration
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
    let mut view = View::new(state);

    println!("Starting...");
    sleep(Duration::from_secs(1));

    view.run()?;

    println!("Done!");

    Ok(())
}
