use std::{
    any::{ Any },
};

use super::{ NestedStore };

#[derive(Debug, Clone)]
pub struct ClientStore {
    is_running: bool,
}

impl NestedStore for ClientStore {
    fn get(&self, field: &str) -> Box<dyn Any> {
        match field {
            "is_running" => Box::new(self.is_running.clone()),

            _ => panic!("{}", self.no_field_found(field)),
        }
    }

    fn action(&mut self, action: &str) {
        match action {
            "start" => self.start(),
            "stop" => self.stop(),

            _ => panic!("{}", self.no_action_found(action)),
        };
    }
}

impl ClientStore {
    pub fn new() -> Self {
        Self {
            is_running: false,
        }
    }

    fn start(&mut self) {
        if self.is_running {
           panic!("Client alread started.");
        }

        self.is_running = true;
    }

    fn stop(&mut self) {
        if !self.is_running {
            panic!("Client not started.");
        }

        self.is_running = false;
    }
}
