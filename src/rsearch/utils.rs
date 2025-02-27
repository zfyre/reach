use super::ReachError;

// Using Python-Crawl4Ai process
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

mod tests {

    #[tokio::test]
    async fn get_markdown_from_python() {
        let url = "https://codeforces.com/";
        let mut binding = tokio::process::Command::new(".venv/Scripts/python.exe");
        let command = binding // Use Python from virtual environment
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