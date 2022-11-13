use clap::Parser;
use url::Url;
use serde::{Deserialize, Serialize};
use reqwest::header::USER_AGENT;

#[derive(Debug, Deserialize, Serialize)]
struct AssetEntry {
    download_count: i32
}

#[derive(Debug, Deserialize, Serialize)]
struct ReleaseEntry {
    assets: Vec<AssetEntry>
}


#[derive(Parser, Debug)]
struct Args {
    github_url: String
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    let input_url = Url::parse(&args.github_url).expect("Unable to parse input URL");
    println!("Calculating Download Count for {input_url}");
    let project_path = input_url.path();
    let url = format!("https://api.github.com/repos{project_path}/releases");

    let client = reqwest::Client::new();
    let body = client.get(url)
        .header(USER_AGENT, "reqwest/0.11.12".to_owned())
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let releases: Vec<ReleaseEntry> = serde_json::from_str(&body).unwrap();

    let total_downloads: i32 = releases.iter().fold(0, |acc , release|{
        acc + release.assets.iter().fold(0, |acc, asset| {
            acc + asset.download_count
        })
    });
    println!("Total Downloads for all assets in all Releases: {total_downloads}");
}
