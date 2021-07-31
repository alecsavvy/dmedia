use dotenv;
use envy;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub actix_url: String,
    pub redis_url: String,
    pub chain_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().expect("could not load dotenv file");
        envy::from_env::<Config>().expect("could not parse dotenv file")
    }
}
