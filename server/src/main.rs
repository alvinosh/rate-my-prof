#![allow(dead_code, unused_variables)]

use tokio::{io::AsyncWriteExt, select};

mod config;
mod error;
mod prelude;
mod server;

#[tokio::main]
async fn main() -> prelude::Result<()> {
    let config = crate::config::Config::new();

    let server = server::Server::new(config)
        .await
        .expect("Could Not Create Server");

    loop {
        select! {
            (req,mut res) = server.get("/hello") => {
                let output = String::from("lol");
                res.write_all(output.as_bytes()).await.unwrap();
            },
            (req,mut res) = server.get("/world") => {
                let output = String::from("wowie");
                res.write_all(output.as_bytes()).await.unwrap();
            }
        }
    }
}
