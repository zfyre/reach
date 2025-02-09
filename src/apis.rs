use reqwest::Client;
use serde_json::json;
use serde_json::Value;


#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
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


pub async fn gemini_search(gemini_api_key: &str, query: &str) -> Result<String, Error> {
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

pub async fn google_search(
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