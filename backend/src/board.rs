//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Default, Clone, Debug, Response)]
pub struct Board {
    /// All of the tiles in the board
    tiles: Vec<Uuid>,
}

impl Board {
    fn storage() -> Arc<Mutex<HashMap<Uuid, Board>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, Board>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a board
    pub fn get(id: &Uuid) -> Option<Board> {
        let store = Board::storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    /// Create a new board, and return a reference to it.
    pub fn post() -> Uuid {
        let store = Board::storage();
        let mut store = store.lock().unwrap();
        let uuid = loop {
            let uuid = Uuid::new_v4();
            if !store.contains_key(&uuid) {
                break uuid;
            }
        };
        store.insert(uuid.clone(), Default::default());
        uuid
    }
}
