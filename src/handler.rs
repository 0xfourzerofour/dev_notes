use crate::{
    app::{App, AppResult, InputMode},
    projects::{Item, Project},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`] depending on the input_mode.
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
        KeyCode::Char('j') | KeyCode::Down => app.projects.next(),
        KeyCode::Char('k') | KeyCode::Up => app.projects.previous(),

        KeyCode::Char('n') => {
            print!("NEW")
        }

        // exit application on Ctrl-D
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
        // exit application on ESC
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
