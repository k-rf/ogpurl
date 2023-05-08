use clap::Parser;

use ogpurl_core::core::get_ogp;

#[derive(Parser)]
struct Cli {
    url: String,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let result = get_ogp(args.url.as_str()).await;

    match result {
        Ok(result) => {
            println!("{:?}", result);
            std::process::exit(exitcode::OK);
        }
        Err(_) => {
            // TODO: エラーハンドリングが雑すぎる
            println!("Error occurred.");
            std::process::exit(exitcode::USAGE);
        }
    }
}
