use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

mod app;
mod components;
mod tui;
mod ui;

use crate::app::{App, AppState};

fn main() -> Result<()> {
    // 1. Terminal Panic Hook (Absolute Fault Tolerance)
    // Ensures terminal restoration even if the app panics.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = tui::restore();
        original_hook(panic_info);
    }));

    // 2. TUI Initialization
    let mut terminal = tui::init()?;
    let mut app = App::new();
    
    // 3. Timing Constants
    let animation_tick = Duration::from_millis(150);
    let mut last_animation = Instant::now();
    let mut last_second = Instant::now();

    // 4. Efficient Event-Driven Loop
    while !app.should_quit {
        // Render only when needed
        terminal.draw(|f| ui::render(f, &app))?;

        // Calculate the precise time until the next animation frame
        // This is the key to LOW CPU usage. We only wait for what's strictly necessary.
        let next_update = animation_tick
            .checked_sub(last_animation.elapsed())
            .unwrap_or(Duration::from_millis(10)); // Minimum fallback

        // Wait for an event OR the timeout
        if event::poll(next_update)? {
            // Drain all events to keep input snappy
            while event::poll(Duration::from_secs(0))? {
                match event::read()? {
                    Event::Key(key) => handle_key_event(&mut app, key),
                    Event::Mouse(mouse) => handle_mouse_event(&mut app, mouse),
                    Event::Resize(_, _) => {}, // Re-render on resize
                    _ => {}
                }
            }
        }

        // Logic Updates (Only when time thresholds are crossed)
        if last_animation.elapsed() >= animation_tick {
            app.update_frame();
            last_animation = Instant::now();
        }

        if last_second.elapsed() >= Duration::from_secs(1) {
            app.tick();
            last_second = Instant::now();
        }
    }

    // 5. Normal Cleanup
    tui::restore()?;
    Ok(())
}

fn handle_key_event(app: &mut App, key: event::KeyEvent) {
    if key.kind != KeyEventKind::Press { return; }

    match (key.code, key.modifiers, &app.state) {
        (KeyCode::Char('q'), _, _) | (KeyCode::Char('c'), KeyModifiers::CONTROL, _) => app.quit(),

        (KeyCode::Up, _, AppState::Menu) => app.prev_option(),
        (KeyCode::Down, _, AppState::Menu) => app.next_option(),
        (KeyCode::Enter, _, AppState::Menu) => app.enter_task_input(),

        (KeyCode::Char(c), _, AppState::TaskInput) => app.add_char(c),
        (KeyCode::Backspace, _, AppState::TaskInput) => app.remove_char(),
        (KeyCode::Enter, _, AppState::TaskInput) => app.start_session(),
        (KeyCode::Esc, _, AppState::TaskInput) => app.state = AppState::Menu,

        (KeyCode::Char(' '), _, AppState::Running) | (KeyCode::Char(' '), _, AppState::Paused) => app.toggle_pause(),

        _ => {}
    }
}

fn handle_mouse_event(app: &mut App, mouse: event::MouseEvent) {
    if app.state == AppState::Menu {
        match mouse.kind {
            MouseEventKind::ScrollDown => app.next_option(),
            MouseEventKind::ScrollUp => app.prev_option(),
            _ => {}
        }
    }
}
