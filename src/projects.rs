use serde::{Deserialize, Serialize};
use tui::widgets::ListState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    title: String,
    description: String,
    finished: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub dev_items: Vec<Item>,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Project {
    pub fn new_project(title: String) -> Self {
        Self {
            title,
            dev_items: Vec::new(),
            updated_at: String::from(""),
        }
    }

    pub fn new(projects: &[Project]) -> &[Self] {
        return projects;
    }
}

impl Item {
    pub fn new(title: String, description: String) -> Self {
        Self {
            title,
            description,
            finished: false,
        }
    }

    pub fn toggle_finished(&mut self) {
        self.finished = !self.finished;
    }
}
