use std::any::Any;

mod command;

use command::{
    CommandStore,
};

pub trait NestedStore {
    fn get(&self, field: &str) -> Box<dyn Any>;

    fn dispatch(&mut self, action: &str, payload: Box<dyn Any>) {
        println!("action: {}\npayload: {:?}", action, payload);
    }
}


#[derive(Debug, Clone)]
pub enum StoreType {
    Command(CommandStore),
}
impl StoreType {
    pub fn get(&self) {
        println!("enum: {:?}", self);
    }
}

#[derive(Debug, Clone)]
pub struct Store {
    pub command: CommandStore,
}
impl Store {
    pub fn new() -> Self {
        Self {
            command: CommandStore::new(),
        }
    }

    pub fn get(&self, store_key: &str, field: &str) -> Box<dyn Any> {
        match store_key {
            "command" => self.command.clone().get(field),

            _ => panic!("store not found"),
        }
    }

    pub fn dispatch(&mut self, store_key: &str, action: &str, payload: Box<dyn Any>) {
        match store_key {
            "command" => {
                self.command.dispatch(action, payload);
            },

            _ => panic!("store not found"),
        }
    }
}
