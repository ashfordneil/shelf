//! Module for doing crud operations on the Tile itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use auth::{Auth, AuthKey};

#[derive(Default, Clone, Debug, Response, Extract)]
pub struct Tile {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    tile_id: Uuid,
}

impl Tile {
    fn tile_storage() -> Arc<Mutex<HashMap<Uuid, Tile>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, Tile>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a Tile
    pub fn get(id: &Uuid) -> Option<Tile> {
        let store = Tile::tile_storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    fn exists(tile_id: &Uuid) -> bool {
        let store = Tile::tile_storage();
        let store = store.lock().unwrap();
        store.get(tile_id).cloned().is_some()
    }

    pub fn checkout(tile_id: &Uuid) -> Option<String> {
        if Tile::exists(tile_id) {
            if let Ok(jwt) = Auth::lock(AuthKey::Tile(*tile_id)) {
                Some(jwt.to_string())
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    pub fn checkin(tile_id: &Uuid, jwt: String, tile: Tile) -> Result<(), ()> {
        let key = AuthKey::Tile(*tile_id);

        if Auth::is_valid(key, jwt.clone()) {
            let store = Tile::tile_storage();
            let mut store = store.lock().unwrap();
            if let Some(x) = store.get_mut(tile_id) {
                *x = tile;
            }

            Auth::unlock(key, jwt.clone())
        }
        else {
            Err(())
        }
    }

    /// Create a new Tile, and return a reference to it.
    pub fn post(new_tile: Tile) -> Uuid {
        let store = Tile::tile_storage();
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
