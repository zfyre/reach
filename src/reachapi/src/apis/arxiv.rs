use super::{
    ArxivConfig, ArxivKeys, Client,
    HashMap, RawOuts, ReachApiError,
    ReachConfig, ReachConfigKeys,
};

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
                .expect("Include Keywords not available")
                .to_owned(),

            exclude_keywords: default_config
                .get(&ArxivKeys::ExcludeWords.as_str())
                .expect("Exclude Keywords not available")
                .to_owned(),

            _authors: default_config
                .get(&ArxivKeys::Authors.as_str())
                .expect("Authors not available")
                .to_owned(),

            categories: default_config
                .get(&ArxivKeys::Categories.as_str())
                .expect("Categories not available")
                .to_owned(),

            start: "0",
            max_results: "500",
            sort_by: "submittedDate",
            sort_order: "descending",
        }
    }
}

#[derive(Debug)]
pub struct ArxivOutput {
    pub title: String,
    pub url: String,
    pub summary: String,
}

pub async fn arxive_search(
    query: Option<&str>,
    max_results: &str,
) -> Result<Vec<RawOuts>, ReachApiError> {
    let arxive_search_url = "http://export.arxiv.org/api/query";
    let client = Client::new();
    let search_query = match query {
        Some(q) => {
            let mut query_obj = ArxivQuery::default();
            query_obj.include_keywords = vec![q.to_string()];
            query_obj.max_results = max_results;

            query_obj.construct_query()
        }
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

    // XML Parsing!
    let xml_content = response.text().await?;

    let doc = roxmltree::Document::parse(&xml_content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    let mut results = Vec::new();

    // Find all entry elements
    for entry in doc.descendants().filter(|n| n.has_tag_name("entry")) {
        let title = entry
            .children()
            .find(|n| n.has_tag_name("title"))
            .and_then(|n| Some(n.text().unwrap_or("").trim().to_string()));

        let url = entry
            .children()
            .find(|n| n.has_tag_name("id"))
            .and_then(|n| Some(n.text().unwrap_or("").to_string()));

        let summary = entry
            .children()
            .find(|n| n.has_tag_name("summary"))
            .and_then(|n| Some(n.text().unwrap_or("").trim().to_string()));

        if let (Some(title), Some(url), Some(summary)) = (title, url, summary) {
            results.push(RawOuts::RawArxivOut(ArxivOutput {
                title,
                url,
                summary,
            }));
        }
    }
    Ok(results)
}

mod tests {
    #[cfg(feature = "requires_config")]
    #[tokio::test]
    async fn check_arxive_search() {
        let _res = crate::apis::arxive_search(Some("Diffusion Models"), "2")
            .await
            .unwrap();
    }
}
