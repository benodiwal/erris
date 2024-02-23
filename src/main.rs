// region :--- Modules
mod error;
mod buddy;
mod ais;

use ais::new_oa_client;
pub use error::{Error, Result};
// endregion: --- Modules

#[tokio::main]
async fn main() {
    println!();

    match start().await {
        Ok(_) => println!("\nBye!\n"),
        Err(e) => println!("\nError: {}\n", e)
    }
}

async fn start() -> Result<()> {
    let oac = new_oa_client()?;
    println!("->> oac: {:?}", oac);
    Ok(())
}
