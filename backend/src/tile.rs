//! Module for doing crud operations on the Tile itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use jwt::{encode, decode, Header, Algorithm, Validation};

#[derive(Default, Clone, Debug, Response, Extract)]
pub struct Tile {
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

    fn auth_storage() -> Arc<Mutex<HashMap<Uuid, String>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, String>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a Tile
    pub fn get(id: &Uuid) -> Option<Tile> {
        let store = Tile::tile_storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    pub fn checkout(tile_id: &Uuid) -> Option<String> {
        let store = Tile::tile_storage();
        let store = store.lock().unwrap();
        if let Some(tile) = store.get(tile_id).cloned() {
            let authstore = Tile::auth_storage();
            let mut authstore = authstore.lock().unwrap();
            if authstore.contains_key(tile_id) {
                return None;
            }

            let claims = JwtClaims {
                tile_id: *tile_id
            };
            let new_jwt = encode(&Header::default(), &claims, "secret".as_ref());
            if let Ok(new_jwt) = new_jwt {
                authstore.insert(tile_id.clone(), new_jwt.to_string());
                Some(new_jwt.to_string())
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
        let authstore = Tile::auth_storage();
        let stored_jwt = {
            let authstore = authstore.lock().unwrap();
            // TODO: Add error checking
            let entry = authstore.get(tile_id).unwrap().clone();
            entry
        };
        if (jwt.eq(&stored_jwt)) {
            let mut authstore = authstore.lock().unwrap();
            authstore.remove(tile_id);
        }
        else {
            return Err(());
        }
        Ok(())
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
