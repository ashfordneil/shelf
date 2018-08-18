extern crate http;
#[macro_use]
extern crate mvdb;
extern crate tokio;
#[macro_use]
extern crate tower_web;
extern crate uuid;
extern crate jsonwebtoken as jwt;
extern crate serde;

use tower_web::ServiceBuilder;

use uuid::Uuid;

mod static_file;

mod board;
mod tile;
mod auth;

use board::Board;
use tile::Tile;

#[derive(Debug, Default, Clone)]
struct DataHandler;

#[derive(Debug, Extract, Response)]
struct UuidWrapper(Uuid);

impl_web! {
    impl DataHandler {
        #[get("/board/:id")]
        #[content_type("json")]
        fn get_board(&self, id: String) -> Result<Board, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Board::get(&id).ok_or(())
        }

        #[post("/board/:name")]
        #[content_type("json")]
        fn post_board(&self, name: String) -> Result<UuidWrapper, ()> {
            Ok(UuidWrapper(Board::post(name)))
        }

        #[post("/board/:id/edit")]
        fn checkout_board(&self, id: String) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Board::checkout(&id).ok_or(())
        }


        #[patch("/board/:id")]
        fn checkin_board(&self, id: String, auth: String, body: Board) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            // let jwt = "".to_string();
            let resp = Board::checkin(&id, auth, body);
            if let Ok(_) = resp {
                Ok("ok".to_string().to_string())
            }
            else {
                Err(())
            }
        }

        #[get("/tile/:id")]
        #[content_type("json")]
        fn get_tile(&self, id: String) -> Result<Tile, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Tile::get(&id).ok_or(())
        }

        #[post("/tile")]
        #[content_type("json")]
        fn post_tile(&self, body: Tile) -> Result<UuidWrapper, ()> {
            Ok(UuidWrapper(Tile::post(body)))
        }

        #[post("/tile/:id")]
        fn checkout_tile(&self, id: String) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Tile::checkout(&id).ok_or(())
        }

        #[patch("/tile/:id")]
        fn checkin_tile(&self, id: String, jwt: String, body: Tile) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            let resp = Tile::checkin(&id, jwt, body);
            if let Ok(_) = resp {
                Ok("ok".to_string().to_string())
            }
            else {
                Err(())
            }
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(DataHandler)
        .resource(static_file::StaticFile)
        .run(&addr)
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_board() {
        let board_id = Board::post("Hello".into());

        let jwt = Board::checkout(&board_id).unwrap();

        let tile = Tile {
            title: "New tile".to_string(),
            content: "heya".to_string()
        };

        let tile_id = Tile::post(tile);

        let board = Board {
            title: "New Board".to_string(),
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id, jwt, board.clone());

        assert!(result.is_ok());

        let gotten_board = Board::get(&board_id).unwrap();
        assert_eq!(board, gotten_board);
    }

    #[test]
    fn test_lock_board() {
        let board_id = Board::post("Hello".into());

        assert!(Board::checkout(&board_id).is_some());
        assert!(Board::checkout(&board_id).is_none());
        assert!(Board::checkout(&board_id).is_none());
        assert!(Board::checkout(&board_id).is_none());
    }

    #[test]
    fn test_lock_removed() {
        let board_id = Board::post("Hello".into());

        let jwt = Board::checkout(&board_id).unwrap();

        // Ensure that the lock is in place
        assert!(Board::checkout(&board_id).is_none());

        let tile = Tile {
            title: "New tile".to_string(),
            content: "heya".to_string()
        };

        let tile_id = Tile::post(tile);

        let board = Board {
            title: "New Board".to_string(),
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id, jwt, board);

        assert!(result.is_ok());

        // Ensure that the lock is no longer in place
        assert!(Board::checkout(&board_id).is_some());
    }

    #[test]
    fn test_correct_jwt() {
        let board_id1 = Board::post("Hello".into());
        let _jwt1 = Board::checkout(&board_id1).unwrap();

        let board_id2 = Board::post("Hello".into());
        let jwt2 = Board::checkout(&board_id2).unwrap();

        let tile_id = Tile::post(Tile {
            title: "New tile".to_string(),
            content: "heya".to_string()
        });

        let board = Board {
            title: "New Board".to_string(),
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id1, jwt2, board);

        assert!(result.is_err());
    }
}
