use super::{Client, FinishReason, GenerateContentResponse, RawOuts, ReachApiError, Value, json};

use futures::TryStreamExt;
use reqwest::StatusCode;
use core::str;

pub async fn gemini_query(
    gemini_api_key: &str,
    query: &str,
) -> Result<Vec<RawOuts>, ReachApiError> {
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
    let res = json_response["candidates"][0]["content"]["parts"][0]["text"].to_string();

    Ok(vec![RawOuts::RawGeminiOut(res)])
    // There is some metadata in the output as well!
}

pub async fn gemini_query_stream(
    gemini_api_key: &str,
    query: &str,
) -> Result<Vec<RawOuts>, ReachApiError> {
    let gemini_request_url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent";

    let client = Client::new();
    let body: Value = json!({"contents": [{"parts": [{ "text": query }]}]});
    let auth = vec![("key", gemini_api_key), ("alt", "sse")];
    let response = client
        .post(gemini_request_url)
        .header("Content-Type", "application/json")
        .query(&auth)
        .json(&body)
        .send()
        .await?;

    if response.status() == StatusCode::OK {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.try_next().await? {
            let chunk_string = str::from_utf8(&chunk).unwrap();
            if chunk_string.starts_with("data: ") {
                let json_str = &chunk_string[6..];
                let json_res = serde_json::from_str::<GenerateContentResponse>(json_str)?;
                let content = render_content(&json_res).await;
                

                match &json_res.candidates[0].finish_reason {
                    Some(reason) => {
                        match reason { FinishReason:: Stop => break, _ => () }
                    },
                    None => continue,
                }
            }

            println!("Chunk: {}", chunk_string);
        }
    }
    else {
        println!("Error: {}", response.text().await?);
    }

    Ok(vec![])
}

async fn render_content(response: &GenerateContentResponse) -> String {
    let tokens = response.candidates[0].content.parts[0]
        .text
        .trim();
        // .trim_start_matches("\"")
        // .trim_start_matches("\\")
        // .trim_end_matches("\"")
        // .trim_end_matches("\\");

    format!("{}", tokens)

    // if let Some(metadata) = &response.usage_metadata {
    //     if metadata.candidates_token_count > 0 {
    //         println!("\nToken count: {}", metadata.candidates_token_count);
    //     }
    // }
    
}

/*
TODO: Things to look at:

- Temperature in the response of LLM, do we keep it 0 to allows deterministic output for an input?


*/

//############################################### TESTS ################################################//

/// To run the tests, use the following command:
/// `cargo test test_streaming --features debug,requires_config`
/// `cargo test test_streaming --features debug,requires_config -- --nocapture` # For verbose output
#[cfg(test)]
// #[cfg(all(feature = "debug", feature = "requires_config"))]
mod tests {

    use crate::{
        ApiConfig, ApiKeys, ReachApiError, ReachConfig, ReachConfigKeys, gemini_query_stream,
    };
    use tokio;

    #[tokio::test]
    async fn test_streaming() -> Result<(), ReachApiError> {
        let api_config: std::collections::HashMap<String, String> =
            ApiConfig::read_config()?.into_iter().collect();
        let gemini_api_key = api_config
            .get(&ApiKeys::Gemini.as_str())
            .expect("Gemini API key is not available");

        gemini_query_stream(gemini_api_key, "generate [XGSB] 100 times").await?;
        Ok(())
    }
}
