# DnD Character Creator (WIP)

A Rust-based desktop application for creating and managing Dungeons & Dragons character sheets.

## Project Structure

```
dnd-character-creator/
├── src/
│   ├── main.rs                  # Application entry point
│   ├── app.rs                   # Main application logic
│   ├── lib.rs                   # Library root
│   ├── bin/
│   │   └── api_cli.rs           # Command-line API client (used as a debug tool, will be replaced by tests!)
│   ├── api/
│   │   ├── mod.rs               # API module
│   │   ├── client.rs            # API client logic
│   │   ├── models.rs            # API data models
│   │   └── cache.rs             # API cache logic
│   ├── ui/
│   │   ├── mod.rs               # UI module
│   │   ├── character_sheet.rs   # Character sheet UI components
│   │   └── widgets.rs           # Custom widgets
│   ├── models/
│   │   ├── mod.rs               # Models module
│   │   └── character.rs         # Character data structures
│   └── utils/
│       ├── mod.rs               # Utilities module
│       └── constants.rs         # Application constants
├── assets/                      # Static assets (images, icons, etc.)
├── Cargo.toml                   # Project dependencies and metadata
├── Cargo.lock                   # Dependency lock file
└── README.md                    # This file
```

## Features (Planned/not implemented)

- A4-sized window interface
- Character sheet with multiple panes
- Basic character information management
- Ability to save and load characters

## Dependencies

- [eframe](https://crates.io/crates/eframe) (egui framework) for the UI
- [serde](https://crates.io/crates/serde) for serialization/deserialization
- [serde_json](https://crates.io/crates/serde_json) for working with JSON
- [rfd](https://crates.io/crates/rfd) for file dialogs
- [reqwest](https://crates.io/crates/reqwest) for HTTP requests
- [tokio](https://crates.io/crates/tokio) for async runtime

## Building and Running (this "testing tool" is all that is supported currently)

```bash
cargo clean && cargo build
```

```bash
cargo run --bin api_cli 
```

## Build the main app (WIP - does nothing for now)

```bash
cargo run
```
