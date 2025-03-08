/// 1. File to configure the Research module using google search, gemini search and Arxive search
/// 2. Handles the search even if Api key is not provided using only Arxiv search : LATER!!
use super::{gemini_query, google_search, ApiConfig, ApiKeys, HashMap, RawOuts, ReachError, Value, Regex};
use crate::rsearch::utils::{append_to_json, get_markdown};

// TODO: Make a struct where we already initialze the api-config during intialization
async fn get_relevent_urls(query: &str, ftype: &str) -> Result<Vec<String>, ReachError> {
    // Get the APIs
    let api_config: HashMap<String, String> =
        ApiConfig::read_config().unwrap().into_iter().collect();
    let google_api_key = api_config
        .get(&ApiKeys::Google.as_str())
        .expect("Google search API key is not available");
    let google_search_engine_id = api_config
        .get(&ApiKeys::SearchEngine.as_str())
        .expect("Google search engine ID is not available");

    // Get the initial responses from the google API
    let out = google_search(&google_api_key, &google_search_engine_id, query, ftype).await?;

    let urls = out
        .iter()
        .map(|a| match a {
            RawOuts::RawGoogleOut((_title, links)) => links.to_owned(),
            _ => "".to_string(),
        })
        .collect();

    Ok(urls)
}

async fn generate_websummary(query: &str, urls: &[String]) -> Result<Value, ReachError> {
    let api_config: HashMap<String, String> =
        ApiConfig::read_config().unwrap().into_iter().collect();
    let gemini_api_key = api_config
        .get(&ApiKeys::Gemini.as_str())
        .expect("Gemini API key is not available");

    let mut url_to_md = super::HashMap::new();
    for url in urls {
        println!("Getting the Markdown for URL= {}", url);
        let md = get_markdown(&url).await?;
        url_to_md.insert(url, md);
    }
    // TODO: Cache this url_to_md HashMap!!

    let urls_md: Vec<_> = url_to_md
        .iter()
        .map(|(url, md)| format!("URL: {}\nMarkdown: {}\n---", url, md))
        .collect();

    let prompt = format!(
        r#"
        You are a PhD researcher with expertise in analyzing academic content. Given multiple webpage contents and a research query, your task is to:

        1. Analyze each webpage's content
        2. Extract and synthesize information specifically relevant to: Query = {}
        3. Provide technically precise summaries.
        4. If there is some math or logic or code included, do not skip that and include in the summary.
        5. Present the information in the format as follows:
            
        markdown_summary_1
        [split]
        markdown_summary_2
        [split]
        markdown_summary_3
        [split]
        ...

        Focus on technical accuracy and academic relevance. Maintain scholarly rigor in your analysis.

        Following are the Webpages' contents:
        {}
        "#,
        query,
        urls_md.join("\n\n")
    );
    println!("Getting the LLM Response");

    let mut gemini_response = gemini_query(&gemini_api_key, &prompt).await?;
    let response_str = match gemini_response.pop().unwrap() {
        RawOuts::RawGeminiOut(output) => Some(output),
        _ => None,
    }
    .unwrap();
    let response_str: Vec<_> = response_str
        .trim()
        .trim_start_matches("\"")
        .trim_end_matches("\"")
        .split("[split]")
        .collect();

    let response: Value = {
        let mut map = serde_json::Map::new();
        for (i, summary) in response_str.iter().enumerate() {
            if i < urls.len() {
                map.insert(urls[i].clone(), Value::String(summary.to_string()));
            }
        }
        Value::Object(map)
    };

    append_to_json(&response, "data/summaries.json")?;

    Ok(response)
}

