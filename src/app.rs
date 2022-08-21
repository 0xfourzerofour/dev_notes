use std::{error, io};
use tui::backend::Backend;
use tui::terminal::Frame;

use crate::controller::Controller;
use crate::widgets::render_page;

pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub controller: Controller,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let controller = Controller::new()?;

        Ok(Self { controller })
    }

    pub fn tick(&self) {}

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        render_page(self, frame);
    }
}
