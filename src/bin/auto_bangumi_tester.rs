use auto_bangumi_rs::parser::Parser;
use colored::Colorize;
use rss::Channel;

use std::{env, process::exit};

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ./program url1 url2 url3...");
        exit(1);
    }

    args.remove(0);
    for url in args {
        test_url(&url).await;
    }
}

async fn test_url(url: &str) {
    let res = reqwest::get(url).await.unwrap();
    let status = res.status();
    if status.is_success() {
        let bytes = &res.bytes().await.unwrap()[..];
        let channel = Channel::read_from(bytes).unwrap();
        for item in channel.items {
            if let Some(raw_title) = item.title {
                let parser = Parser::new(raw_title).unwrap();
                if parser.can_parse() {
                    match parser.to_bangumi() {
                        Some(b) => println!("{}", b),
                        None => eprintln!("{}", "FAILED".red())
                    }
                } else {
                    eprintln!("{}", "FAILED".red())
                }
            }
        }
    }
}
