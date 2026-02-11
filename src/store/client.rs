use std::any::Any;

use super::{
    NestedStore,
};

#[derive(Debug, Clone)]
pub struct ClientStore {
    is_running: bool,
}

impl NestedStore for ClientStore {
    fn get(&self, field: &str) -> Box<dyn Any> {
        let result = match field {
            "is_running" => self.is_running.clone(),

            _ => panic!("key not found"),
        };

        Box::new(result)
    }

    fn dispatch(&mut self, action: &str, _payload: Box<dyn Any>) {
        match action {
            "start" => self.start(),
            "stop" => self.stop(),

            _ => panic!("action not found"),
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
