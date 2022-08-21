use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, controller::Panels};

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

pub fn header_chunks(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(area);

    return chunks;
}

pub fn render_header<B: Backend>(_app: &App, frame: &mut Frame<'_, B>, size: Vec<Rect>) {
    let mode = Paragraph::new("HEADER").block(Block::default().title("MODE").borders(Borders::ALL));

    frame.render_widget(mode, size[0]);
}

pub fn render_side_bar<B: Backend>(app: &App, frame: &mut Frame<'_, B>, size: Vec<Rect>) {
    let mode = Paragraph::new(format!("{}", app.controller.mode))
        .block(Block::default().title("MODE").borders(Borders::ALL));

    frame.render_widget(mode, size[0]);

    let items = app.controller.state.projects.clone();

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

    if matches!(app.controller.selected_panel, Panels::SideBar) {
        sb_color = Color::Yellow;
    }

    let project_list = List::new(projects)
        .block(Block::default().title("Projects").borders(Borders::ALL))
        .style(Style::default().fg(sb_color))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    frame.render_widget(project_list, size[1]);
}

pub fn render_editor<B: Backend>(app: &App, frame: &mut Frame<'_, B>) {}

pub fn calculate_modal_margin(size: Rect) -> (u16, u16) {
    (size.width / 5, size.height / 5)
}

pub fn render_insert_modal<B: Backend>(app: &App, frame: &mut Frame<'_, B>, size: Rect) {
    let mut modal_title = "";
    if app.controller.insert_modal_visible {
        match app.controller.selected_panel {
            Panels::NotePad => modal_title = "NOTEPAD",
            Panels::SideBar => modal_title = "SIDEBAR",
        }

        let (x, y) = calculate_modal_margin(size);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Length(2)].as_ref())
            .vertical_margin(y)
            .horizontal_margin(x)
            .split(size);

        frame.render_widget(
            Paragraph::new(modal_title)
                .block(Block::default().title("Insert").borders(Borders::ALL)),
            layout[0],
        );
    }
}

pub fn render_page<B: Backend>(app: &App, frame: &mut Frame<'_, B>) {
    let header_main = header_and_main(frame.size());
    let header_chunks = header_chunks(header_main[0]);
    let chunks = main_chunks(header_main[1]);
    let sb_chunks = sidebar_chunks(chunks[0]);

    render_header(app, frame, header_chunks);
    render_side_bar(app, frame, sb_chunks);
    render_editor(app, frame);
    render_insert_modal(app, frame, frame.size());
}
