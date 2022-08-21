use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DevItem {
    pub title: String,
    pub complete: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub items: Vec<DevItem>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProjectState {
    pub projects: Vec<Project>,
    pub selected_index: usize,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            projects: Vec::new(),
            selected_index: 0,
        }
    }
}

impl ProjectState {
    pub fn load() {}
}
