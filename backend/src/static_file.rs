//! Static file serving
use std::env;

use tokio::prelude::Future;

use http::Response;

use tokio::fs::File;

#[derive(Clone, Debug)]
pub struct StaticFile;

lazy_static! {
    static ref ROOT_PATH: String = env::var("STATIC_FILES").unwrap_or("../frontend/dist".into());
}

impl_web! {
    impl StaticFile {
        #[get("/")]
        fn index(&self) -> impl Send + Future<Item = Response<File>, Error = ()> {
            File::open(format!("{}/index.html", *ROOT_PATH))
                .map_err(drop)
                .map(|file| Response::builder().status(200).body(file).unwrap())
        }

        #[get("/share/:_share")]
        fn index2(&self, _share: String) -> impl Send + Future<Item = Response<File>, Error = ()> {
            File::open(format!("{}/index.html", *ROOT_PATH))
                .map_err(drop)
                .map(|file| Response::builder().status(200).body(file).unwrap())
        }

        #[get("/static/:file")]
        fn file(&self, file: String) -> impl Send + Future<Item = Response<File>, Error = ()> {
            File::open(format!("{}/{}", *ROOT_PATH, file))
                .map_err(drop)
                .map(|file| Response::builder().status(200).body(file).unwrap())
        }
    }
}
