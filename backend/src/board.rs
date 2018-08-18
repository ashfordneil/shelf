//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use jwt::{encode, Header};

use auth::{Auth, AuthKey};

#[derive(Clone, Debug, Response, Extract, PartialEq)]
pub struct Board {
    pub title: String,
    /// All of the tiles in the board
    pub tiles: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    board_id: Uuid,
}

impl Board {
    fn board_storage() -> Arc<Mutex<HashMap<Uuid, Board>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, Board>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a board
    pub fn get(id: &Uuid) -> Option<Board> {
        let store = Board::board_storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    fn exists(board_id: &Uuid) -> bool {
        let store = Board::board_storage();
        let store = store.lock().unwrap();
        store.get(board_id).cloned().is_some()
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

    pub fn checkin(board_id: &Uuid, jwt: String, board: Board) -> Result<(), ()> {
        let key = AuthKey::Board(*board_id);

        if Auth::is_valid(key, jwt.clone()) {
            let store = Board::board_storage();
            let mut store = store.lock().unwrap();
            if let Some(x) = store.get_mut(board_id) {
                *x = board;
            }

            Auth::unlock(key, jwt.clone())
        }
        else {
            Err(())
        }
    }

    /// Create a new board, and return a reference to it.
    pub fn post(title: String) -> Uuid {
        let store = Board::board_storage();
        let mut store = store.lock().unwrap();
        let uuid = loop {
            let uuid = Uuid::new_v4();
            if !store.contains_key(&uuid) {
                break uuid;
            }
        };
        store.insert(uuid.clone(), Board { title, tiles: Vec::new() });
        uuid
    }

    
}
