//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::env;
use mvdb::Mvdb;
use std::path::Path;
use uuid::Uuid;

use auth::{Auth, AuthKey};

use std::io::prelude::*;
use std::fs::File;

#[derive(Clone, Debug, Response, Extract, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub title: String,
    /// All of the tiles in the board
    pub tiles: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    board_id: Uuid,
}

lazy_static! {
    static ref ROOT_PATH: String = env::var("STORAGE").unwrap_or("./target".into());
}


impl Board {
    pub fn board_storage() -> Mvdb<HashMap<Uuid, Board>> {

        lazy_static! {
            static ref STORAGE: Mvdb<HashMap<Uuid, Board>> = {
                let path = format!("{}/board.json", *ROOT_PATH);
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

    /// Get the internals of a board
    pub fn get(id: &Uuid) -> Option<Board> {
        let store = Board::board_storage();
        let store = store.access(|db| db.clone())
            .expect("Could not read Board file");
        store.get(id).cloned()
    }

    fn exists(board_id: &Uuid) -> bool {
        let store = Board::board_storage();
        store.access(|db| db.contains_key(board_id))
            .expect("Could not read Board file")
    }

    pub fn checkout(board_id: &Uuid) -> Option<String> {
        if Board::exists(board_id) {
            if let Ok(jwt) = Auth::lock(AuthKey::Board(*board_id)) {
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

    pub fn checkin(board_id: &Uuid, jwt: String, board: Board) -> Result<(), String> {
        let key = AuthKey::Board(*board_id);

        if Auth::is_valid(key, jwt.clone()) {
            let store = Board::board_storage();
            store.access_mut(|store| {
                if let Some(x) = store.get_mut(board_id) {
                    *x = board;
                }
            })
            .expect("Could not read board file");


            Auth::unlock(key, jwt.clone())
        }
        else {
            Err("Key is not valid".into())
        }
    }

    /// Create a new board, and return a reference to itr.
    pub fn post(board: Board) -> Uuid {
        let store = Board::board_storage();
        let mut retval: Option<Uuid> = None;
        store.access_mut(|store| {
            let uuid = loop {
                let uuid = Uuid::new_v4();
                if !store.contains_key(&uuid) {
                    break uuid;
                }
            };
            store.insert(uuid.clone(), board);
            retval = Some(uuid.clone());
        })
        .expect("Could not access board file");

        retval.unwrap()
    }
}
