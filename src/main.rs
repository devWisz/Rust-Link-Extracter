use select::document::Document;
use select::predicate::Name;
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let target_url = "https://www.rust-lang.org/en-US/";
    println!("Fetching {}...", target_url);

    let client = reqwest::Client::new();
    let res = client.get(target_url)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .send()
        .await?
        .text()
        .await?;

    println!("Links found:\n");

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            if link.starts_with('/') {
              
                println!("https://www.rust-lang.org{}", link);
            } else if link.starts_with("http") {
             
                println!("{}", link);
            }
        });

    Ok(())
}