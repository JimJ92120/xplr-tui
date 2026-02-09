use std::{
    env,
    io::{ Result },
    thread::sleep,
    time::Duration
};

mod api;
mod view;
mod state;

use api::Api;
use view::View;
use state::State;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if 2 < args.len() {
        panic!("Too many arguments.\nTry '$xplr' or '$xplr /path/to/directory'.");
    }

    let path_name = if 2 == args.len() {
         args[1].clone()
    } else {
        String::new()
    };

    let state = State::new(
        String::from("XPLR"),
        path_name.to_string()
    );
    let mut view = View::new(state);

    println!("Starting...");
    sleep(Duration::from_secs(1));

    view.run()?;

    println!("Done!");

    Ok(())
}
