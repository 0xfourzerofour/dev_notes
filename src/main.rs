use dev_notes::app::{App, AppResult};
use dev_notes::event::{Event, EventHandler};
use dev_notes::file_handler::save_projects;
use dev_notes::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    let mut app = App::new()?;

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.controller.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => app.controller.handle_key_event(key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    save_projects(&app.controller.state)?;

    tui.exit()?;
    Ok(())
}
