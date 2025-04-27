use ratatui::crossterm::event::KeyCode;

use crate::infrastructure::ui::app::{App, UiMode, View};

pub fn handle_key_up(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(0, -1);
    } else if app.ui_mode == UiMode::TowerSelection && app.is_tower_selection_on_map() {
        app.select_tower_on_map_up();
    } else {
        app.previous_item();
    }
}

pub fn handle_key_down(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(0, 1);
    } else if app.ui_mode == UiMode::TowerSelection && app.is_tower_selection_on_map() {
        app.select_tower_on_map_down();
    } else {
        app.next_item();
    }
}

pub fn handle_key_left(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(-1, 0);
    } else if app.ui_mode == UiMode::TowerSelection && app.is_tower_selection_on_map() {
        app.select_tower_on_map_left();
    }
}

pub fn handle_key_right(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(1, 0);
    } else if app.ui_mode == UiMode::TowerSelection && app.is_tower_selection_on_map() {
        app.select_tower_on_map_right();
    }
}

pub fn handle_key_enter(app: &mut App) {
    app.confirm_selection();
}

pub fn handle_key_esc(app: &mut App) {
    if app.ui_mode == UiMode::Placement
        || app.ui_mode == UiMode::TowerSelection
        || app.ui_mode == UiMode::TowerUpgrade
    {
        app.cancel_action();
    }
}

pub fn handle_key_p(app: &mut App) {
    if app.current_view == View::Game {
        app.set_view(View::Pause);
    }
}

pub fn handle_key_q(app: &mut App) {
    if app.current_view == View::Game {
        app.set_view(View::Pause);
    } else {
        app.quit();
    }
}

pub fn handle_key_t(app: &mut App) {
    if app.current_view == View::Game && app.ui_mode == UiMode::Normal {
        app.start_tower_selection_on_map();
    }
}
