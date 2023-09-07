use anyhow::anyhow;
use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub secret: String,
}

impl Config {
    pub fn init() -> Result<Self, anyhow::Error> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow!("DATABASE_URL env variable must be set"))?;
        let secret = env::var("SECRET").map_err(|_| anyhow!("SECRET env var must be set"))?;

        Ok(Self {
            database_url,
            secret,
        })
    }
}
