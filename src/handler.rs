use crate::{
    app::{App, AppResult, InputMode, Panels},
    movement::{handle_change_focus, handle_down, handle_up},
    projects::{Item, Project},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if matches!(app.input_mode, InputMode::Normal) {
        return handle_normal_mode(key_event, app);
    } else {
        return handle_insert_mode(key_event, app);
    }
}

fn handle_normal_mode(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Char('i') => app.input_mode = InputMode::Insert,
        KeyCode::Char('j') | KeyCode::Down => handle_down(app),
        KeyCode::Char('k') | KeyCode::Up => handle_up(app),
        KeyCode::Char('n') => match app.selected_panel {
            Panels::SideBar => println!("NEW PROJECT"),
            Panels::NotePad => println!("NEW NOTE"),
        },
        KeyCode::Tab => handle_change_focus(app),
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_insert_mode(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => app.input_mode = InputMode::Normal,

        KeyCode::Enter => app.projects.items[app.projects.state.selected().unwrap_or(0)]
            .dev_items
            .push(Item {
                title: String::from("test title"),
                description: String::from("ewtewtew"),
                finished: false,
            }),
        _ => {}
    }
    Ok(())
}
