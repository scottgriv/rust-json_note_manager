use crate::notes::{Note, load_notes, save_notes};

pub fn add_note(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: add <note_content>");
        return;
    }

    let mut notes = load_notes();
    let new_id = if notes.is_empty() {
        1
    } else {
        notes.last().unwrap().id + 1
    };

    notes.push(Note {
        id: new_id,
        content: args[0].clone(),
    });

    save_notes(&notes);
}

pub fn list_notes() {
    let notes = load_notes();
    for note in notes {
        println!("{}: {}", note.id, note.content);
    }
}

pub fn delete_note(args: &[String]) {
    if args.len() != 1 {
        eprintln!("Usage: delete <note_id>");
        return;
    }

    let id: u32 = match args[0].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Invalid ID");
            return;
        }
    };

    let mut notes = load_notes();
    if let Some(pos) = notes.iter().position(|n| n.id == id) {
        notes.remove(pos);
        save_notes(&notes);
    } else {
        eprintln!("Note with ID {} not found", id);
    }
}
