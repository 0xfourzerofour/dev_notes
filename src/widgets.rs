use tui::layout::{Constraint, Direction, Layout, Rect};

pub fn main_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(area);

    return chunks;
}
