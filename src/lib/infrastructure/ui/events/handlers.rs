use ratatui::crossterm::event::KeyCode;

use crate::infrastructure::ui::app::{App, UiMode, View};

pub fn handle_key_up(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(0, -1);
    } else {
        app.previous_item();
    }
}

pub fn handle_key_down(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(0, 1);
    } else {
        app.next_item();
    }
}

pub fn handle_key_left(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(-1, 0);
    }
}

pub fn handle_key_right(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.move_cursor(1, 0);
    }
}

pub fn handle_key_enter(app: &mut App) {
    if app.ui_mode == UiMode::Placement {
        app.confirm_selection();
    }
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
