use dotenv::dotenv;

const DEFAULT_URL: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 1234;

pub struct Config {
    pub url: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        let mut url = String::from(DEFAULT_URL);
        let mut port = DEFAULT_PORT;

        for (key, value) in std::env::vars() {
            match key.to_lowercase().as_str() {
                "url" => url = value,
                "port" => {
                    port = value
                        .parse()
                        .expect("ERROR: PORT IN ENV VARIABLE IS NOT A NUMBER")
                }
                _ => {
                    //println!("FOUND UNDEFINED ENV VARIABLE: {}", key);
                }
            }
        }

        Config { url, port }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.url, self.port)
    }
}
