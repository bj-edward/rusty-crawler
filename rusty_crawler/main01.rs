use anyhow::Result;
use std::process;

async fn try_main() -> Result<()> {
    let resp = reqwest::get("https://google.com").await?;

    println!("{:?}", resp.text().await);

    log::info!("hello world!");

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match try_main().await {
        Ok(_) => {
            log::info!("Finished");
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}
