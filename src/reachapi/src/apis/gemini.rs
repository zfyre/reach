use super::{Client, Value, json, ReachApiError, RawOuts, GenerateContentResponse, FinishReason};

use futures_util::TryStreamExt;
use tokio_util::io::StreamReader;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::LinesStream;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::io;


pub async fn gemini_query(gemini_api_key: &str, query: &str) -> Result<Vec<RawOuts>, ReachApiError> {
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

pub async fn gemini_query_stream(gemini_api_key: &str, query: &str, stream: bool) -> Result<Vec<RawOuts>, ReachApiError> {


    let gemini_request_url = match stream {
        false => format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent"
        ),
        true => format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:streamGenerateContent"
        ),
    };

    let client = Client::new();
    let body: Value = json!({"contents": [{"parts": [{ "text": query }]}]});
    let query = vec![("key", gemini_api_key), ("alt", "sse")];

    // let query = match stream {
    //     false => vec![("key", gemini_api_key)],
    //     true => vec![("key", gemini_api_key), ("alt", "sse")],
    // };

    let resp = client
        .post(gemini_request_url)
        .header("Content-Type", "application/json")
        .query(&query)
        .json(&body)
        .send()
        .await?;

    let stream = resp.bytes_stream(); // <-- this works if "stream" feature is enabled

    let reader = StreamReader::new(
        stream.map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
    );

    let lines = LinesStream::new(BufReader::new(reader).lines());
    tokio::pin!(lines);

    while let Some(line) = lines.next().await {
        let line = line?;

        if line.starts_with("data: ") {
            let json = &line[6..]; // Skip the "data: " prefix

            match serde_json::from_str::<GenerateContentResponse>(json) {

                Ok(response) => {
                    println!("Response: {:?}", response);
                    match &response.candidates[0].finish_reason {
                            Some(reason) => {
                                if matches!(reason, FinishReason::Stop) {
                                    break;
                            }
                        },
                        None => {
                            let tokens = response.candidates[0].content.parts[0].text
                            .trim()
                            .trim_start_matches("\"")
                            .trim_start_matches("\\")
                            .trim_end_matches("\"")
                            .trim_end_matches("\\");
                        
                        print!("{}", tokens);
                            tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                        }
                    };
                },
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    break;
                }
                    
            }

        }
    }



    Ok(vec![])

    // let json_response: Value = response.json().await?;
    // let res = json_response["candidates"][0]["content"]["parts"][0]["text"].to_string();
    // Ok(vec![RawOuts::RawGeminiOut(res)])
    
    // There is some metadata in the output as well!
}


/*
Things to look at:

- Temperature in the response of LLM, do we keep it 0 to allows deterministic output for an input?


*/

//############################################### TESTS ################################################//

/// To run the tests, use the following command:
/// `cargo test test_streaming --features debug,requires_config`
/// `cargo test test_streaming --features debug,requires_config -- --nocapture` # For verbose output
#[cfg(test)]
// #[cfg(all(feature = "debug", feature = "requires_config"))]
mod tests{

    use crate::{gemini_query_stream, ApiConfig, ApiKeys, ReachApiError, ReachConfig, ReachConfigKeys};
    use tokio;

    #[tokio::test]
    async fn test_streaming() -> Result<(), ReachApiError>{
        let api_config: std::collections::HashMap<String, String> = ApiConfig::read_config()?.into_iter().collect();
        let gemini_api_key = api_config.get(&ApiKeys::Gemini.as_str()).expect("Gemini API key is not available");
        
        gemini_query_stream(gemini_api_key, "Tell me about David Finchers's movies!", true).await?;
        Ok(())
    }
}