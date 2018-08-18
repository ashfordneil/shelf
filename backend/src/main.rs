extern crate http;
#[macro_use]
extern crate lazy_static;
extern crate tokio;
#[macro_use]
extern crate tower_web;
extern crate uuid;
extern crate jsonwebtoken as jwt;

use tower_web::ServiceBuilder;

use uuid::Uuid;

mod static_file;

mod board;
mod tile;

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

        #[post("/board")]
        #[content_type("json")]
        fn post_board(&self) -> Result<UuidWrapper, ()> {
            Ok(UuidWrapper(Board::post()))
        }

        #[post("/board/:id")]
        fn checkout_board(&self, id: String) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Board::checkout(&id).ok_or(())
        }


        #[patch("/board/:id")]
        fn checkin_board(&self, id: String, jwt: String, body: Board) -> Result<String, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            // let jwt = "".to_string();
            let resp = Board::checkin(&id, jwt, body);
            if let Ok(_) = resp {
                Ok("ok".to_string().to_string())
            }
            else {
                Err(())
            }
        }

        #[post("/tile")]
        #[content_type("json")]
        fn post_tile(&self, body: Tile) -> Result<UuidWrapper, ()> {
            Ok(UuidWrapper(Tile::post(body)))
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
        let board_id = Board::post();

        let jwt = Board::checkout(&board_id).unwrap();

        let tile = Tile {
            content: "heya".to_string()
        };

        let tile_id = Tile::post(tile);

        let board = Board {
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id, jwt, board);

        assert!(result.is_ok());
    }

    #[test]
    fn test_lock_board() {
        let board_id = Board::post();

        assert!(Board::checkout(&board_id).is_some());
        assert!(Board::checkout(&board_id).is_none());
        assert!(Board::checkout(&board_id).is_none());
        assert!(Board::checkout(&board_id).is_none());
    }

    #[test]
    fn test_lock_removed() {
        let board_id = Board::post();

        let jwt = Board::checkout(&board_id).unwrap();

        // Ensure that the lock is in place
        assert!(Board::checkout(&board_id).is_none());

        let tile = Tile {
            content: "heya".to_string()
        };

        let tile_id = Tile::post(tile);

        let board = Board {
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id, jwt, board);

        assert!(result.is_ok());

        // Ensure that the lock is no longer in place
        assert!(Board::checkout(&board_id).is_some());
    }

    #[test]
    fn test_correct_jwt() {
        let board_id1 = Board::post();
        let jwt1 = Board::checkout(&board_id1).unwrap();

        let board_id2 = Board::post();
        let jwt2 = Board::checkout(&board_id2).unwrap();

        let tile_id = Tile::post(Tile {
            content: "heya".to_string()
        });

        let board = Board {
            tiles: vec![tile_id]
        };

        let result = Board::checkin(&board_id1, jwt2, board);

        assert!(result.is_err());
    }
}