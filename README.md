# Rust JSON Note Manager
A simple command-line tool for managing `JSON` Notes built using **Rust**. 

The project will use the `serde` and `serde_json` crates for serialization and deserialization, allowing users to store notes in a `JSON` file.

## Commands
- Add a note: `cargo run -- add "Your note content"`
- List notes: `cargo run -- list`
- Delete a note by ID: `cargo run -- delete <note_id>`

## Getting Started

### Clone the Repository:
```bash
git clone https://github.com/scottgriv/rust-json_note_manager
```

### Change to the Directory:
```bash
cd rust-json_note_manager
```

### Build the Project:
```bash
cargo build --release
```

### Run the Command:
```bash
cargo run -- <command> [arguments]
```

Example:
```bash
cargo run -- add "This is a new note"
```

Your notes will be stored in a file called `notes.json` in the same directory as the executable.

## What's Inside?
```
rust-note-manager/ (root)
├── src/ (source code)
│   ├── main.rs (for running the program)
│   ├── notes.rs (for managing notes)
│   └── commands.rs (for parsing command-line arguments)
├── sample/ (sample data)
│   └── notes_sample_output.json (aample data)
├── target/ (generated)
├── notes.json (generated notes file)
├── Cargo.toml (dependencies)
├── .gitignore (git ignore file)
├── LICENSE (MIT)
└── README.md (this file)
```

## Credit
**Author:** Scott Grivner <br>
**Email:** scott.grivner@gmail.com <br>
**Website:** [scottgrivner.dev](https://www.scottgriv.dev) <br>
**Reference:** [Main Branch](https://github.com/scottgriv/rust-json_note_manager)
