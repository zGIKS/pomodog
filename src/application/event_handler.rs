use crate::domain::{App, AppState, InputError};
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

pub fn handle_key_event(app: &mut App, key: event::KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }

    match (key.code, key.modifiers, app.state()) {
        (KeyCode::Char('q'), _, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL, _) => app.quit(),

        (KeyCode::Up, _, AppState::Menu) => app.prev_option(),
        (KeyCode::Down, _, AppState::Menu) => app.next_option(),
        (KeyCode::Enter, _, AppState::Menu) => app.enter_task_input(),

        (KeyCode::Char(c), mods, AppState::TaskInput) => {
            if mods.is_empty() || mods == KeyModifiers::SHIFT {
                app.add_char_to_task(c);
                app.clear_input_error();
            }
        }
        (KeyCode::Backspace, mods, AppState::TaskInput) => {
            if mods == KeyModifiers::CONTROL {
                app.remove_word_from_task();
            } else {
                if app.task_name().is_empty() {
                    app.set_input_error(InputError::Empty);
                } else {
                    app.remove_char_from_task();
                }
            }
            app.clear_input_error();
        }
        (KeyCode::Enter, _, AppState::TaskInput) => {
            if !app.task_name().is_empty() {
                app.start_session();
            } else {
                app.set_input_error(InputError::Empty);
            }
        }
        (KeyCode::Esc, _, AppState::TaskInput) => app.enter_menu(),

        (KeyCode::Char(' '), _, AppState::Running) | (KeyCode::Char(' '), _, AppState::Paused) => {
            app.toggle_pause()
        }

        _ => {}
    }
}

pub fn handle_mouse_event(app: &mut App, mouse: event::MouseEvent) {
    if app.state() == AppState::Menu {
        match mouse.kind {
            MouseEventKind::ScrollDown => app.next_option(),
            MouseEventKind::ScrollUp => app.prev_option(),
            _ => {}
        }
    }
}
