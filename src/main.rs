use std::{
    io::{Result},
    thread::sleep,
    time::Duration
};

mod client;

use client::Client;

fn main() -> Result<()> {
    let mut client = Client::new(|| Ok(crossterm::event::read()?.is_key_press()));
    println!("client: {:?}", client);

    sleep(Duration::from_secs(2));

    client.run()?;

    Ok(())
}
