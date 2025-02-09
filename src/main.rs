mod apis;
mod config;

use tokio;
use clap::Parser;
// use dotenv::dotenv;
// use std::{env, str::FromStr};
use std::str::FromStr;
use std::collections::HashMap;

use apis::*;
use config::*;


#[derive(Parser, Debug)]
#[command(
    author = "Me <kshitiz4kaushik@gmail.com>",
    version = "1.0.0",
    about = "A CLI tool for Web/LLM Searching with a touch of arxive! coming soon",
    long_about = "A command line interface tool for performing web searches using Google Search API and Gemini LLM",
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The search query to be processed
    query: Option<String>,

    /// Enable LLM mode to use Gemini AI for search
    #[arg(long, default_value_t = false)]
    llm: bool,

    /// Specify file type for search results (e.g., pdf, doc)
    #[arg(long, default_value_t = String::from_str("pdf").unwrap(), conflicts_with = "llm")]
    ftype: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    // println!("{args:?}");

    match args.command {
        Some(Commands::Config(config)) => {
            if config.show {
                let config_list = ApiConfig::read_config()?;
                if config_list.is_empty() {
                    println!("No configuration found.");
                } else {
                    println!("Current configuration:");
                    for (key, value) in config_list {
                        println!("{}={}", key, value);
                    }
                }
                return Ok(());
            }

            // Prompt the user to input the config (essentially API-Keys)
            ApiConfig::get_config_from_user()?;

            Ok(())
        }
        None => {

            let api_config: HashMap<String, String> = ApiConfig::read_config()?.into_iter().collect();

            let gemini_api_key = api_config.get(ApiKeys::Gemini.as_str()).expect("Gemini API key is not available");
            let google_api_key = api_config.get(ApiKeys::Google.as_str()).expect("Google search API key is not available");
            let google_search_engine_id = api_config.get(ApiKeys::SearchEngine.as_str()).expect("Google search engine ID is not available");

            println!("query: {:?}, llm: {:?}", args.query, args.llm);

            if args.llm {
                let out = gemini_search(
                    &gemini_api_key,
                    &args.query.expect("No query provided!")
                ).await?;
                println!("{out}");
                Ok(())
            } else {
                let out = google_search(
                    &google_api_key,
                    &google_search_engine_id,
                    &args.query.expect("No query provided!"),
                    &args.ftype,
                )
                .await?;

                for val in out {
                    println!("{val}\n");
                }
                Ok(())
            }

        }
    }
}
