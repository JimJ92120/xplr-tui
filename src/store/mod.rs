use std::any::Any;

mod command;
mod client;

use command::{
    CommandStore,
};
use client::{
    ClientStore,
};

pub trait NestedStore {
    fn get(&self, field: &str) -> Box<dyn Any>;

    fn dispatch(&mut self, action: &str, payload: Box<dyn Any>) {
        println!("action: {}\npayload: {:?}", action, payload);
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub command: CommandStore,
    pub client: ClientStore,
}
impl Store {
    pub fn new() -> Self {
        Self {
            command: CommandStore::new(),
            client: ClientStore::new(),
        }
    }

    pub fn get(&self, store_key: &str, field: &str) -> Box<dyn Any> {
        match store_key {
            "command" => self.command.clone().get(field),
            "client" => self.client.clone().get(field),

            _ => panic!("store not found"),
        }
    }

    pub fn dispatch(&mut self, store_key: &str, action: &str, payload: Box<dyn Any>) {
        match store_key {
            "command" => {
                self.command.dispatch(action, payload);
            },
            "client" => {
                self.client.dispatch(action, payload);
            },

            _ => panic!("store not found"),
        }
    }
}
