use core::fmt::{Display, Formatter, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
}

impl Controller {
    pub fn new() -> io::Result<Self> {
        let saved_projects = load_projects()?;

        Ok(Self {
            mode: InputMode::Normal,
            selected_panel: Panels::SideBar,
            running: true,
            state: saved_projects,
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
            // KeyCode::Char('j') | KeyCode::Down => self.handle_down(app),
            // KeyCode::Char('k') | KeyCode::Up => self.handle_up(app),
            KeyCode::Char('n') => match self.selected_panel {
                Panels::SideBar => println!("NEW PROJECT"),
                Panels::NotePad => println!("NEW NOTE"),
            },
            KeyCode::Tab => self.toggle_panel(),
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.running = false;
            }
            _ => {}
        }
    }

    fn _handle_down(&self, _app: &mut App) {
        // if matches!(self.selected_panel, Panels::SideBar) {
        //     app.projects.next();

        //     app.item_list.items = app.projects.items[app.projects.state.selected().unwrap_or(0)]
        //         .dev_items
        //         .clone();

        //     app.item_list.state.select(Some(0));
        // } else {
        //     if app.item_list.items.len() > 0 {
        //         app.item_list.next();
        //     }
        // }
    }

    fn _handle_up(&self, _app: &mut App) {
        // if matches!(self.selected_panel, Panels::SideBar) {
        //     app.projects.previous();

        //     app.item_list.items = app.projects.items[app.projects.state.selected().unwrap_or(0)]
        //         .dev_items
        //         .clone();

        //     app.item_list.state.select(Some(0));
        // } else {
        //     if app.item_list.items.len() > 0 {
        //         app.item_list.previous();
        //     }
        // }
    }
}
