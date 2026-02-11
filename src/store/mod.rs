use std::any::Any;

mod command;
mod client;
mod directory;

use command::{
    CommandStore,
};
use client::{
    ClientStore,
};
use directory::{
    DirectoryStore
};

pub trait NestedStore {
    fn get(&self, field: &str) -> Box<dyn Any>;

    fn action(&mut self, action: &str) {
        println!("No action implemented.\naction: {}", action);
    }

    fn dispatch(&mut self, action: &str, payload: Box<dyn Any>) {
        println!("No dispatch implemented.\naction: {}\npayload: {:?}", action, payload);
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
    command: CommandStore,
    client: ClientStore,
    directory: DirectoryStore,
}
impl Store {
    pub fn new(path_name: String) -> Self {
        Self {
            command: CommandStore::new(),
            client: ClientStore::new(),
            directory: DirectoryStore::new(path_name),
        }
    }

    fn store_not_found(store_key: &str) -> String {
        format!("'{}' store not found.", store_key)
    }

    pub fn get(&self, store_key: &str, field: &str) -> Box<dyn Any> {
        match store_key {
            "command" => self.command.clone().get(field),
            "client" => self.client.clone().get(field),
            "directory" => self.directory.clone().get(field),

            _ => panic!("{}", Self::store_not_found(store_key)),
        }
    }

    pub fn action(&mut self, store_key: &str, action: &str) {
        match store_key {
            "command" => self.command.action(action),
            "client" => self.client.action(action),
            "directory" => self.directory.action(action),

            _ => panic!("{}", Self::store_not_found(store_key)),
        };
    }

    pub fn dispatch(&mut self, store_key: &str, action: &str, payload: Box<dyn Any>) {
        match store_key {
            "command" => self.command.dispatch(action, payload),
            "client" => self.client.dispatch(action, payload),
            "directory" => self.directory.dispatch(action, payload),

            _ => panic!("{}", Self::store_not_found(store_key)),
        };
    }
}
