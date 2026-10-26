use crate::core::{CmsResult, ContentRepository, EventBus, MarkdownProcessor};
use crate::domain::{ContentItem, ContentKind, DomainEvent};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

/// Application service – orchestrates domain + infra (SRP).
pub struct ContentService {
    repo: Arc<dyn ContentRepository>,
    markdown: Arc<dyn MarkdownProcessor>,
    events: Arc<dyn EventBus>,
}

impl ContentService {
    pub fn new(
        repo: Arc<dyn ContentRepository>,
        markdown: Arc<dyn MarkdownProcessor>,
        events: Arc<dyn EventBus>,
    ) -> Self {
        Self {
            repo,
            markdown,
            events,
        }
    }

    pub async fn create(
        &self,
        slug: String,
        title: String,
        body: String,
        kind: ContentKind,
        author: String,
        tags: Vec<String>,
    ) -> CmsResult<ContentItem> {
        if self.repo.get_by_slug(&slug).await?.is_some() {
            return Err(crate::core::CmsError::Validation(format!(
                "Slug '{}' already exists",
                slug
            )));
        }

        let mut item = ContentItem::new(slug, title, body, kind, author);
        item.tags = tags;
        item.body_html = Some(self.markdown.to_html(&item.body_markdown)?);

        self.repo.save(&item).await?;
        self.events.publish(DomainEvent::ContentCreated {
            id: item.id,
            slug: item.slug.clone(),
            at: Utc::now(),
        });
        Ok(item)
    }

    pub async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        body: Option<String>,
        tags: Option<Vec<String>>,
    ) -> CmsResult<ContentItem> {
        let mut item = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| crate::core::CmsError::NotFound(id.to_string()))?;

        if let Some(t) = title {
            item.title = t;
        }
        if let Some(b) = body {
            item.body_markdown = b;
            item.body_html = Some(self.markdown.to_html(&item.body_markdown)?);
        }
        if let Some(tg) = tags {
            item.tags = tg;
        }
        item.updated_at = Utc::now();

        self.repo.save(&item).await?;
        self.events.publish(DomainEvent::ContentUpdated {
            id: item.id,
            slug: item.slug.clone(),
            at: Utc::now(),
        });
        Ok(item)
    }

    pub async fn publish(&self, id: Uuid) -> CmsResult<ContentItem> {
        let mut item = self
            .repo
            .get_by_id(id)
            .await?
            .ok_or_else(|| crate::core::CmsError::NotFound(id.to_string()))?;

        item.publish();
        self.repo.save(&item).await?;
        self.events.publish(DomainEvent::ContentPublished {
            id: item.id,
            slug: item.slug.clone(),
            at: Utc::now(),
        });
        Ok(item)
    }

    pub async fn list(&self) -> CmsResult<Vec<ContentItem>> {
        self.repo.get_all().await
    }

    pub async fn get(&self, slug: &str) -> CmsResult<Option<ContentItem>> {
        self.repo.get_by_slug(slug).await
    }

    pub async fn delete(&self, id: Uuid) -> CmsResult<()> {
        self.repo.delete(id).await
    }
}
