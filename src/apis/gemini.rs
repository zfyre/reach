use super::{
    json,
    Value,
    Client,
    RawOuts,
    ReachError,
};

pub async fn gemini_query(gemini_api_key: &str, query: &str) -> Result<Vec<RawOuts>, ReachError> {
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
