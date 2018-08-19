//! Module for doing crud operations on the Tile itself.
use std::collections::HashMap;
use std::env;
use std::path::Path;

use mvdb::Mvdb;
use uuid::Uuid;
use auth::{Auth, AuthKey};

use board::Board;

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

lazy_static! {
    static ref ROOT_PATH: String = env::var("STORAGE").unwrap_or("./target".into());
}

impl Tile {
    fn tile_storage() -> Mvdb<HashMap<Uuid, Tile>> {
        lazy_static! {
            static ref STORAGE: Mvdb<HashMap<Uuid, Tile>> = {
                let path = format!("{}/tile.json", *ROOT_PATH);
                let file = Path::new(&path);

                if !file.exists() {
                    let mut f = File::create(&path).unwrap();
                    f.write_all(b"{}").unwrap();
                    f.sync_all().unwrap();
                }

                Mvdb::from_file(&file).expect("File does not exist, or schema mismatch")
            };
        }
         STORAGE.clone()
    }

    /// Get the internals of a Tile
    pub fn get(id: &Uuid) -> Option<Tile> {
        let store = Tile::tile_storage();
        store.access(|db| db.get(id).cloned())
            .expect("Failed to access tile file")
    }

    fn exists(tile_id: &Uuid) -> bool {
        let store = Tile::tile_storage(); 
        store.access(|db| db.contains_key(tile_id))
            .expect("Failed to access tile file")
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

    pub fn checkin(tile_id: &Uuid, jwt: String, tile: Tile) -> Result<(), String> {
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
            Err("Key not valid".into())
        }
    }

    /// Create a new Tile, and return a reference to it.
    pub fn post(new_tile: Tile) -> Uuid {
        let store = Tile::tile_storage();
        store.access_mut(|store| {
            let uuid = loop {
                let uuid = Uuid::new_v4();
                if !store.contains_key(&uuid) {
                    break uuid;
                }
            };
            store.insert(uuid.clone(), new_tile);
            uuid.clone()
        })
        .expect("Could not access tile file")
    }

    pub fn delete(tile_id: &Uuid) -> Result<(), String> {
        if Tile::exists(tile_id) {
            let authkey = AuthKey::Tile(tile_id.clone());
            if let Ok(jwt) = Auth::lock(authkey) {
                // TODO: Remove tile from boards

                let boardstore = Board::board_storage();
                let boardstore = boardstore.access(|db| db.clone())
                    .expect("Could not read Board file");

                for (bkey, bval) in boardstore {
                    if bval.tiles.contains(tile_id) {
                        let jwt = Board::checkout(&bkey).unwrap();
                        let mut bval = bval.clone();
                        bval.tiles.retain(|t| t != tile_id);
                        let _res = Board::checkin(&bkey, jwt, bval);
                    }
                }

                let store = Tile::tile_storage();
                store.access_mut(|store| {
                    store.remove(&tile_id.clone());
                })
                .expect("Could not read tile file");

                Auth::unlock(authkey, jwt.clone()).unwrap();
                Ok(())
            }
            else {
                Err("Could not lock".into())
            }
        }
        else {
            Err("Tile doesn't exist".into())
        }
    }
}
