use std::io::{Result};

use ratatui::{DefaultTerminal, Frame};

type EventCallback = fn(state: &mut ClientState) -> Result<()>;

#[derive(Debug)]
pub struct ClientState {
    pub is_running: bool,
    pub frame: usize,
    pub count: isize
}

#[derive(Debug)]
pub struct Client {
    state: ClientState,
    event_callback: EventCallback
}

impl Client {
    pub fn new(state: ClientState, event_callback: EventCallback) -> Self {
        Self {
            state,
            event_callback
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal| -> Result<()> {
            self.state.is_running = true;

            while self.state.is_running {
                self.state.frame += 1;

                terminal.draw(|frame| self.render(frame))?;

                (self.event_callback)(&mut self.state)?;
            };

            Ok(())
        })?;

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {

        frame.render_widget(
            format!("frame: {}\ncount: {}", self.state.frame, self.state.count),
            frame.area()
        );
    }
}
