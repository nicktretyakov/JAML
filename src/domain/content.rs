use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Content status – Open/Closed friendly via enum.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    Published,
    Archived,
}

/// Content type – Interface Segregation: different kinds implement different capabilities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Post,
    Page,
    Product,
    Custom(String),
}

/// Core content entity – Single Responsibility: holds only content data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentItem {
    pub id: Uuid,
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: Option<String>,
    pub kind: ContentKind,
    pub status: ContentStatus,
    pub tags: Vec<String>,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}

impl ContentItem {
    pub fn new(
        slug: impl Into<String>,
        title: impl Into<String>,
        body_markdown: impl Into<String>,
        kind: ContentKind,
        author: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            slug: slug.into(),
            title: title.into(),
            body_markdown: body_markdown.into(),
            body_html: None,
            kind,
            status: ContentStatus::Draft,
            tags: Vec::new(),
            author: author.into(),
            created_at: now,
            updated_at: now,
            published_at: None,
            metadata: serde_json::json!({}),
        }
    }

    pub fn publish(&mut self) {
        self.status = ContentStatus::Published;
        self.published_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn is_published(&self) -> bool {
        matches!(self.status, ContentStatus::Published)
    }
}

/// Trait for anything that can be rendered to HTML (Liskov + Interface Segregation).
pub trait Renderable: Send + Sync {
    fn render_key(&self) -> &str;
    fn template_name(&self) -> &str;
    fn context(&self) -> serde_json::Value;
}

impl Renderable for ContentItem {
    fn render_key(&self) -> &str {
        &self.slug
    }

    fn template_name(&self) -> &str {
        match self.kind {
            ContentKind::Post => "post.html",
            ContentKind::Page => "page.html",
            ContentKind::Product => "product.html",
            ContentKind::Custom(_) => "custom.html",
        }
    }

    fn context(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.id,
            "slug": self.slug,
            "title": self.title,
            "body_html": self.body_html,
            "body_markdown": self.body_markdown,
            "kind": self.kind,
            "status": self.status,
            "tags": self.tags,
            "author": self.author,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "published_at": self.published_at,
            "metadata": self.metadata,
        })
    }
}
