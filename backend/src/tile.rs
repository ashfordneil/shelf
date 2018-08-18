//! Module for doing crud operations on the Tile itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

#[derive(Default, Clone, Debug, Response, Extract)]
pub struct Tile {
    pub content: String,
}

impl Tile {
    fn storage() -> Arc<Mutex<HashMap<Uuid, Tile>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, Tile>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a Tile
    pub fn get(id: &Uuid) -> Option<Tile> {
        let store = Tile::storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    /// Create a new Tile, and return a reference to it.
    pub fn post(new_tile: Tile) -> Uuid {
        let store = Tile::storage();
        let mut store = store.lock().unwrap();
        let uuid = loop {
            let uuid = Uuid::new_v4();
            if !store.contains_key(&uuid) {
                break uuid;
            }
        };
        store.insert(uuid.clone(), new_tile);
        uuid
    }
}
