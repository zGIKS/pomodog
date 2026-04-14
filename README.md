# POMODOG

![POMODOG](https://i.imgur.com/eXUcFEH.png)

**Pomodog** is a minimalist, high-performance Pomodoro CLI tool built with Rust. It features a cute ASCII dog companion to keep you focused during your deep work sessions.

## Features

- Terminal-native workflow with no browser distractions.
- Built-in presets: `Classic (25/5)`, `Focus (50/10)`, `Quick (15/5)`.
- Instant `Space` pause/resume toggle during active sessions.
- Automatic persistence and resume flow after restart.
- Clean architecture with separated domain/application/infrastructure/presentation layers.

## Quick Start

### Prerequisites

- Rust toolchain (Edition 2024)
- Cargo

### Run in development

```bash
cargo run
```

### Build release binary

```bash
cargo build --release
```

### Test and lint

```bash
cargo test
cargo clippy
```

## Controls

| Key | Action |
|-----|--------|
| `↑` / `↓` | Navigate menu |
| Mouse wheel | Navigate menu |
| `Enter` | Select option / Start session from task input |
| `Esc` | Return to menu (from task input) |
| `Space` | Toggle pause/resume while running |
| `Backspace` | Delete one character in task input |
| `Ctrl+Backspace` | Delete one word in task input |
| `q` / `Ctrl+C` | Quit |

## Usage Flow

1. Select a session preset in the main menu.
2. Enter your task name.
3. Press `Enter` to start.
4. Pomodog alternates automatically between work and break phases.
5. Press `Space` to pause/resume at any moment.

Note: work/break phases loop continuously until you quit the app.

## Persistence

- Backend: TOML (`TomlPersistence`).
- File location (Linux): `~/.config/pomodog/session.toml`.
- Auto-save: every second while app state is `Running` or `Paused`.
- Resume flow: startup menu shows `RESUME PREVIOUS` when a session exists.
- Cleanup: the saved file is removed when no session needs to be preserved.

## Architecture

- `src/domain/`: core models and rules (`App`, `Session`, `Timer`, `TaskName`, `Phase`).
- `src/application/`: input/event handling and main loop (`event_handler`, `runner`).
- `src/infrastructure/`: terminal setup/restore and persistence adapter.
- `src/presentation/`: `ratatui` rendering and stateless UI components.

## Nix Support

```bash
nix build
nix run
nix develop
```

## License

MIT © [zGIKS](https://github.com/zGIKS)
