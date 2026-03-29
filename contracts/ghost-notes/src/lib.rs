#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub visit_count: u32,
}

const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

#[contract]
pub struct GhostNotesContract;

#[contractimpl]
impl GhostNotesContract {
    /// Get all notes from storage
    pub fn get_all_notes(env: Env) -> Vec<Note> {
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env))
    }

    /// Create a new ghost note
    pub fn create_note(env: Env, title: String, content: String) -> u64 {
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        
        let id = env.prng().gen::<u64>();
        let note = Note {
            id,
            title,
            content,
            visit_count: 0,
        };
        
        notes.push_back(note);
        env.storage().instance().set(&NOTE_DATA, &notes);
        
        id
    }

    /// View a note. This changes the note's content every time it's called!
    pub fn view_note(env: Env, id: u64) -> Note {
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        let mut found_index = None;

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                found_index = Some(i);
                break;
            }
        }

        if let Some(i) = found_index {
            let mut note = notes.get(i).unwrap();
            note.visit_count += 1;
            
            // The "Ghost" effect: Content changes based on visit count
            let new_content = if note.visit_count == 1 {
                String::from_str(&env, "First glance... the secret is safe.")
            } else if note.visit_count == 2 {
                String::from_str(&env, "Wait, did this change? Something is wrong.")
            } else if note.visit_count == 3 {
                String::from_str(&env, "THE GHOST IS HERE. LEAVE NOW.")
            } else {
                String::from_str(&env, "...) Only silence remains (... ")
            };

            note.content = new_content;
            notes.set(i, note.clone());
            env.storage().instance().set(&NOTE_DATA, &notes);
            
            note
        } else {
            panic!("Note not found");
        }
    }

    /// Delete a note
    pub fn delete_note(env: Env, id: u64) {
        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
        
        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);
                env.storage().instance().set(&NOTE_DATA, &notes);
                return;
            }
        }
    }
}