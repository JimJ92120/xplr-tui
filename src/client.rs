use std::io::{Result};

use ratatui::{DefaultTerminal, Frame};

type StopCallback = fn() -> Result<bool>;

#[derive(Debug)]
pub struct Client {
    stop_callback: StopCallback
}

impl Client {
    pub fn new(stop_callback: StopCallback) -> Self {
        Self {
            stop_callback
        }
    }

    pub fn run(&mut self) -> Result<()> {
        ratatui::run(|terminal| self.render(terminal))?;

        Ok(())
    }

    fn render(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;

            if (self.stop_callback)()? {
                break Ok(());
            }
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }
}
