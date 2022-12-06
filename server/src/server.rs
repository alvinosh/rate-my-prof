use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

use crate::{config::Config, prelude::Result};

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        let listener = TcpListener::bind(config.address()).await?;

        println!("SERVER LISTENING ON ADDRESS : {}", config.address());

        Ok(Server { listener })
    }

    pub async fn listen(&self, method: &str, path: &str) -> (String, TcpStream) {
        loop {
            if let Ok((mut stream, addr)) = self.listener.accept().await {
                let mut buffer = [0; 1024];
                stream
                    .read(&mut buffer)
                    .await
                    .expect("Could Not Read Buffer Data");

                let mut headers = [httparse::EMPTY_HEADER; 20];
                let mut req = httparse::Request::new(&mut headers);
                let res = req.parse(&buffer).unwrap();

                if path.eq(req.path.unwrap()) {
                    return (String::from(req.path.unwrap()), stream);
                }
            }
        }
    }

    pub async fn get(&self, path: &str) -> (String, TcpStream) {
        self.listen("GET", path).await
    }
}
