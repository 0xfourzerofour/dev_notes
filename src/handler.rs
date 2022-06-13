use crate::app::{App, AppResult, InputMode};
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
        KeyCode::Char('j') | KeyCode::Down => {
            print!("DOWN");
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
        _ => {}
    }
    Ok(())
}
