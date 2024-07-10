pub mod asst;
mod msg;

use crate::{env, utils::constants};
use crate::error::Result;
use async_openai::{Client, config::OpenAIConfig};

pub type OaClient = Client<OpenAIConfig>;

pub fn new_oa_client() -> Result<OaClient> {
    if env::read(constants::ENV_OPENAI_API_KEY).is_ok() {
        Ok(Client::new())
    } else {
        Err("No openai API key is in env".into())
    }
}
