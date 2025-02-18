/// 1. File to configure the Research module using google search, gemini search and Arxive search
/// 2. Handles the search even if Api key is not provided using only Arxiv search : LATER!!



// MKAE THIS OPEN IN A VIM TYPE DIFFERENT GUI IN TERMINAL WHEERE YOU CAN DO VARIOUS TO & FRO COMMS


use clap::Parser;
use scraper::{Html, Selector};

#[derive(Parser, Debug)]
pub struct RSearch {

}

// fn get_inner_html_from_a_page() {

// }


mod tests {
    use std::collections::HashMap;

    use crate::{apis::gemini_query, config::{ApiConfig, ApiKeys}};

    #[tokio::test]
    async fn get_html_from_site() {
        let body = reqwest::get("https://aisafetyfundamentals.com/blog/introduction-to-mechanistic-interpretability/").await.unwrap();
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
        
        let api_config: HashMap<String, String> = ApiConfig::read_config()
            .unwrap()
            .into_iter()
            .collect();

        let prompt = "I will provide you with the extracted text content from a webpage. Please analyze it thoroughly and provide a comprehensive summary including: 1) The main topic and purpose of the content, 2) Key concepts and arguments presented, 3) Any significant findings or conclusions, 4) The target audience, and 5) The overall writing style and tone. Be detailed but concise in your analysis. Here is the text:";

        let gemini_api_key = api_config.get(&ApiKeys::Gemini.as_str()).expect("Gemini API key is not available");
        let a = gemini_query(gemini_api_key, &format!("{}\n{}", prompt, res)).await.unwrap();

        println!("{:?}", a);
    }
}
