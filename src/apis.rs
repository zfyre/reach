use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::ArxivConfig;
use crate::config::ArxivKeys;

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

pub async fn gemini_query(gemini_api_key: &str, query: &str) -> Result<String, Error> {
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
            ("num", "10"),
            // ("start", "0"), // for pagination!v-> Move in gaps of 10 becuase the result shows 10 links per request
            // ("exactTerms", "BJP"), // Can be used to search about a author!
            // ("excludeTerms", "elections Modi ... ... ...") // Can directly use this for arxiver!
            // ("dateRestrict", "2016-01-01:m1".to_string()),
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

#[derive(Debug)]
struct ArxivQuery<'a> {
    include_keywords: Vec<String>,
    exclude_keywords: Vec<String>,
    _authors: Vec<String>,
    categories: Vec<String>,
    start: &'a str,       // for pagination!
    max_results: &'a str, // ~500
    sort_by: &'a str,     // "relevance", "lastUpdatedDate"
    sort_order: &'a str,  // 'ascending', 'descending'

                          // id_list: Vstr<'a>,
}
impl<'a> ArxivQuery<'a> {
    fn construct_query(&self) -> Vec<(&'a str, String)> {
        let format_title_include = format!(
            "%28{}%29",
            self.include_keywords
                .iter()
                .map(|keyword| {
                    if keyword.contains(' ') {
                        format!("ti:%22{}%22", keyword.replace(' ', "+"))
                    } else {
                        format!("ti:{}", keyword)
                    }
                })
                .collect::<Vec<_>>()
                .join("+OR+")
        );

        let format_abstract_include = format!(
            "%28{}%29",
            self.include_keywords
                .iter()
                .map(|keyword| {
                    if keyword.contains(' ') {
                        format!("abs:%22{}%22", keyword.replace(' ', "+"))
                    } else {
                        format!("abs:{}", keyword)
                    }
                })
                .collect::<Vec<_>>()
                .join("+OR+")
        );

        let format_abstract_exclude = format!(
            "%28{}%29",
            self.exclude_keywords
                .iter()
                .map(|keyword| {
                    if keyword.contains(' ') {
                        format!("abs:%22{}%22", keyword.replace(' ', "+"))
                    } else {
                        format!("abs:{}", keyword)
                    }
                })
                .collect::<Vec<_>>()
                .join("+AND+")
        );

        let format_categories = format!(
            "%28{}%29",
            self.categories
                .iter()
                .map(|category| category.as_str())
                .collect::<Vec<_>>()
                .join("+OR+")
        );
        // println!("{format_title_include:?}");
        // println!("{format_abstract_include:?}");
        // println!("{format_abstract_exclude:?}");
        // println!("{format_categories:?}");

        let query = format!(
            "{}+AND+{}+ANDNOT+{}+AND+{}",
            format_title_include,
            format_abstract_include,
            format_abstract_exclude,
            format_categories
        );

        vec![
            ("search_query", query),
            ("start", self.start.to_string()),
            ("max_results", self.max_results.to_string()),
            ("sortBy", self.sort_by.to_string()),
            ("sortOrder", self.sort_order.to_string()),
        ]
    }
}
impl Default for ArxivQuery<'_> {
    fn default() -> Self {
        let default_config: HashMap<String, Vec<String>> = ArxivConfig::read_config()
            .expect("[Error] While Reading Arxiv Config")
            .into_iter()
            .collect();
        Self {
            include_keywords: default_config
                .get(&ArxivKeys::IncludeWords.as_str())
                .expect("Gemini API key is not available")
                .to_owned(),

            exclude_keywords: default_config
                .get(&ArxivKeys::ExcludeWords.as_str())
                .expect("Gemini API key is not available")
                .to_owned(),

            _authors: default_config
                .get(&ArxivKeys::Authors.as_str())
                .expect("Gemini API key is not available")
                .to_owned(),

            categories: default_config
                .get(&ArxivKeys::Categories.as_str())
                .expect("Gemini API key is not available")
                .to_owned(),

            start: "0",
            max_results: "500",
            sort_by: "submittedDate",
            sort_order: "descending",
        }
    }
}

pub async fn arxive_search(query: Option<&str>, max_results: &str) -> Result<Vec<String>, Error> {
    let arxive_search_url = "http://export.arxiv.org/api/query";
    let client = Client::new();
    let search_query = match query {
        Some(q) => {
            let mut query_obj = ArxivQuery::default();
            query_obj.include_keywords = vec![q.to_string()];
            query_obj.max_results = max_results;
            
            query_obj.construct_query()
        },
        None => ArxivQuery::default().construct_query(),
    };

    // Manually construct URL with parameters
    let url = search_query
        .iter()
        .fold(String::from(arxive_search_url), |acc, (key, value)| {
            if acc == arxive_search_url {
                format!("{}?{}={}", acc, key, value)
            } else {
                format!("{}&{}={}", acc, key, value)
            }
        });

    let response = client.get(url).send().await?;

    println!("Response: {}", response.text().await?);

    Ok(vec!["placeholder".to_string()])
}

#[cfg(test)]
mod tests {
    use super::arxive_search;
    #[tokio::test]
    async fn check_arxive_search() {
        let _res = arxive_search(None, "10").await;
    }
}
