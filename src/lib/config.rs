use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub client_id: String,
    pub secret: String,
    pub subscribes: Vec<String>
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { client_id: "".into(), secret: "".into(), subscribes: vec![] }}
}
