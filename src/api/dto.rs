use crate::domain::{ContentKind, ContentStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateContentRequest {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub kind: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateContentRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ContentResponse {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: Option<String>,
    pub kind: ContentKind,
    pub status: ContentStatus,
    pub tags: Vec<String>,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: Option<String>,
}

impl From<crate::domain::ContentItem> for ContentResponse {
    fn from(item: crate::domain::ContentItem) -> Self {
        Self {
            id: item.id,
            slug: item.slug,
            title: item.title,
            body_markdown: item.body_markdown,
            body_html: item.body_html,
            kind: item.kind,
            status: item.status,
            tags: item.tags,
            author: item.author,
            created_at: item.created_at.to_rfc3339(),
            updated_at: item.updated_at.to_rfc3339(),
            published_at: item.published_at.map(|t| t.to_rfc3339()),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BuildResponse {
    pub success: bool,
    pub pages: usize,
    pub duration_ms: u64,
    pub generated_at: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}
