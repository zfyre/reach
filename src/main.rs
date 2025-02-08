
use std::{env, str::FromStr};
// use reqwest::Error;
// use reqwest::header::USER_AGENT;
use clap::Parser;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use tokio;
// use std::io::{self, Write};

use dotenv::dotenv;

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
    #[arg(requires_ifs = [("command", "None")])]
    query: Option<String>,

    /// Enable LLM mode to use Gemini AI for search
    #[arg(long, default_value_t = false)]
    llm: bool,

    /// Specify file type for search results (e.g., pdf, doc)
    #[arg(long, default_value_t = String::from_str("pdf").unwrap(), conflicts_with = "llm")]
    ftype: String,
}

#[derive(clap::Subcommand)]
#[derive(Debug)]
enum Commands {
    /// Configure API keys
    Config,
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

#[tokio::main]
async fn main() -> Result<(), Error> {

    
    // dotenv().ok(); // Reads the .env file
    // let gemini_api_key = env::var("REACH_GEMINI_API_KEY").expect("Gemini API key is not available");
    // let google_api_key =
    //     env::var("REACH_GOOGLE_SEARCH_API_KEY").expect("Google search API key is not available");
    // let google_search_engine_id = env::var("REACH_GOOGLE_SEARCH_ENGINE_ID")
    //     .expect("Google search engine ID is not available");

    // // Replace the following as a config command
    // // std::env::set_var(GOOGLE_SEARCH_API, "AIzaSyDkAFBZX5dD-x8ePw0zpgfaq3QWxNPgA7k");
    // // std::env::set_var(SEARCH_ENGINE_ID, "a35c7338a6b824947");
    // // std::env::set_var(GEMINI_API_KEY, "AIzaSyBeRzObKLp1AgW83_K8gcBTWLshVI__miE");

    // // // Using buffer flushing instead of println! for faster and non-repetitive flushing!
    // // let stdout = io::stdout(); // get the global stdout entity
    // // let mut handle = stdout.lock(); // acquire a lock on it
    // // writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

    let args = Cli::parse();
    println!("{args:?}");
    // println!("query: {:?}, llm: {:?}", args.query, args.llm);

    // if args.llm {
    //     let out = gemini_search(
    //         &gemini_api_key,
    //         &args.query
    //     ).await?;
    //     println!("{out}");
    //     Ok(())
    // } else {
    //     let out = google_search(
    //         &google_api_key,
    //         &google_search_engine_id,
    //         &args.query,
    //         &args.ftype,
    //     )
    //     .await?;

    //     for val in out {
    //         println!("{val}\n");
    //     }
    //     Ok(())
    // }

    Ok(())
}

// GOOGLE_SEARCH_API=AIzaSyDkAFBZX5dD-x8ePw0zpgfaq3QWxNPgA7k
// SEARCH_ENGINE_ID=a35c7338a6b824947
// GEMINI_API_KEY=AIzaSyBeRzObKLp1AgW83_K8gcBTWLshVI__miE
