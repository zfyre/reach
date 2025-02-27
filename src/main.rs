mod apis;
mod config;
mod display;
mod rsearch;
mod errors;

use tokio;
use clap::Parser;
use std::str::FromStr;
use std::collections::HashMap;
use rsearch::Rsearch;

use apis::*;
use config::*;
use display::*;
use errors::*;
use rsearch::*;


pub const AUTHOR: &str = "Me <kshitiz4kaushik@gmail.com>";
pub const VERSION: &str = "1.0.0";

/// The commands that can be executed
/// 
/// * `Config` - Configure API keys
#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Configure API keys
    ApiConfig(ApiConfig),

    /// Configure Arxiv config
    ArxivConfig(ArxivConfig),

    /// Configure RSearch config
    Rsearch(Rsearch)
}

#[derive(Parser, Debug)]
#[command(
    author = AUTHOR,
    version = VERSION,
    about = "A CLI tool for Web/LLM Searching with a touch of arxive! coming soon",
    long_about = "A command line interface tool for performing web searches using Google Search API and Gemini LLM",
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// The search query to be processed
    query: Option<String>,

    /// Enable LLM mode to use Gemini AI for search
    #[arg(short = 'l', long = "llm", default_value_t = false)]
    llm: bool,

    /// Enable Google mode to search Google Search API
    #[arg(short = 'g', long = "google", default_value_t = true)]
    gs: bool,
    
    /// Specify file type for search results (e.g., pdf, doc)
    #[arg(long, default_value_t = String::from_str("").unwrap(), requires = "gs")]
    ftype: String,

    /// Enable arxive mode to search papers from Arxiv.com
    #[arg(short = 'a', long = "arxiv", default_value_t = false)]
    ax: bool,

    /// Specify the maximum number of results to be returned
    #[arg(long, default_value_t = String::from_str("10").unwrap(), requires = "ax")]
    maxr: String,

}


#[tokio::main]
async fn main() -> Result<(), ReachError> {
    let args = Cli::parse();
    // println!("{args:?}");

    match args.command {
        Some(Commands::ApiConfig(config)) => { // Change Api Config!
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
        Some(Commands::ArxivConfig(config)) => { // Change Arxiv config
            if config.show {
                let config_list = ArxivConfig::read_config()?;
                if config_list.is_empty() {
                    println!("No configuration found.");
                } else {
                    println!("Current configuration:");
                    for (key, value) in config_list {
                        println!("{}={:?}", key, value);
                    }
                }
                return Ok(());
            }

            // Prompt the user to input the config (essentially API-Keys)
            ArxivConfig::get_config_from_user()?;
            Ok(())
        }
        Some(Commands::Rsearch(cmd)) => {
            Ok(())
        }
        None => { // Apply Proper Search

            let api_config: HashMap<String, String> = ApiConfig::read_config()?.into_iter().collect();

            let gemini_api_key = api_config.get(&ApiKeys::Gemini.as_str()).expect("Gemini API key is not available");
            let google_api_key = api_config.get(&ApiKeys::Google.as_str()).expect("Google search API key is not available");
            let google_search_engine_id = api_config.get(&ApiKeys::SearchEngine.as_str()).expect("Google search engine ID is not available");

            // println!("query: {:?}, llm: {:?}", args.query, args.llm);

            if args.llm {
                let out = gemini_query(
                    &gemini_api_key,
                    &args.query.expect("No query provided!")
                ).await?;
                // gemini_display_output(&format!("{}", out).trim_matches('"'));
                GeminiTerminalDisplay::display_in_terminal(out)?;
                // println!("{out}");
                Ok(())
            } else if args.ax {
                let out = arxive_search(
                    Some(&args.query.expect("No query provided!")),
                    &args.maxr
                ).await?;
                // arxiv_display_output(&out);
                ArxivTerminalDisplay::display_in_terminal(out)?;
                // println!("{out:?}");
                Ok(())
            } else {
                let out = google_search(
                    &google_api_key,
                    &google_search_engine_id,
                    &args.query.expect("No query provided!"),
                    &args.ftype,
                ).await?;
                GoogleTerminalDisplay::display_in_terminal(out)?;
                Ok(())
            }

        }
    }
}
