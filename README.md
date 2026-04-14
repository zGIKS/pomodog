# POMODOG

![POMODOG](https://i.imgur.com/8lYTeu7.png)

**Pomodog** is a minimalist, high-performance Pomodoro CLI tool built with Rust. It features a cute ASCII dog companion to keep you focused during your deep work sessions.

## Why Pomodog?

Most Pomodoro apps are either too complex or live in a browser tab that distracts you. **Pomodog** solves this by:
- **Staying in your Terminal**: No tab-switching, no distractions.
- **Extreme Efficiency**: Consumes < 4MB of RAM and 0% CPU when idle.
- **Persistence**: Your session is automatically saved. If you close the terminal or it crashes, you can resume exactly where you left off.
- **Cute Motivation**: An animated ASCII dog that "works" and "rests" with you.

---

## Architecture & Design

The project follows **Domain-Driven Design (DDD)** principles and a clean separation of concerns, avoiding "God Object" patterns.

### 1. Domain Layer (`src/domain/`)
- **`Session`**: The core "aggregate". It manages the Pomodoro logic (Work vs. Break phases), the internal `Timer`, and the `TaskName`.
- **`App`**: A UI coordinator. It handles application states (`Menu`, `TaskInput`, `Running`, `Paused`), manages user navigation, and orchestrates the transition between the UI and the domain `Session`.
- **`Value Objects`**: Immutable-like structures such as `Timer`, `Phase`, and `TaskName` that ensure the internal state is always valid.

### 2. Application Layer (`src/application/`)
- **`Runner`**: The engine of the app. It handles the 150ms animation tick, the 1s logic tick, and the event polling loop. It is also responsible for triggering the persistence auto-saves.
- **`EventHandler`**: Decouples keyboard/mouse input from the `App` logic, translating raw events into domain actions.

### 3. Infrastructure Layer (`src/infrastructure/`)
- **`Persistence`**: Abstracted via a trait. Currently implemented as `TomlPersistence`, which saves sessions to the standard XDG config directory for your OS.
- **`Terminal`**: Manages the raw mode and TUI initialization/cleanup.

### 4. Presentation Layer (`src/presentation/`)
- **Functional Rendering**: Uses `ratatui` for an immediate-mode UI. Components are stateless and purely represent the current domain state.

---

## Persistence Mechanism

Pomodog features a robust persistence system:
- **Storage**: Sessions are stored in `~/.config/pomodog/session.toml` (on Linux).
- **Auto-Save**: The `Runner` saves the state every second while a session is active.
- **Resume Logic**: On startup, if a saved session is detected, Pomodog prompts you to "RESUME PREVIOUS". If you decline, the old session is safely deleted to avoid stale data.
- **Trait-Based**: The persistence is decoupled via the `Persistence` trait, making it easy to swap the TOML backend for a database or cloud sync in the future.

---

## Nix Support

This project is fully "Nixified" with **Nix Flakes**.

- **Build**: `nix build`
- **Run**: `nix run`
- **Development**: `nix develop` (provides a complete Rust toolchain, `rust-analyzer`, and `pkg-config`).

---

## Controls

| Key | Action |
|-----|--------|
| `↑` `↓` | Navigate menu / Scroll |
| `Enter` | Select option / Start session |
| `Esc` | Return to menu |
| `Space` | Toggle Pause/Resume |
| `Backspace` | Delete character in input |
| `Ctrl+BS` | Delete full word in input |
| `q` / `Ctrl+C` | Quit |

---

## Development

### Prerequisites
- Rust (Edition 2024)
- Cargo

### Commands
```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Build optimized release binary
cargo build --release

# Run Clippy (Linter)
cargo clippy
```

---

## License
MIT © [zGIKS](https://github.com/zGIKS)
