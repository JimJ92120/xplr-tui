use std::{
    any::Any,
    clone::Clone
};

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

    fn no_field_found(&self, field: &str) -> String {
        format!("Field '{}' not found", field)
    }

    fn no_action_found(&self, action: &str) -> String {
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

    pub fn get<T: Clone + 'static>(&self, store_key: &str, field: &str) -> T {
        self.get_nested_store(store_key)
            .get(field)
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    pub fn action(&mut self, store_key: &str, action: &str) {
        self.get_nested_store_mut(store_key)
            .action(action);
    }

    pub fn dispatch(&mut self, store_key: &str, action: &str, payload: Box<dyn Any>) {
        self.get_nested_store_mut(store_key)
            .dispatch(action, payload)
    }

    fn get_nested_store(&self, store_key: &str) -> Box<&dyn NestedStore> {
        Box::new(match store_key {
            "command" => &self.command,
            "client" => &self.client,
            "directory" => &self.directory,

            _ => panic!("{}", Self::store_not_found(store_key)),
        })
    }

    fn get_nested_store_mut(&mut self, store_key: &str) -> Box<&mut dyn NestedStore> {
        Box::new(match store_key {
            "command" => &mut self.command,
            "client" => &mut self.client,
            "directory" => &mut self.directory,

            _ => panic!("{}", Self::store_not_found(store_key)),
        })
    }
}
