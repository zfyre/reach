use std::{env, str::FromStr, fs, path::PathBuf};
// use reqwest::Error;
// use reqwest::header::USER_AGENT;
use clap::Parser;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use tokio;
// use std::io::{self, Write};

use dotenv::dotenv;

const CONFIG_FILE: &str = ".websearch-config";
const GOOGLE_API_KEY: &str = "REACH_GOOGLE_SEARCH_API_KEY";
const SEARCH_ENGINE_ID: &str = "REACH_GOOGLE_SEARCH_ENGINE_ID";
const GEMINI_API_KEY: &str = "REACH_GEMINI_API_KEY";

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

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Configure API keys
    Config {
        /// Set Google Search API key
        #[arg(long)]
        google_key: Option<String>,
        
        /// Set Search Engine ID
        #[arg(long)]
        search_engine: Option<String>,
        
        /// Set Gemini API key
        #[arg(long)]
        gemini_key: Option<String>,

        /// Show current configuration
        #[arg(long)]
        show: bool,
    },
}

#[derive(Debug)]
#[allow(dead_code)]
enum Error {
    ReqwestError(reqwest::Error),
    IoError(std::io::Error),
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::ReqwestError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

async fn gemini_search(gemini_api_key: &str, query: &str) -> Result<String, Error> {
    let gemini_request_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"
    );

    let client = Client::new();
    let body: Value = json!({"contents": [{"parts": [{ "text": query }]}]});

    let response = client
        .post(gemini_request_url)
        .header("Content-Type", "application/json")
        .query(&[("key", gemini_api_key)])
        .json(&body)
        .send()
        .await?;

    let json_response: Value = response.json().await?;

    Ok(json_response["candidates"][0]["content"]["parts"][0]["text"].to_string())
    // There is some metadata in the output as well!
}

async fn google_search(
    google_api_key: &str,
    search_engine_id: &str,
    query: &str,
    ftype: &str,
) -> Result<Vec<String>, Error> {
    let google_search_request_url = format!("https://www.googleapis.com/customsearch/v1");
    let client = Client::new();
    let response = client
        .get(google_search_request_url)
        .query(&[
            ("key", google_api_key),
            ("cx", search_engine_id),
            ("q", query),
            ("fileType", ftype),
            // ("dateRestrict", "2016-01-01:m1".to_string()),
            // ("start", "11".to_string()), // for pagination!
            // ("searchType", "image".to_string()),
            // ("lr", "lang_en".to_string()),
            // ("gl", "US".to_string())
        ])
        .send()
        .await?;

    // println!("{}", response.text().await?);
    let json_response: Value = response.json().await?;

    let mut results = Vec::new();

    if let Some(items) = json_response.get("items").and_then(|v| v.as_array()) {
        for item in items {
            if let (Some(title), Some(link)) = (
                item.get("title").and_then(|t| t.as_str()),
                item.get("link").and_then(|l| l.as_str()),
            ) {
                results.push(format!("Title: {}\nURL: {}", title, link));
            }
        }
        Ok(results)
    } else {
        Ok(vec!["No Response!, Try rephrasing your query.".to_string()])
    }
}

fn get_config_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();
    PathBuf::from(home).join(CONFIG_FILE)
}

fn save_config(key: &str, value: &str) -> Result<(), Error> {
    let config_path = get_config_path();
    let content = if config_path.exists() {
        let existing = fs::read_to_string(&config_path)?;
        let mut lines: Vec<String> = existing
            .lines()
            .filter(|line| !line.starts_with(key))
            .map(|line| line.to_string())
            .collect();
        lines.push(format!("{}={}", key, value));
        lines.join("\n")
    } else {
        format!("{}={}", key, value)
    };
    fs::write(config_path, content)?;
    Ok(())
}

fn read_config() -> Result<Vec<(String, String)>, Error> {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(config_path)?;
    Ok(content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else {
                None
            }
        })
        .collect())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    println!("{args:?}");
    match args.command {
        Some(Commands::Config { google_key, search_engine, gemini_key, show }) => {
            if show {
                let config = read_config()?;
                println!("Current configuration:");
                for (key, value) in config {
                    println!("{}: {}", key, value);
                }
                return Ok(());
            }

            if let Some(key) = google_key {
                save_config(GOOGLE_API_KEY, &key)?;
                println!("Google API key saved");
            }
            if let Some(id) = search_engine {
                save_config(SEARCH_ENGINE_ID, &id)?;
                println!("Search Engine ID saved");
            }
            if let Some(key) = gemini_key {
                save_config(GEMINI_API_KEY, &key)?;
                println!("Gemini API key saved");
            }
            Ok(())
        }
        None => {
            dotenv().ok(); // Reads the .env file
            let gemini_api_key = env::var("REACH_GEMINI_API_KEY").expect("Gemini API key is not available");
            let google_api_key =
                env::var("REACH_GOOGLE_SEARCH_API_KEY").expect("Google search API key is not available");
            let google_search_engine_id = env::var("REACH_GOOGLE_SEARCH_ENGINE_ID")
                .expect("Google search engine ID is not available");

            // Replace the following as a config command
            // std::env::set_var(GOOGLE_SEARCH_API, "AIzaSyDkAFBZX5dD-x8ePw0zpgfaq3QWxNPgA7k");
            // std::env::set_var(SEARCH_ENGINE_ID, "a35c7338a6b824947");
            // std::env::set_var(GEMINI_API_KEY, "AIzaSyBeRzObKLp1AgW83_K8gcBTWLshVI__miE");

            // // Using buffer flushing instead of println! for faster and non-repetitive flushing!
            // let stdout = io::stdout(); // get the global stdout entity
            // let mut handle = stdout.lock(); // acquire a lock on it
            // writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

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
