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
    dev_items: Option<Vec<Item>>,
    selected_item: u32,
}

impl Project {
    pub fn new(title: String) -> Self {
        Self {
            title,
            dev_items: None,
            selected_item: 0,
        }
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
