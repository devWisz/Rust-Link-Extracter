use anyhow::Result;
use select::document::Document;
use select::predicate::Name;

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("HTML length: {}", res.len());

    for link in Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
    {
        println!("{}", link);
    }

    Ok(())
}