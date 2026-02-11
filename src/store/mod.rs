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

    fn action(&mut self, action: &str) {
        println!("action: {}", action);
    }

    fn dispatch(&mut self, action: &str, payload: Box<dyn Any>) {
        println!("action: {}\npayload: {:?}", action, payload);
    }

    fn no_field_found(field: &str) -> String {
        format!("Field '{}' not found", field)
    }

    fn no_action_found(action: &str) -> String {
        format!("Action '{}' not found", action)
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

    pub fn action(&mut self, store_key: &str, action: &str) {
        match store_key {
            "command" => {
                self.command.action(action);
            },
            "client" => {
                self.client.action(action);
            },

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
