use anyhow::{Context, Result};
use serde_json::Value;
use std::env;
use std::fs;

pub fn read_github_event(event_path: &str) -> Result<Value> {
    let content =
        fs::read_to_string(event_path).context(format!("Failed to read file: {}", event_path))?;

    let json: Value = serde_json::from_str(&content).context("Failed to parse event JSON")?;

    Ok(json)
}

pub fn extract_pr_number(event: &Value) -> Result<u64> {
    // Check if this is a pull_request event
    if let Some(pr_number) = event["pull_request"]["number"].as_u64() {
        return Ok(pr_number);
    }

    // Check if this is a pull_request_target event
    if let Some(pr_number) = event["pull_request_target"]["number"].as_u64() {
        return Ok(pr_number);
    }

    // Check if issue_comment on a PR
    if let Some(issue) = event["issue"].as_object() {
        if issue.contains_key("pull_request") {
            if let Some(number) = issue["number"].as_u64() {
                return Ok(number);
            }
        }
    }

    Err(anyhow::anyhow!("Could not find PR number in event payload"))
}

pub fn get_repository_info() -> Result<(String, String)> {
    let repo =
        env::var("GITHUB_REPOSITORY").context("GITHUB_REPOSITORY environment variable not set")?;

    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("Invalid repository format: {}", repo));
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}
