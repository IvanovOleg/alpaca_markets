use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// News Article Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    /// Unique article ID
    pub id: u64,
    /// Article headline/title
    pub headline: String,
    /// Author of the article
    pub author: Option<String>,
    /// Publication timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: Option<DateTime<Utc>>,
    /// Article summary/excerpt
    pub summary: Option<String>,
    /// Full article content (if include_content=true)
    pub content: Option<String>,
    /// Associated symbols
    pub symbols: Option<Vec<String>>,
    /// Article URL
    pub url: Option<String>,
    /// Array of image URLs
    pub images: Option<Vec<NewsImage>>,
}

/// News Image Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsImage {
    /// Image size (e.g., "thumb", "small", "large")
    pub size: Option<String>,
    /// Image URL
    pub url: String,
}

/// Response for news articles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsResponse {
    /// List of news articles
    pub news: Vec<NewsArticle>,
    /// Next page token for pagination
    pub next_page_token: Option<String>,
}
