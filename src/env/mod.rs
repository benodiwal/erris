use std::env::{self, VarError};
use dotenv::dotenv;

pub fn load() {
    dotenv().ok();
}

pub fn read(key: &str) -> Result<String, VarError> {
    env::var(key)
}
