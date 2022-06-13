use core::fmt::{Display, Formatter, Result};
use std::error;
use std::mem::{replace, swap, take};
use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::projects::{Project, StatefulList};
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

impl Display for InputMode {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            InputMode::Normal => write!(f, "NORMAL"),
            InputMode::Insert => write!(f, "INSERT"),
        }
    }
}

#[derive(Debug)]
pub enum Panels {
    SideBar,
    NotePad,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub selected_panel: Panels,
    pub input_mode: InputMode,
    pub projects: StatefulList<Project>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(projects: Vec<Project>) -> Self {
        Self {
            running: true,
            selected_panel: Panels::SideBar,
            input_mode: InputMode::Normal,
            projects: StatefulList::with_items(projects),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let chunks = widgets::main_chunks(frame.size());

        let items = self.projects.clone();

        let projects = items
            .items
            .into_iter()
            .map(|p| ListItem::new(p.title))
            .collect::<Vec<ListItem>>();

        let project_list = List::new(projects)
            .block(Block::default().title("Projects").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(project_list, chunks[0], &mut self.projects.state);

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
