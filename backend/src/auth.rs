//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use jwt::{encode, Header};

#[derive(Default, Clone, Debug)]
pub struct Auth;

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Copy)]
#[serde(tag = "type", content = "id")]
pub enum AuthKey {
    Board(Uuid),
    Tile(Uuid),
}

type JwtString = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    key: AuthKey
}

impl Auth {
    fn storage() -> Arc<Mutex<HashMap<AuthKey, JwtString>>> {
        lazy_static! {
            static ref STORAGE: Arc<Mutex<HashMap<AuthKey, JwtString>>> = Default::default();
        }

        STORAGE.clone()
    }

    pub fn is_locked(key: AuthKey) -> bool {
        let store = Auth::storage();
        let store = store.lock().unwrap();
        store.contains_key(&key)
    }

    pub fn lock(key: AuthKey) -> Result<String, ()> {
        if (!Auth::is_locked(key)) {
            let store = Auth::storage   ();
            let mut store = store.lock().unwrap();


            let claims = JwtClaims {
                key: key
            };

            let claims = key.clone();
            println!("hi");
            let new_jwt = encode(&Header::default(), &claims, "secret".as_ref());
            if let Ok(new_jwt) = new_jwt {
                store.insert(key.clone(), new_jwt.to_string());
                println!("{:?}", new_jwt.to_string());
                Ok(new_jwt.to_string())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_locked() {
        // let uuid = "cbeba719-29dd-4758-9b58-1d9e3b2894d6";
        let uuid = Uuid::new_v4();
        let key = AuthKey::Board(uuid);
        let jwt = Auth::lock(key);
        println!("{:?}", jwt.unwrap());
    }
}
