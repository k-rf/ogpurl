use clap::Parser;
use std::collections::HashMap;

use ogpurl_core::core::{get_og_title, get_ogp};

#[derive(Parser)]
struct Cli {
    url: String,

    #[arg(short, long, help = "Get og:title")]
    title: bool,
}

enum OgpOrOgTitle {
    Ogp(HashMap<String, String>),
    OgTitle(String),
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let result: OgpOrOgTitle = if args.title {
        OgpOrOgTitle::OgTitle(
            get_og_title(args.url.as_str())
                .await
                .unwrap_or_else(|_| std::process::exit(exitcode::USAGE)), // TODO: エラーハンドリングが雑すぎる
        )
    } else {
        OgpOrOgTitle::Ogp(
            get_ogp(args.url.as_str())
                .await
                .unwrap_or_else(|_| std::process::exit(exitcode::USAGE)), // TODO: エラーハンドリングが雑すぎる
        )
    };

    match result {
        OgpOrOgTitle::OgTitle(r) => println!("{}", r),
        OgpOrOgTitle::Ogp(r) => println!("{:?}", r),
    }

    std::process::exit(exitcode::OK);
}
