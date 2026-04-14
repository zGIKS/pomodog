use anyhow::Result;
use crossterm::event::{self, Event};
use std::time::{Duration, Instant};

use crate::application::event_handler::{handle_key_event, handle_mouse_event};
use crate::domain::App;
use crate::infrastructure::terminal::Tui;
use crate::presentation::tui::render;

pub fn run(terminal: &mut Tui, mut app: App) -> Result<()> {
    let animation_tick = Duration::from_millis(150);
    let mut last_animation = Instant::now();
    let mut last_second = Instant::now();

    while !app.should_quit() {
        terminal.draw(|f| render(f, &app))?;

        let next_update = animation_tick
            .checked_sub(last_animation.elapsed())
            .unwrap_or(Duration::from_millis(10));

        if event::poll(next_update)? {
            let mut event_count = 0;
            const MAX_EVENTS_PER_FRAME: usize = 100;

            while event_count < MAX_EVENTS_PER_FRAME && event::poll(Duration::from_secs(0))? {
                event_count += 1;
                match event::read()? {
                    Event::Key(key) => handle_key_event(&mut app, key),
                    Event::Mouse(mouse) => handle_mouse_event(&mut app, mouse),
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }

        if last_animation.elapsed() >= animation_tick {
            app.update_frame();
            last_animation = Instant::now();
        }

        if last_second.elapsed() >= Duration::from_secs(1) {
            app.tick();
            last_second = Instant::now();
        }
    }

    Ok(())
}
