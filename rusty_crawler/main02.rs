use anyhow::Result;
use html_parser::{Dom, Node};
use std::process;

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    let html = reqwest::get(url).await?.text().await?;

    let dom = Dom::parse(&html)?;

    for child in dom.children {
        match child {
            Node::Text(text) => {
                log::info!("Node found: {}", text);
            }
            Node::Element(elem) => {
                log::info!("Element found: {}", elem.name);
            }
            Node::Comment(comment) => {
                log::info!("Comment found: {}", comment);
            }
        }
    }

    let res: Vec<String> = Vec::new();
    Ok(res)
}

async fn try_main() -> Result<()> {
    // let resp = reqwest::get("https://google.com").await?;

    let _ = crawl_url("https://google.com").await?;

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
