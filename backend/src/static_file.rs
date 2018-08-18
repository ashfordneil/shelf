//! Static file serving
use tokio::prelude::Future;

use http::Response;

use tokio::fs::File;

#[derive(Clone, Debug)]
pub struct StaticFile;

impl_web! {
    impl StaticFile {
        #[get("/")]
        fn index(&self) -> impl Send + Future<Item = Response<File>, Error = ()> {
            File::open("../frontend/dist/index.html")
                .map_err(drop)
                .map(|file| Response::builder().status(200).body(file).unwrap())
        }

        #[get("/static/:file")]
        fn file(&self, file: String) -> impl Send + Future<Item = Response<File>, Error = ()> {
            File::open(format!("../frontend/dist/{}", file))
                .map_err(drop)
                .map(|file| Response::builder().status(200).body(file).unwrap())
        }
    }
}
