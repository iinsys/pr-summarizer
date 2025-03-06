use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize)]
struct JokeResponse {
    joke: String,
}

const JOKE_APIS: &[&str] = &[
    "https://official-joke-api.appspot.com/jokes/programming/random",
    "https://v2.jokeapi.dev/joke/Programming?type=single&safe-mode",
    "https://icanhazdadjoke.com/",
];

pub async fn fetch_random_joke() -> Result<String> {
    let client = Client::new();
    
    // Try different joke APIs in sequence until one works
    for (index, &api_url) in JOKE_APIS.iter().enumerate() {
        match fetch_from_api(&client, api_url, index).await {
            Ok(joke) => return Ok(joke),
            Err(e) => log::warn!("Failed to fetch joke from {}: {}", api_url, e),
        }
    }
    
    // If all APIs fail, return a fallback joke
    Ok(get_fallback_joke())
}

async fn fetch_from_api(client: &Client, url: &str, api_index: usize) -> Result<String> {
    let request = client
        .get(url)
        .timeout(Duration::from_secs(5))
        .header("Accept", "application/json");
    
    let response = request.send().await.context("Failed to send request")?;
    
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("API returned error status: {}", response.status()));
    }
    
    let text = response.text().await.context("Failed to read response body")?;
    
    // Parse response based on the API format
    match api_index {
        0 => {
            // official-joke-api returns an array with one joke object
            let jokes: Vec<serde_json::Value> = serde_json::from_str(&text)?;
            if jokes.is_empty() {
                return Err(anyhow::anyhow!("Empty joke response"));
            }
            
            let setup = jokes[0]["setup"].as_str().unwrap_or_default();
            let punchline = jokes[0]["punchline"].as_str().unwrap_or_default();
            
            if !setup.is_empty() && !punchline.is_empty() {
                Ok(format!("{} {}", setup, punchline))
            } else {
                Err(anyhow::anyhow!("Invalid joke format"))
            }
        },
        1 => {
            // jokeapi.dev returns a single joke object
            let joke: serde_json::Value = serde_json::from_str(&text)?;
            if let Some(joke_text) = joke["joke"].as_str() {
                Ok(joke_text.to_string())
            } else {
                Err(anyhow::anyhow!("Invalid joke format"))
            }
        },
        2 => {
            // icanhazdadjoke returns a single joke
            let joke: JokeResponse = serde_json::from_str(&text)?;
            Ok(joke.joke)
        },
        _ => Err(anyhow::anyhow!("Unknown API index")),
    }
}

// Function to get programming-related fallback jokes if APIs fail
fn get_fallback_joke() -> String {
    let jokes = [
        "Why do programmers prefer dark mode? Because light attracts bugs!",
        "Why did the developer go broke? Because he used up all his cache.",
        "How many programmers does it take to change a light bulb? None, that's a hardware problem.",
        "A SQL query walks into a bar, sees two tables and asks, 'Can I join you?'",
        "Debugging: Being the detective in a crime movie where you are also the murderer.",
    ];
    
    // Use a simple random selection (not cryptographically secure but good enough for jokes)
    let joke_index = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize % jokes.len();
    
    jokes[joke_index].to_string()
}