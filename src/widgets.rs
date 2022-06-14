use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(area);

    return chunks;
}

pub fn header_and_main(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    return chunks;
}

pub fn sidebar_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    return chunks;
}
