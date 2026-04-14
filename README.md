# PomDog

A TUI Pomodoro timer built with Rust using ratatui and crossterm.

## Features

- **3 Pomodoro Modes**: Classic (25/5), Focus (50/10), Quick (15/5)
- **ASCII Dog Animation**: Cute animated dog companion during sessions
- **Keyboard Controls**: Full keyboard navigation
- **Session Tracking**: Visual progress bar and timer
- **Error Validation**: Prevents empty task names

## Controls

| Key | Action |
|-----|--------|
| `↑` `↓` | Navigate menu |
| `Enter` | Select/start |
| `Esc` | Back to menu |
| `Space` | Pause/resume |
| `Backspace` | Delete character |
| `Ctrl+Backspace` | Delete word |
| `q` / `Ctrl+C` | Quit |

## Architecture

```
src/
├── domain/          # Core business logic (App, Timer, TaskName)
├── application/     # Event handling and runner loop
├── infrastructure/ # Terminal setup and raw mode
└── presentation/    # TUI components (menu, task_input, dog, etc.)
```

## Testing

```bash
cargo test
```

## Building

```bash
cargo build --release
```