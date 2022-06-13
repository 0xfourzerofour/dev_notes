use tui::layout::{Constraint, Layout, Rect};

pub fn sidebar_widget(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(9),
                Constraint::Min(8),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);

    return chunks;
}
