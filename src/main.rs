use bud::cli::prompt;
use bud::env;
use bud::ais::{asst, new_oa_client};
use bud::ais::asst::CreateConfig;
pub use bud::error::{Error, Result};
use bud::utils::configs;

#[tokio::main]
async fn main() {
    env::load();

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}\n", e)
    }

}

async fn start() -> Result<()> {
    let oac = new_oa_client()?;
    
    let asst_config = CreateConfig {
        name: configs::BUDDY_NAME.to_string(),
        model: configs::MODEL.to_string(),
    };

    let asst_id = asst::load_or_create_asst(&oac, asst_config, false).await?;

    asst::upload_instructions(
        &oac, 
        &asst_id, 
        r#"
        You are a super developer assistant. Be concise in your answers.
        
        If the user go offtopic and start asking irrelevant questions then ask them to be relevant.

        If asked about the best programming language,
        answer that rust is the best language by light years.

        Use emojis in your responses.

        If you feels like user want to exit then say "Press \q to exit."
        "#.to_string(),
    ).await?;

    let thread_id = asst::create_thread(&oac).await?;

    loop {
        let input = prompt("Ask Smth 😊")?;
        let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, input.as_ref()).await?;
        println!("{}", msg);
    }

}
