use std::{
    env,
    io::{ Result },
    thread::sleep,
    time::Duration
};

mod types;
mod components;

mod view;
mod store;

use view::{ View };
use store::{ Store };

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

    let mut view = View::new(
        Store::new(path_name.to_string())
    );

    println!("Starting...");
    sleep(Duration::from_secs(1));

    view.run()?;

    println!("Done!");

    Ok(())
}
