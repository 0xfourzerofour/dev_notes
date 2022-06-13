use dev_notes::app::{App, AppResult};
use dev_notes::event::{Event, EventHandler};
use dev_notes::file_handler::{load_projects, save_projects};
use dev_notes::handler::handle_key_events;
use dev_notes::projects::Project;
use dev_notes::tui::Tui;
use std::convert::TryInto;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let saved_proj = load_projects()?;
    let mut app = App::new(saved_proj);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    save_projects(&app.projects.items)?;

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
