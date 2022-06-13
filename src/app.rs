use std::error;
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::widgets;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum InputMode {
    /// Navigation
    Normal,
    /// Inserting data
    Insert,
}

#[derive(Debug)]
pub enum Panels {
    SideBar,
    NotePad,
}

#[derive(Debug)]
pub struct Item {
    title: String,
    description: String,
    finished: bool,
}

#[derive(Debug)]
pub struct Project {
    title: String,
    dev_items: Vec<Item>,
    selected_item: u32,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub selected_panel: Panels,
    pub input_mode: InputMode,
    pub projects: Option<Vec<Project>>,
    pub selected_project: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            selected_panel: Panels::SideBar,
            input_mode: InputMode::Normal,
            projects: None,
            selected_project: 0,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(frame.size());

        let items = [
            ListItem::new("Project Esign"),
            ListItem::new("Company Dash"),
            ListItem::new("New pro"),
        ];

        let list = List::new(items)
            .block(Block::default().title("Projects").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_widget(list, chunks[0]);

        let items = [
            ListItem::new("Project Esign"),
            ListItem::new("Company Dash"),
            ListItem::new("New pro"),
        ];

        let list = List::new(items)
            .block(Block::default().title("Items").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_widget(list, chunks[1]);
    }
}