/// Generates Knoweldge Graph from Webpapes
/// Currently implementing for single webpage, try to implement this for multiple webpages to prevent context loss
async fn generate_webkg(query: &str, url: &str, md: &str) -> Result<Value, ReachError> {
    let api_config: HashMap<String, String> =
        ApiConfig::read_config().unwrap().into_iter().collect();
    let gemini_api_key = api_config
        .get(&ApiKeys::Gemini.as_str())
        .expect("Gemini API key is not available");
    let prompt = format!(
        r#"
            You are a PhD researcher with expertise in analyzing academic content. Given multiple webpage contents and a research query, your task is to create a Knowledge Graph from it:

            1. Extract and synthesize information specifically relevant to: Query = {}
            2. Create a focused knowledge graph for this question. Include only the most important elements:
                - Key Concepts (2-3 main concepts)
                - If the Concepts are mathematical or Logical, include them as well

            3. Essential Relationships (using only these types):
                - IS-A: classification/type relationships
                - RELATES-TO: how concepts connect
                - INFLUENCES: how one concept affects another

            4. Present each relationship in the format as follows only, where [split] is a 'split' token :

            [Concept A1]-[Relationship1]->[Concept B1] [split] [Concept A2]-[Relationship2]->[Concept B2] [split] ...

            Focus on technical accuracy and academic relevance. Maintain scholarly rigor in your analysis.

            Following are the Webpages' contents:
            {}
            "#,
        query, md
    );
    println!("Getting the LLM Response\n");

    let mut gemini_response = gemini_query(&gemini_api_key, &prompt).await?;
    let response_str = match gemini_response.pop().unwrap() {
        RawOuts::RawGeminiOut(output) => Some(output),
        _ => None,
    }
    .unwrap();
    let response_str: Vec<_> = response_str
        .trim()
        .trim_start_matches("\"")
        .trim_start_matches("```")
        .trim_start_matches("\\n")
        .trim_end_matches("\"")
        .trim_end_matches("```")
        .trim_end_matches("\\n")
        .split("[split]")
        .collect();

    // println!("{response_str:?}");

    let mut edges = Vec::new();
    for edge_str in response_str {
        let edge = edge_str.trim();
        println!("{edge:#?}");
        let re = Regex::new(r"\[([^\]]+)\]-\[([^\]]+)\]->\[([^\]]+)\]").unwrap();
        if let Some(captures) = re.captures(edge) {
            if captures.len() == 4 {
            edges.push((
                captures[1].to_string(),
                captures[2].to_string(), 
                captures[3].to_string()
            ));
            }
        }
    }
    // println!("{edges:?}");
    let response: Value = {
        let mut map = serde_json::Map::new();
        map.insert(
            url.to_string(),
            Value::Array(
                edges
                    .iter()
                    .map(|(src, rel, dst)| {
                        let mut edge_map = serde_json::Map::new();
                        edge_map.insert("source".to_string(), Value::String(src.clone()));
                        edge_map.insert("relationship".to_string(), Value::String(rel.clone()));
                        edge_map.insert("target".to_string(), Value::String(dst.clone()));
                        Value::Object(edge_map)
                    })
                    .collect(),
            ),
        );
        Value::Object(map)
    };

    append_to_json(&response, "data/knowledge_graph.json")?;

    Ok(response)
}

/// Generates the Context for the next query
/// # IMP `The query can be user decided new query or the initial query`
async fn get_context_for_next_query_from_kg(query: &str, num_steps: i8, num_queries: i8) -> Result<Vec<String>, ReachError> {
    // Extract the Recent Extracted Concepts & Relationships that is the recent Edges

    // Use these Concepts to perform a random walk on KG for R steps

    // Get the Context for the next query

    // Return the next possible quries
    Ok(vec!["".to_string()])
}

async fn build_kg_iterativiely(query: &str, num_iter: i16, _ftype: &str) -> Result<(), ReachError> {

    for itr in 0..num_iter {

        let _next_queries = match itr {
            0 => vec![query.to_string()],
            _ => get_context_for_next_query_from_kg(query, 2, 3).await?,
        };
        
        todo!("Allow user to edit/choose the next query from the list of next_queries");

        for query in &_next_queries {
            let urls = get_relevent_urls(&query, "").await?;
            println!("{urls:?}");

            for url in &urls {
                println!("Calling generate_webkg for url: {:?}", url);
                let md = get_markdown(&url).await?;
                let _kg = generate_webkg(&query, &url, &md).await?;
                println!("generate_webkg completed!");
            }
        }
    }

    Ok(())
}


mod tests {
    // use std::{collections::{linked_list, HashMap}, fmt::format};

    // use crate::{
    //     apis::{gemini_query, google_search},
    //     config::{ApiConfig, ApiKeys}, display::RawOuts, rsearch::{get_markdown, Context},
    // };

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

    #[cfg(feature = "requires_config")]
    #[tokio::test]
    async fn test_summarizer() -> Result<(), super::ReachError> {
        let query = "What are Diffusion Models?";
        let urls = super::get_relevent_urls(&query, "").await?;
        println!("{urls:?}");

        for chunk in urls.chunks(2) {
            println!("Calling generate_websummary for chunk: {:?}", chunk);
            let _summaries = super::generate_websummary(&query, chunk).await?;
            println!("generate_websummary completed for chunk: {:?}", chunk);
        }
        Ok(())
    }
    
    // #[cfg(feature = "requires_config")]
    #[tokio::test]
    async fn test_kg_gen() -> Result<(), super::ReachError> {
        let query = "What are Diffusion Models?";
        let urls = super::get_relevent_urls(&query, "").await?;
        println!("{urls:#?}");

        for url in &urls {
            println!("Calling generate_webkg for url: {:?}", url);
            let md = crate::rsearch::utils::get_markdown(&url).await?;
            let _kg = super::generate_webkg(&query, &url, &md).await?;
            println!("generate_webkg completed!");
        }
        Ok(())
    }
}
