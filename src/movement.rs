use crate::app::{App, Panels};

pub fn handle_down(app: &mut App) -> () {
    if matches!(app.selected_panel, Panels::SideBar) {
        app.projects.next();

        app.item_list.items = app.projects.items[app.projects.state.selected().unwrap_or(0)]
            .dev_items
            .clone();

        app.item_list.state.select(Some(0));
    } else {
        if app.item_list.items.len() > 0 {
            app.item_list.next();
        }
    }
}

pub fn handle_up(app: &mut App) -> () {
    if matches!(app.selected_panel, Panels::SideBar) {
        app.projects.previous();

        app.item_list.items = app.projects.items[app.projects.state.selected().unwrap_or(0)]
            .dev_items
            .clone();

        app.item_list.state.select(Some(0));
    } else {
        if app.item_list.items.len() > 0 {
            app.item_list.previous();
        }
    }
}

pub fn handle_change_focus(app: &mut App) -> () {
    if matches!(app.selected_panel, Panels::SideBar) {
        app.selected_panel = Panels::NotePad;
    } else {
        app.selected_panel = Panels::SideBar;
    }
}
