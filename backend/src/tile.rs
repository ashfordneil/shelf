//! Module for doing crud operations on the Tile itself.
use std::collections::HashMap;
use mvdb::Mvdb;
use std::path::Path;
use uuid::Uuid;
use auth::{Auth, AuthKey};

use std::io::prelude::*;
use std::fs::File;

#[derive(Default, Clone, Debug, Response, Extract, Serialize, Deserialize)]
pub struct Tile {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    tile_id: Uuid,
}

impl Tile {
    fn tile_storage() -> Mvdb<HashMap<Uuid, Tile>> {
        let path = "./target/tile.json";

        let file = Path::new(path);

        if !file.exists() {
            let mut f = File::create(path).unwrap();
            f.write_all(b"{}").unwrap();
            f.sync_all().unwrap();
            println!("Created: {:?}", path);
        }

        let storage: Mvdb<HashMap<Uuid, Tile>> = Mvdb::from_file(&file)
             .expect("File does not exist, or schema mismatch");
         storage.clone()
    }

    /// Get the internals of a Tile
    pub fn get(id: &Uuid) -> Option<Tile> {
        let store = Tile::tile_storage();
        let store = store.access(|db| db.clone())
            .expect("Failed to access tile file");
        store.get(id).cloned()
    }

    fn exists(tile_id: &Uuid) -> bool {
        let store = Tile::tile_storage(); 
        let store = store.access(|db| db.clone())
            .expect("Failed to access tile file");
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
            store.access_mut(|store| {
                if let Some(x) = store.get_mut(tile_id) {
                    *x = tile;
                }
            })
            .expect("Could not read tile file");

            Auth::unlock(key, jwt.clone())
        }
        else {
            Err(())
        }
    }

    /// Create a new Tile, and return a reference to it.
    pub fn post(new_tile: Tile) -> Uuid {
        let store = Tile::tile_storage();
        let mut retval: Option<Uuid> = None;
        store.access_mut(|store| {
            let uuid = loop {
                let uuid = Uuid::new_v4();
                if !store.contains_key(&uuid) {
                    break uuid;
                }
            };
            store.insert(uuid.clone(), new_tile);
            retval = Some(uuid.clone());
        })
        .expect("Could not access tile file");

        retval.unwrap()
    }

    pub fn delete(tile_id: &Uuid) -> Result<(), ()> {
        if Tile::exists(tile_id) {
            let authkey = AuthKey::Tile(tile_id.clone());
            if let Ok(jwt) = Auth::lock(authkey) {
                // TODO: Remove tile from boards

                let store = Tile::tile_storage();
                store.access_mut(|store| {
                    store.remove(&tile_id.clone());
                })
                .expect("Could not read tile file");

                Auth::unlock(authkey, jwt.clone()).unwrap();
                Ok(())
            }
            else {
                Err(())
            }
        }
        else {
            Err(())
        }
    }
}
