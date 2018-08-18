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

#[derive(Debug, Extract)]
struct MyData2 {
    foo: usize,
    bar: Option<String>,
}


#[derive(Debug, Response)]
#[web(status = "201")]
struct CreatedResponse {
    message: &'static str,

    /// This specifies that the value of this field should be set as a HTTP
    /// header of the same name (x-my-header).
    #[web(header)]
    x_my_header: &'static str,
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

        #[post("/data")]
        fn greet2(&self, body: MyData2) -> Result<String, ()> {
            Ok(format!("Hello, {:?}", body))
        }

        #[post("/request-body")]
        fn request_body(&self, body: Vec<u8>) -> Result<String, ()> {
            Ok(format!("We received {} bytes", body.len()))
        }

        #[post("/create")]
        #[content_type("application/json")]
        fn create(&self) -> Result<CreatedResponse, ()> {
            Ok(CreatedResponse {
                message: "created",
                x_my_header: "awesome",
            })
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