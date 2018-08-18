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
        if !Auth::is_locked(key) {
            let store = Auth::storage   ();
            let mut store = store.lock().unwrap();


            // let claims = JwtClaims {
            //     key: key
            // };

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

    pub fn unlock(key: AuthKey, jwt: String) -> Result<(), ()> {
        let store = Auth::storage();
        let stored_jwt = {
            let store = store.lock().unwrap();
            let entry = match store.get(&key) {
                Some(val) => Some(val.clone()),
                None => None
            };
            entry
        };
        if let Some(stored_jwt) = stored_jwt {
            if jwt.eq(&stored_jwt) {
                let store = Auth::storage();
                let mut store = store.lock().unwrap();
                store.remove(&key);
                return Ok(());
            }
            else {
                return Err(());
            }
        }
        Err(())
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
        let _jwt = Auth::lock(key);
        assert!(Auth::is_locked(key));
    }

    #[test]
    fn test_unlock() {
        let uuid = Uuid::new_v4();
        let key = AuthKey::Board(uuid);
        let jwt = Auth::lock(key);
        assert!(Auth::is_locked(key));
        let result = Auth::unlock(key, jwt.unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn test_unlock_jwt() {
        let key1 = AuthKey::Board(Uuid::new_v4());
        assert!(!Auth::is_locked(key1));
        let jwt1 = Auth::lock(key1);

        let key2 = AuthKey::Board(Uuid::new_v4());
        assert!(!Auth::is_locked(key2));
        let jwt2 = Auth::lock(key2);

        assert!(Auth::is_locked(key1));
        assert!(Auth::is_locked(key2));
        assert!(!Auth::unlock(key1, jwt2.unwrap()).is_ok());
        assert!(Auth::unlock(key1, jwt1.unwrap()).is_ok());
    }
}
