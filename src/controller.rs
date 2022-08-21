use core::fmt::{Display, Formatter, Result};
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

use crate::{app::App, file_handler::load_projects, state::ProjectState};

#[derive(Debug, Clone, Copy)]
pub enum Panels {
    SideBar,
    NotePad,
}

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Insert,
}

impl Display for InputMode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InputMode::Normal => write!(f, "NORMAL"),
            InputMode::Insert => write!(f, "INSERT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub mode: InputMode,
    pub selected_panel: Panels,
    pub running: bool,
    pub state: ProjectState,
    pub insert_modal_visible: bool,
}

impl Controller {
    pub fn new() -> io::Result<Self> {
        let saved_projects = load_projects()?;

        Ok(Self {
            mode: InputMode::Normal,
            selected_panel: Panels::SideBar,
            running: true,
            state: saved_projects,
            insert_modal_visible: false,
        })
    }

    pub fn mode(&self) -> InputMode {
        self.mode
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            InputMode::Insert => self.mode = InputMode::Normal,
            InputMode::Normal => self.mode = InputMode::Insert,
        }
    }

    pub fn toggle_panel(&mut self) {
        match self.selected_panel {
            Panels::SideBar => self.selected_panel = Panels::NotePad,
            Panels::NotePad => self.selected_panel = Panels::SideBar,
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            InputMode::Insert => self.handle_insert_mode(key_event),
            InputMode::Normal => self.handle_normal_mode(key_event),
        }
    }

    fn handle_insert_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.mode = InputMode::Normal,
            _ => {}
        }
    }
    fn handle_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('i') => self.mode = InputMode::Insert,
            KeyCode::Char('n') => {
                self.insert_modal_visible = true;
                self.toggle_mode();
            }
            KeyCode::Tab => self.toggle_panel(),
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.running = false;
            }
            _ => {}
        }
    }

    fn _handle_down(&self, _app: &mut App) {}

    fn _handle_up(&self, _app: &mut App) {}
}
