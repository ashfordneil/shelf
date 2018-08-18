extern crate http;
#[macro_use]
extern crate lazy_static;
extern crate tokio;
#[macro_use]
extern crate tower_web;
extern crate uuid;

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
