#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tower_web;
extern crate uuid;

use tower_web::ServiceBuilder;

use uuid::Uuid;

mod board;

use board::Board;

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
    impl HelloWorld {
        #[get("/")]
        fn hello_world(&self) -> Result<String, ()> {
            Ok("Hello world".to_string())
        }

        #[get("/healthz")]
        fn health(&self) -> Result<String, ()> {
            Ok("ok".to_string())
        }

        #[get("/one/:param")]
        fn path_str(&self, param: String) -> Result<String, ()> {
            Ok(format!("We received: {} in the path", param))
        }

        #[get("/data")]
        #[content_type("json")]
        fn greet(&self) -> Result<MyData, ()> {
            Ok(MyData {
                foo: 123,
                bar: None,
            })
        }
    }

    impl DataHandler {
        #[get("/board/:id")]
        #[content_type("json")]
        fn get(&self, id: String) -> Result<Board, ()> {
            let id = Uuid::parse_str(&id).map_err(|e| {
                println!("{:?}", e);
            })?;
            Board::get(&id).ok_or(())
        }

        #[post("/board")]
        #[content_type("json")]
        fn post(&self) -> Result<UuidWrapper, ()> {
            Ok(UuidWrapper(Board::post()))
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
