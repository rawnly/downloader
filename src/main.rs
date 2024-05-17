use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::Url;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Clone, Debug)]
struct Args {
    /// Download location (default to current dir)
    #[arg(short, long)]
    output: Option<PathBuf>,

    urls: Vec<Url>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::try_parse()?;

    let mut handles = vec![];

    for url in args.clone().urls {
        let args = args.clone();

        let output = args.output;

        let handle = tokio::spawn(async move { download(url, output).await });

        handles.push(handle);
    }

    let mut i = 0;
    for h in handles {
        i += 1;
        h.await??;
        let total = args.urls.len();
        println!("[{i}/{total}] Downloaded");
    }

    Ok(())
}

async fn download(url: Url, output: Option<PathBuf>) -> Result<()> {
    let filename = url.path().split('/').last().map(|f| f.to_string());

    let response = reqwest::get(url).await?;
    let body = response.bytes().await?;

    let filename = filename.ok_or(anyhow!("Expected file name"))?;

    let path = output
        .map(|output| output.join(&filename))
        .unwrap_or(filename.into());

    let mut file = File::create(path).await?;
    file.write_all(&body).await?;

    Ok(())
}
