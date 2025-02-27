use super::{
    Client,
    RawOuts,
    ReachError,
    Value
};

pub async fn google_search(
    google_api_key: &str,
    search_engine_id: &str,
    query: &str,
    ftype: &str,
) -> Result<Vec<RawOuts>, ReachError> {
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
                // results.push(format!("Title: {}\nURL: {}", title, link));
                results.push(RawOuts::RawGoogleOut((title.to_string(), link.to_string())));
            }
        }
        Ok(results)
    } else {
        // TODO: Make this a better error handling!
        Ok(vec![RawOuts::RawGoogleOut((
            "No Response!, Try rephrasing your query.".to_string(),
            "".to_string(),
        ))])
    }
}
