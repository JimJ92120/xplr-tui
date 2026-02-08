use std::{
    env,
    io::{Result},
    thread::sleep,
    time::Duration
};

mod helpers;
mod app;
mod client;

use app::App;
use client::Client;

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
    println!("root dir: {}", root_directory_name);

    let app = App::new(root_directory_name);
    println!("current: {:?}", app.get_current_directory_name());
    println!("list: {:?}", app.get_directory_list());

    let mut client = Client::new(|| Ok(crossterm::event::read()?.is_key_press()));
    println!("client: {:?}", client);

    sleep(Duration::from_secs(2));

    client.run()?;

    Ok(())
}
