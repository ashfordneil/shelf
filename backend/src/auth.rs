//! Module for doing crud operations on the board itself.
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use uuid::Uuid;

use jwt::{encode, Header};

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json;

use mvdb::Mvdb;

#[derive(Default, Clone, Debug)]
pub struct Auth;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AuthKey {
    Board(Uuid),
    Tile(Uuid),
}

impl Serialize for AuthKey {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::Error;
        let intermediate = match self {
            AuthKey::Board(x) => (true, x),
            AuthKey::Tile(x) => (false, x),
        };
        let output = serde_json::to_string(&intermediate).map_err(S::Error::custom)?;
        output.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AuthKey {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        let string = String::deserialize(deserializer)?;
        let intermediate = serde_json::from_str(&string).map_err(D::Error::custom)?;
        let output = match intermediate {
            (true, x) => AuthKey::Board(x),
            (false, x) => AuthKey::Board(x),
        };
        Ok(output)
    }
}

type JwtString = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    key: AuthKey
}
impl Auth {
    fn storage() -> Mvdb<HashMap<AuthKey, JwtString>> {
        let path = "./target/auth.json";

        let file = Path::new(path);

        if !file.exists() {
            let mut f = File::create(path).unwrap();
            f.write_all(b"{}").unwrap();
            f.sync_all().unwrap();
            println!("Created: {:?}", path);
        }

        let storage: Mvdb<HashMap<AuthKey,JwtString>> = Mvdb::from_file(&file)
            .expect("File does not exist, or schema mismatch");
        storage.clone()
    }

    pub fn is_locked(key: AuthKey) -> bool {
        let store = Auth::storage();
        let store = store.access(|db| db.clone())
            .expect("Could not read Auth file");
        store.contains_key(&key)
    }

    pub fn lock(key: AuthKey) -> Result<String, String> {
        if !Auth::is_locked(key) {
            let store = Auth::storage();

            let claims = key.clone();
            let new_jwt = encode(&Header::default(), &claims, "secret".as_ref());
            if let Ok(new_jwt) = new_jwt {

                store.access_mut(|store_from_disk| 
                {
                    store_from_disk.insert(key.clone(), new_jwt.to_string());
                })
                .expect("Failed to access file");

                Ok(new_jwt.to_string())
            }
            else {
                Err("JWT doesn't match".into())
            }


        }
        else {
            Err("Item is locked".into())
        }
    }

    pub fn is_valid(key: AuthKey, jwt: String) -> bool {
        let store = Auth::storage();
        let store_from_disk = store.access(|db| db.clone())
             .expect("Failed to access file");
        let stored_jwt = match store_from_disk.get(&key) {
            Some(val) => Some(val.clone()),
            None => None
        };
        if let Some(stored_jwt) = stored_jwt {
            if jwt.eq(&stored_jwt) {
                return true;
            }
        }
        return false;
    }

    pub fn unlock(key: AuthKey, jwt: String) -> Result<(), String> {
        if Auth::is_valid(key, jwt) {
            let store = Auth::storage();
            store.access_mut(|store| {
                store.remove(&key);
            })
            .expect("Failed to access file");
            return Ok(());
        }
        else {
            return Err("Key is not valid".into());
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
