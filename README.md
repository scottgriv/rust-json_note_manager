<!-- Begin README -->

<div align="center">
    <a href="https://github.com/scottgriv/rust-json_note_manager" target="_blank">
        <img src="./docs/images/icon.png" width="200" height="200"/>
    </a>
</div>
<br>
<p align="center">
    <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.74.0-000000?style=for-the-badge&logo=rust" alt="Rust Badge" /></a>
    <br>
    <a href="https://github.com/scottgriv"><img src="https://img.shields.io/badge/github-follow_me-181717?style=for-the-badge&logo=github&color=181717" alt="GitHub Badge" /></a>
    <a href="mailto:scott.grivner@gmail.com"><img src="https://img.shields.io/badge/gmail-contact_me-EA4335?style=for-the-badge&logo=gmail" alt="Email Badge" /></a>
    <a href="https://www.buymeacoffee.com/scottgriv"><img src="https://img.shields.io/badge/buy_me_a_coffee-support_me-FFDD00?style=for-the-badge&logo=buymeacoffee&color=FFDD00" alt="BuyMeACoffee Badge" /></a>
    <br>
    <a href="https://prgoptimized.com" target="_blank"><img src="https://img.shields.io/badge/PRG-Bronze Project-CD7F32?style=for-the-badge&logo=data:image/svg%2bxml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBzdGFuZGFsb25lPSJubyI/Pgo8IURPQ1RZUEUgc3ZnIFBVQkxJQyAiLS8vVzNDLy9EVEQgU1ZHIDIwMDEwOTA0Ly9FTiIKICJodHRwOi8vd3d3LnczLm9yZy9UUi8yMDAxL1JFQy1TVkctMjAwMTA5MDQvRFREL3N2ZzEwLmR0ZCI+CjxzdmcgdmVyc2lvbj0iMS4wIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciCiB3aWR0aD0iMjYuMDAwMDAwcHQiIGhlaWdodD0iMzQuMDAwMDAwcHQiIHZpZXdCb3g9IjAgMCAyNi4wMDAwMDAgMzQuMDAwMDAwIgogcHJlc2VydmVBc3BlY3RSYXRpbz0ieE1pZFlNaWQgbWVldCI+Cgo8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSgwLjAwMDAwMCwzNC4wMDAwMDApIHNjYWxlKDAuMTAwMDAwLC0wLjEwMDAwMCkiCmZpbGw9IiNDRDdGMzIiIHN0cm9rZT0ibm9uZSI+CjxwYXRoIGQ9Ik0xMiAzMjggYy04IC04IC0xMiAtNTEgLTEyIC0xMzUgMCAtMTA5IDIgLTEyNSAxOSAtMTQwIDQyIC0zOCA0OAotNDIgNTkgLTMxIDcgNyAxNyA2IDMxIC0xIDEzIC03IDIxIC04IDIxIC0yIDAgNiAyOCAxMSA2MyAxMyBsNjIgMyAwIDE1MCAwCjE1MCAtMTE1IDMgYy04MSAyIC0xMTkgLTEgLTEyOCAtMTB6IG0xMDIgLTc0IGMtNiAtMzMgLTUgLTM2IDE3IC0zMiAxOCAyIDIzCjggMjEgMjUgLTMgMjQgMTUgNDAgMzAgMjUgMTQgLTE0IC0xNyAtNTkgLTQ4IC02NiAtMjAgLTUgLTIzIC0xMSAtMTggLTMyIDYKLTIxIDMgLTI1IC0xMSAtMjIgLTE2IDIgLTE4IDEzIC0xOCA2NiAxIDc3IDAgNzIgMTggNzIgMTMgMCAxNSAtNyA5IC0zNnoKbTExNiAtMTY5IGMwIC0yMyAtMyAtMjUgLTQ5IC0yNSAtNDAgMCAtNTAgMyAtNTQgMjAgLTMgMTQgLTE0IDIwIC0zMiAyMCAtMTgKMCAtMjkgLTYgLTMyIC0yMCAtNyAtMjUgLTIzIC0yNiAtMjMgLTIgMCAyOSA4IDMyIDEwMiAzMiA4NyAwIDg4IDAgODggLTI1eiIvPgo8L2c+Cjwvc3ZnPgo=" alt="Bronze" /></a>
</p>

---------------

<h1 align="center">Rust JSON Note Manager</h1>

A simple command-line tool for managing `JSON` Notes built using **Rust**. 

---------------

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Usage](#usage)
- [What's Inside?](#whats-inside)
- [Resources](#resources)
- [License](#license)
- [Credits](#credits)


## Features

The project will use the `serde` and `serde_json` crates for serialization and deserialization, allowing users to store notes in a `JSON` file.

## Getting Started

### Installation

1. Clone the Repository:
```bash
git clone https://github.com/scottgriv/rust-json_note_manager
```

2. Change to the Directory:
```bash
cd rust-json_note_manager
```

3. Build the Project:
```bash
cargo build --release
```

4. Run the Command:
```bash
cargo run -- <command> [arguments]
```

5. Example:
```bash
cargo run -- add "This is a new note"
```

## Usage

- Add a note: `cargo run -- add "Your note content"`
- List notes: `cargo run -- list`
- Delete a note by ID: `cargo run -- delete <note_id>`

> [!NOTE]
> Your notes will be stored in a file called `notes.json` in the same directory as the executable.

## What's Inside?

```bash
rust-note-manager # Root directory
├── src/ # Source code
│   ├── main.rs # For running the program
│   ├── notes.rs # For managing notes
│   └── commands.rs # For parsing command-line arguments
├── sample/ # Sample data
│   └── notes_sample_output.json # Sample data
├── target/ # Generated
├── notes.json # Generated notes file
├── Cargo.toml # Dependencies
├── .gitignore # git ignore file
├── LICENSE # License file
└── README.md # This file
```

## Resources

- [Rust](https://www.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust Documentation](https://doc.rust-lang.org/std/)
- [Rust Playground](https://play.rust-lang.org/)
- [Rust Crates](https://crates.io/)

## License

This project is released under the terms of **The Unlicense**, which allows you to use, modify, and distribute the code as you see fit. 
- [The Unlicense](https://choosealicense.com/licenses/unlicense/) removes traditional copyright restrictions, giving you the freedom to use the code in any way you choose.
- For more details, see the [LICENSE](LICENSE) file in this repository.

## Credits

**Author:** [Scott Grivner](https://github.com/scottgriv) <br>
**Email:** [scott.grivner@gmail.com](mailto:scott.grivner@gmail.com) <br>
**Website:** [scottgrivner.dev](https://www.scottgrivner.dev) <br>
**Reference:** [Main Branch](https://github.com/scottgriv/rust-json_note_manager) <br>

---------------

<div align="center">
    <a href="https://github.com/scottgriv" target="_blank">
        <img src="./docs/images/footer.png" width="100" height="100"/>
    </a>
</div>

<!-- End README -->
