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

pub enum StoreType {
    Command,
    Client,
    Directory,
}

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

    // prevent using `get_nested_store_mut` for getters with e.g:
    // fn get_value(&mut self) {
    //   self.store.get("store_key", "field")
    // }
    pub fn get<T: Clone + 'static>(&self, store_type: StoreType, field: &str) -> T {
        self.get_nested_store(store_type)
            .get(field)
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }

    pub fn action(&mut self, store_type: StoreType, action: &str) {
        self.get_nested_store_mut(store_type)
            .action(action);
    }

    pub fn dispatch(&mut self, store_type: StoreType, action: &str, payload: Box<dyn Any>) {
        self.get_nested_store_mut(store_type)
            .dispatch(action, payload)
    }

    fn get_nested_store(&self, store_type: StoreType) -> Box<&dyn NestedStore> {
        Box::new(match store_type {
            StoreType::Command => &self.command,
            StoreType::Client => &self.client,
            StoreType::Directory => &self.directory,
        })
    }

    fn get_nested_store_mut(&mut self, store_type: StoreType) -> Box<&mut dyn NestedStore> {
        Box::new(match store_type {
            StoreType::Command => &mut self.command,
            StoreType::Client => &mut self.client,
            StoreType::Directory => &mut self.directory,
        })
    }
}
