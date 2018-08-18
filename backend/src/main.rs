#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Debug, Response)]
#[web(status = "201")]
struct MyData {
    foo: usize,
    bar: Option<String>,
}

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

        #[post("/request-body")]
        fn request_body(&self, body: Vec<u8>) -> Result<String, ()> {
            Ok(format!("We received {} bytes", body.len()))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}