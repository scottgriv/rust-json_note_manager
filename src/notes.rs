use serde::{Serialize, Deserialize};
use std::fs;

const FILE_NAME: &str = "notes.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: u32,
    pub content: String,
}

pub fn load_notes() -> Vec<Note> {
    if let Ok(data) = fs::read_to_string(FILE_NAME) {
        if let Ok(notes) = serde_json::from_str(&data) {
            return notes;
        }
    }
    Vec::new()
}

pub fn save_notes(notes: &Vec<Note>) {
    let data = serde_json::to_string(notes).unwrap();
    fs::write(FILE_NAME, data).expect("Unable to write to file");
}
