use std::any::Any;

use crate::{
    types::{
        Command,
    }
};

use super::{
    NestedStore,
};

mod controller;

use controller::{
    CommandController,
};

#[derive(Debug, Clone)]
pub struct CommandStore {
    current_command: Option<Command>,
    input: String,
}

impl NestedStore for CommandStore {
    fn get(&self, field: &str) -> Box<dyn Any> {
        match field {
            "current_command" => Box::new(self.current_command.clone()),
            "input" => Box::new(self.input.clone()),

            _ => panic!("{}", self.no_field_found(field)),
        }
    }

    fn action(&mut self, action: &str) {
        match action {
            "delete_input_last_char" => self.delete_input_last_char(),

            _ => panic!("{}", self.no_action_found(action)),
        };
    }

    fn dispatch(&mut self, action: &str, payload: Box<dyn Any>) {
        match action {
            "type_input" => self.type_input(
                payload.downcast_ref::<char>().unwrap().clone()
            ),
            "run_command" => self.run_command(
                payload.downcast_ref::<Command>().unwrap().clone()
            ),
            "copy_file" => self.copy_file(
                payload.downcast_ref::<String>().unwrap().clone()
            ),


            _ => panic!("{}", self.no_action_found(action)),
        };
    }
}

impl CommandStore {
    pub fn new() -> Self {
        Self {
            current_command: None,
            input: String::from(""),
        }
    }
    
    fn type_input(&mut self, char: char) {
        self.input.push(char);
    }
    fn delete_input_last_char(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }
    fn clear_input(&mut self) {
        self.input = String::new();
    }

    fn run_command(&mut self, command: Command) {
        match self.current_command.clone() {
            Some(current_command) => {
                if current_command == command {
                    self.current_command = None;
                }
            },
            None => {
                self.clear_input();
                self.current_command = Some(command);
            }
        }
    }

    fn copy_file(&mut self, source_path_name: String) {
        match self.current_command {
            Some(Command::Copy) => {
                if "" == self.input {
                    return;
                }

                let target_path_name = self.input.clone();
                let _ = CommandController::copy_file(
                    source_path_name.clone(),
                    target_path_name.clone()
                );

                self.current_command = None;
                self.input = format!(
                    "Copied '{}' to '{}'.",
                    source_path_name,
                    target_path_name,
                );
            },

            _ => (),
        } 
    }
}
