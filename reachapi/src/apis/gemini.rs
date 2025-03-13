use super::{Client, Value, json, ReachApiError, RawOuts};

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


/*
Things to look at:

- Temperature in the response of LLM, do we keep it 0 to allows deterministic output for an input?


*/