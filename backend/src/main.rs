#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tower_web;
extern crate uuid;

use tower_web::ServiceBuilder;

use uuid::Uuid;

mod board;
mod tile;

use board::Board;
use tile::Tile;

#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Debug, Response)]
#[web(status = "201")]
struct MyData {
    foo: usize,
    bar: Option<String>,
}

#[derive(Debug, Default, Clone)]
struct DataHandler;

#[derive(Debug, Extract, Response)]
struct UuidWrapper(Uuid);

impl_web! {
    // impl HelloWorld {
    //     #[get("/")]
    //     fn hello_world(&self) -> Result<String, ()> {
    //         Ok("Hello world".to_string())
    //     }

    //     #[get("/healthz")]
    //     fn health(&self) -> Result<String, ()> {
    //         Ok("ok".to_string())
    //     }

    //     #[get("/one/:param")]
    //     fn path_str(&self, param: String) -> Result<String, ()> {
    //         Ok(format!("We received: {} in the path", param))
    //     }

    //     #[get("/data")]
    //     #[content_type("json")]
    //     fn greet(&self) -> Result<MyData, ()> {
    //         Ok(MyData {
    //             foo: 123,
    //             bar: None,
    //         })
    //     }

    //     #[post("/data")]
    //     fn greet2(&self, body: MyData2) -> Result<String, ()> {
    //         Ok(format!("Hello, {:?}", body))
    //     }

    //     #[post("/request-body")]
    //     fn request_body(&self, body: Vec<u8>) -> Result<String, ()> {
    //         Ok(format!("We received {} bytes", body.len()))
    //     }

    //     #[post("/create")]
    //     #[content_type("application/json")]
    //     fn create(&self) -> Result<CreatedResponse, ()> {
    //         Ok(CreatedResponse {
    //             message: "created",
    //             x_my_header: "awesome",
    //         })
    //     }


    // }

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
        .run(&addr)
        .unwrap();
}
