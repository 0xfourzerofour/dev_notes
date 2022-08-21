use core::fmt::{Display, Formatter, Result};
use std::error;
use tui::backend::Backend;
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::projects::{Item, Project, StatefulList};
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
    pub item_list: StatefulList<Item>,
}

impl App {
    pub fn new(projects: Vec<Project>) -> Self {
        let proj_clone = projects.clone();

        let item_1 = &*proj_clone[0].dev_items;

        println!("{:?}", item_1);

        Self {
            running: true,
            selected_panel: Panels::SideBar,
            input_mode: InputMode::Normal,
            projects: StatefulList::with_items(projects),
            item_list: StatefulList::with_items(item_1.clone().to_vec()),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let header_main = widgets::header_and_main(frame.size());
        let chunks = widgets::main_chunks(header_main[1]);
        let sb_chunks = widgets::sidebar_chunks(chunks[0]);

        let mode = Paragraph::new(format!("{}", self.input_mode))
            .block(Block::default().title("MODE").borders(Borders::ALL));

        frame.render_widget(mode, sb_chunks[0]);

        let items = self.projects.clone();

        let projects: Vec<ListItem> = items
            .items
            .into_iter()
            .map(|p| {
                let content = vec![Spans::from(vec![
                    Span::raw(p.title),
                    Span::raw(p.updated_at),
                ])];
                ListItem::new(content)
            })
            .collect();

        let mut sb_color = Color::White;
        let mut main_color = Color::Yellow;

        if matches!(self.selected_panel, Panels::SideBar) {
            sb_color = Color::Yellow;
            main_color = Color::White;
        }

        let project_list = List::new(projects)
            .block(Block::default().title("Projects").borders(Borders::ALL))
            .style(Style::default().fg(sb_color))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(project_list, sb_chunks[1], &mut self.projects.state);

        let project_items = self
            .item_list
            .items
            .clone()
            .into_iter()
            .map(|d| ListItem::new(d.title))
            .collect::<Vec<ListItem>>();

        let project_list = List::new(project_items)
            .block(Block::default().title("Items").borders(Borders::ALL))
            .style(Style::default().fg(main_color))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_stateful_widget(project_list, chunks[1], &mut self.item_list.state);
    }
}
