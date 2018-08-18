//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use jwt::{encode, decode, Header, Algorithm, Validation};

#[derive(Default, Clone, Debug, Response, Extract)]
pub struct Board {
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

    fn auth_storage() -> Arc<Mutex<HashMap<Uuid, String>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<Uuid, String>>> = Default::default();
        }

        STORAGE.clone()
    }

    /// Get the internals of a board
    pub fn get(id: &Uuid) -> Option<Board> {
        let store = Board::board_storage();
        let store = store.lock().unwrap();
        store.get(id).cloned()
    }

    pub fn checkout(board_id: &Uuid) -> Option<String> {
        let store = Board::board_storage();
        let store = store.lock().unwrap();
        if let Some(board) = store.get(board_id).cloned() {
            let authstore = Board::auth_storage();
            let mut authstore = authstore.lock().unwrap();
            if authstore.contains_key(board_id) {
                return None;
            }

            let claims = JwtClaims {
                board_id: *board_id
            };
            let new_jwt = encode(&Header::default(), &claims, "secret".as_ref());
            if let Ok(new_jwt) = new_jwt {
                authstore.insert(board_id.clone(), new_jwt.to_string());
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

    pub fn checkin(board_id: &Uuid, jwt: String, board: Board) -> Result<(), ()> {
        let authstore = Board::auth_storage();
        let mut authstore = authstore.lock().unwrap();
        let mut entry = authstore.get(board_id);
        if let Some(stored_jwt) = entry {
            println!("stored JWT: {:?}", stored_jwt);
            println!("header JWT: {:?}", jwt);
        }
        Ok(())
    }

    /// Create a new board, and return a reference to it.
    pub fn post() -> Uuid {
        let store = Board::board_storage();
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
