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
        configs::instructions(),
    ).await?;

    let thread_id = asst::create_thread(&oac).await?;

    loop {
        let input = prompt("Ask Smth 😊")?;
        let msg = asst::run_thread_msg(&oac, &asst_id, &thread_id, input.as_ref()).await?;
        println!("{}", msg);
    }

}
