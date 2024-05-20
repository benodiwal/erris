// region :--- Modules
mod error;
mod buddy;
mod ais;

use dotenv::dotenv;
use ais::{asst, new_oa_client};
use ais::asst::CreateConfig;
pub use error::{Error, Result};
// endregion: --- Modules

#[tokio::main]
async fn main() {

    dotenv().ok();
    // println!();

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}\n", e)
    }

}

async fn start() -> Result<()> {

    // OpenAI Assistant Client
    let oac = new_oa_client()?;    

    // Assistant Configuration
    let asst_config = CreateConfig {
        name: "buddy-02".to_string(),
        model: "gpt-3.5-turbo-1106".to_string(),
    };

    // Loading/Creating assistant
    let asst_id = asst::load_or_create_asst(&oac, asst_config, false).await?;

    // Uploading Instructions
    asst::upload_instructions(
        &oac, 
        &asst_id, 
        r#"
        You are a super developer assistant. Be concise in your answers.
        
        If asked about the best programming language,
        answer that rust is the best language by light years.

        And the second best language is OCaml.
        "#.to_string(),
    ).await?;

    // Creating thread
    let thread_id = asst::create_thread(&oac).await?;

    let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, "What is the best language").await?;
    println!("->> response: {}", msg);

    Ok(())
}
