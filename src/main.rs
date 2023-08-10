mod notes;
mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rust-note-manager <command> [arguments]");
        return;
    }

    match args[1].as_str() {
        "add" => commands::add_note(&args[2..]),
        "list" => commands::list_notes(),
        "delete" => commands::delete_note(&args[2..]),
        _ => eprintln!("Unknown command"),
    }
}
