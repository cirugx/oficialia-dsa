//! Google Gemini API Client

use reqwest::Client;
use serde::{Deserialize, Serialize};

const GEMINI_API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta";

/// Google Gemini API client
pub struct GeminiClient {
    client: Client,
    api_key: String,
}

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Debug, Serialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
struct GeminiPart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Debug, Deserialize)]
struct GeminiCandidate {
    content: GeminiResponseContent,
}

#[derive(Debug, Deserialize)]
struct GeminiResponseContent {
    parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
struct GeminiResponsePart {
    text: String,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Generate content using Gemini
    pub async fn generate_content(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!("{}/models/gemini-pro:generateContent?key={}", 
                         GEMINI_API_BASE, self.api_key);

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("Gemini API error: {}", response.status()).into());
        }

        let gemini_response: GeminiResponse = response.json().await?;
        
        let text = gemini_response
            .candidates
            .and_then(|candidates| candidates.into_iter().next())
            .and_then(|candidate| candidate.content.parts.into_iter().next())
            .map(|part| part.text)
            .unwrap_or_default();

        Ok(text)
    }

    /// Summarize text using Gemini
    pub async fn summarize(&self, text: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt = format!("Please provide a concise summary of the following text:\n\n{}", text);
        self.generate_content(&prompt).await
    }

    /// Extract key points from text
    pub async fn extract_key_points(&self, text: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let prompt = format!("Extract the key points from the following text as a bulleted list:\n\n{}", text);
        let response = self.generate_content(&prompt).await?;
        
        // Parse bullet points (simple implementation)
        let points = response
            .lines()
            .filter(|line| line.trim().starts_with('-') || line.trim().starts_with('•'))
            .map(|line| line.trim().trim_start_matches('-').trim_start_matches('•').trim().to_string())
            .collect();

        Ok(points)
    }
}
