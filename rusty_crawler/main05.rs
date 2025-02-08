use anyhow::{Result, anyhow};
use clap::Parser;
use html_parser::{Dom, Element, Node};
use std::process;

/// Simple program to greet a persion
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ProgramArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    starting_url: String,
}

/// This will turn relativ eurls into full urls.
/// E.g. get_url("/services/" "https://google.com/") -> "https://google.com/service/"
fn get_url(url: &str, root_url: &str) -> String {
    // log::info!("comparing {} and {}", url, root_url);
    if url.starts_with("https://") || url.starts_with("http://") {
        return url.into();
    }

    // log::info!("formatting string");
    format!(
        "{}/{}",
        root_url.strip_suffix('/').unwrap_or(root_url),
        url.strip_prefix('/').unwrap_or(url)
    )
}

fn is_node(node: &Node) -> bool {
    match node {
        Node::Element(..) => true,
        _ => false,
    }
}

fn crawl_element(elem: &Element, root_url: &str) -> Result<Vec<String>> {
    let mut links: Vec<String> = Vec::new();

    // Figure out whether we have a link on this node!
    if elem.name == "a" {
        let href_attrib = elem
            .attributes
            .get("href")
            .ok_or_else(|| anyhow!("could not find href in link"))?
            .as_ref()
            .ok_or_else(|| anyhow!("href does not have a value"))?
            .into();

        links.push(href_attrib);
    }

    for node in elem.children.iter().filter(|c| is_node(c)) {
        match node {
            Node::Element(elem) => {
                // add whatever links from this elem to our vector
                let mut children_links = crawl_element(elem, root_url)?;
                links.append(&mut children_links);
            }
            _ => {}
        }
    }

    Ok(links)
}

async fn crawl_url(url: &str) -> Result<Vec<String>> {
    // Parsing html into a DOM obj
    let html = reqwest::get(url).await?.text().await?;

    let dom = Dom::parse(&html)?;

    // Crawls all the nodes in the main html
    for child in dom.children {
        match child {
            Node::Element(elem) => {
                for link in crawl_element(&elem, url)? {
                    log::info!("Links found for elem {}: {:?}", url, link);
                }
            }
            _ => {}
        }
    }

    // TODO: change this to the sum of links
    let res: Vec<String> = Vec::new();
    Ok(res)
}

async fn try_main(args: ProgramArgs) -> Result<()> {
    // let resp = reqwest::get("https://google.com").await?;

    let _ = crawl_url(&args.starting_url).await?;

    log::info!("hello world!");

    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = ProgramArgs::parse();

    match try_main(args).await {
        Ok(_) => {
            log::info!("Finished");
        }
        Err(e) => {
            log::error!("Error: {:?}", e);
            process::exit(-1);
        }
    }
}
