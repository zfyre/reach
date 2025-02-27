use super::{ReachError, Value};

// Using Python-Crawl4Ai process
#[allow(dead_code)]
pub async fn get_markdown(url: &str) -> Result<String, ReachError> {
    let output = tokio::process::Command::new(".venv/Scripts/python.exe") // Use Python from virtual environment
        .arg("src/scripts/crawl.py")
        .arg(&format!("--url={}", url))
        .output()
        .await?;

    if !output.status.success() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    let result = String::from_utf8_lossy(&output.stdout).into(); // .into() converts the Cow to the Owned type because we are returning Result<String, Err>
    Ok(result)
}

pub fn append_to_json(value: &Value, file_path: &str) -> Result<(), ReachError> {
    let existing_json = if std::path::Path::new(file_path).exists() {
        std::fs::read_to_string(file_path)?
    } else {
        "{}".to_string()
    };

    let mut existing_data: Value = if existing_json.trim().is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_str(&existing_json)?
    };

    if let (Value::Object(ref mut map), Value::Object(new_map)) = (&mut existing_data, value) {
        map.extend(new_map.into_iter().map(|(k, v)| (k.clone(), v.clone())));
    }

    let json_string = serde_json::to_string_pretty(&existing_data)?;
    std::fs::write(file_path, json_string)?;

    Ok(())
}
mod tests {

    #[tokio::test]
    async fn get_markdown_from_python() {
        let url = "https://codeforces.com/";
        let python_path = if cfg!(windows) {
            ".venv/Scripts/python.exe"
        } else {
            ".venv/bin/python"  // Unix-like systems (Linux/MacOS)
        };

        let mut binding = tokio::process::Command::new(python_path);
        let command = binding
            .arg("src/scripts/crawl.py")
            .arg(&format!("--url={}", url));

        println!("Running command: {:?}", command);

        let output = command.output().await.unwrap();

        if !output.status.success() {
            eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }

        let result = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", result);
    }
}