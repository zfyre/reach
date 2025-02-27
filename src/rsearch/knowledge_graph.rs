/// 1. File to configure the Research module using google search, gemini search and Arxive search
/// 2. Handles the search even if Api key is not provided using only Arxiv search : LATER!!
use super::{gemini_query, get_markdown, ApiConfig, ApiKeys, HashMap, RawOuts, ReachError, Regex};

#[derive(Debug)]
struct UrlData {
    url: String,
    summary: String,
    backlinks: Vec<String>,
}

/// Stores the context for the conversation with the Gemini API!

#[derive(Debug)]
struct Context {
    data: Vec<(bool, UrlData)>,       // (Status, Data of each URL!)
    knowledge: Vec<(String, String)>, // The Overall knowledge from links and the corresponding query
}
impl Context {
    fn new() -> Self {
        Self {
            data: vec![],
            knowledge: vec![],
        }
    }

    async fn add(&mut self, url: &[String], query: &str) -> Result<(), ReachError> {
        // Get the Markdown of multiple URLs
        let mut url_to_md = HashMap::new();
        for url in url {
            println!("Getting the Markdown for URL= {}", url);
            let md = get_markdown(url).await?;
            url_to_md.insert(url, md);
        }

        // Get the summary from the markdowns using a single Gemini query
        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();

        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");

        let urls_md: Vec<_> = url_to_md
            .iter()
            .map(|(url, md)| format!("URL: {}\nMarkdown: {}\n---", url, md))
            .collect();

        let prompt = format!(
            r#"
            You are a PhD researcher with expertise in analyzing academic content. Given multiple webpage contents and a research query, your task is to:

            1. Analyze each webpage's content
            2. Extract and synthesize information specifically relevant to: Query = {}
            3. Provide technically precise yet concise summaries
            4. Present the information in valid JSON format with the following structure:
            {{
                "summaries": {{
                    "url1": "markdown_summary1",
                    "url2": "markdown_summary2",
                    ...
                }}
            }}

            Focus on technical accuracy and academic relevance. Maintain scholarly rigor in your analysis.

            Following are the Webpages' contents:
            {}
            "#,
            query,
            urls_md.join("\n")
        );

        println!("Getting the LLM Response");
        let mut gemini_response = gemini_query(&gemini_api_key, &prompt).await?;
        let json_str = match gemini_response.pop().unwrap() {
            RawOuts::RawGeminiOut(output) => Some(output),
            _ => None,
        }
        .unwrap();
        // let json_str = "```json\n{\n    \"summaries\": {\n        \"https://www.reddit.com/r/MachineLearning/comments/1eki8kn/d_diffusion_vs_flow/\": \"This Reddit trend.\",\n        \"https://en.wikipedia.org/wiki/Flow-based_generative_model\": \"Flow-based generative detection.\",\n        \"https://proceedings.neurips.cc/paper_files/paper/2021/file/876f1f9954de0aa402d91bb988d12cd4-Paper.pdf\": \"The provided.\"\n    }\n}\n```".to_string();
        println!("Raw LLM Response: {json_str}");
        let re = Regex::new(r"(?s)```json(.*?)```").unwrap();
        todo!();
        let cleaned_json = re.replace(&json_str, "$1").trim().to_string();
        println!("Cleaned LLM Response: {cleaned_json}");
        // Parse JSON response
        let response: serde_json::Value = serde_json::from_str(&cleaned_json)?;
        println!("Parsed JSON Response: {response}");

        if let Some(summaries) = response["summaries"].as_object() {
            println!("Summaries found: {:?}", summaries);
            for (url, summary) in summaries {
                println!("Processing URL: {url}");
                self.data.push((
                    false,
                    UrlData {
                        url: url.to_string(),
                        summary: summary.as_str().unwrap_or_default().to_string(),
                        backlinks: vec![],
                    },
                ));
            }
            println!("Updated data: {:?}", self.data);
        } else {
            println!("No summaries found in the response.");
        }

        Ok(())
    }
}

// async fn run() -> Vec<String> { // Returns the Vector of String of urls Selected by Gemini

// }

mod tests {
    // use std::{collections::{linked_list, HashMap}, fmt::format};

    // use crate::{
    //     apis::{gemini_query, google_search},
    //     config::{ApiConfig, ApiKeys}, display::RawOuts, rsearch::{get_markdown, Context},
    // };

    #[cfg(feature = "requires_config")]
    #[tokio::test]
    async fn get_html_from_site() {
        let body = reqwest::get("https://www.airbnb.co.in/").await.unwrap();
        // let res = body.text().await.unwrap();
        let html = body.text().await.unwrap();
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("body").unwrap();
        let res = document
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<Vec<_>>().join(" "))
            .unwrap_or_default();

        // println!("{}", res.trim_end().split_whitespace().collect::<Vec<_>>().join(" "));

        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();

        let prompt = "I will provide you with the extracted text content from a webpage. Please analyze it thoroughly and provide a comprehensive summary including: 1) The main topic and purpose of the content, 2) Key concepts and arguments presented, 3) Any significant findings or conclusions, 4) The target audience, and 5) The overall writing style and tone. Be detailed but concise in your analysis. Here is the text:";

        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");
        let a = gemini_query(gemini_api_key, &format!("{}\n{}", prompt, res))
            .await
            .unwrap();

        println!("{:?}", a);
    }

    #[cfg(feature = "requires_config")]
    #[tokio::test]
    async fn run_rsearch() {
        let api_config: HashMap<String, String> =
            ApiConfig::read_config().unwrap().into_iter().collect();

        // let gemini_api_key = api_config.get(&ApiKeys::Gemini.as_str()).expect("Gemini API key is not available");
        let google_api_key = api_config
            .get(&ApiKeys::Google.as_str())
            .expect("Google search API key is not available");
        let google_search_engine_id = api_config
            .get(&ApiKeys::SearchEngine.as_str())
            .expect("Google search engine ID is not available");

        let q = "What are Flow based Diffusion Models?";
        // Get the initial responses from the google API
        let out = google_search(&google_api_key, &google_search_engine_id, q, "")
            .await
            .unwrap();

        let urls: Vec<_> = out
            .iter()
            .map(|a| match a {
                RawOuts::RawGoogleOut((_, links)) => links.to_owned(),
                _ => "".to_string(),
            })
            .collect();

        // GET the initial URls
        println!("URLS => {:?}", urls);

        let mut con = Context::new();
        // Use the url -> Markdown converter
        // for url in urls {
        //     con.add(&url, q).await.unwrap()
        // }
        con.add(&urls[0..3], q).await.unwrap();
        //TODO: Maybe we can collect all the futures and run the python process parallely to get all the markdowns simultaneously
        println!("{:?}", con);
    }
}
