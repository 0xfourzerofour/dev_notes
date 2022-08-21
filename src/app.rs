use std::{error, io};
use tui::backend::Backend;
use tui::style::{Color, Modifier, Style};
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::controller::{Controller, Panels};
use crate::state::Project;
use crate::widgets;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    pub controller: Controller,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let controller = Controller::new()?;

        Ok(Self { controller })
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        let header_main = widgets::header_and_main(frame.size());
        let chunks = widgets::main_chunks(header_main[1]);
        let sb_chunks = widgets::sidebar_chunks(chunks[0]);

        let mode = Paragraph::new(format!("{}", self.controller.mode))
            .block(Block::default().title("MODE").borders(Borders::ALL));

        frame.render_widget(mode, sb_chunks[0]);

        let items = self.controller.state.projects.clone();

        let projects: Vec<ListItem> = items
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

        if matches!(self.controller.selected_panel, Panels::SideBar) {
            sb_color = Color::Yellow;
            main_color = Color::White;
        }

        let project_list = List::new(projects)
            .block(Block::default().title("Projects").borders(Borders::ALL))
            .style(Style::default().fg(sb_color))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_widget(project_list, sb_chunks[1]);

        let sel_project = self.controller.state.selected_index;

        match self.controller.state.projects.get(sel_project) {
            Some(project) => {
                let project_items = project
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

                frame.render_widget(project_list, chunks[1]);
            }
            None => {}
        }
    }
}
