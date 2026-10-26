use crate::domain::{ContentItem, DomainEvent, SiteConfig, SiteManifest};
use crate::core::CmsResult;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;

/// Repository – Interface Segregation + Dependency Inversion.
#[async_trait]
pub trait ContentRepository: Send + Sync {
    async fn get_all(&self) -> CmsResult<Vec<ContentItem>>;
    async fn get_by_slug(&self, slug: &str) -> CmsResult<Option<ContentItem>>;
    async fn get_by_id(&self, id: uuid::Uuid) -> CmsResult<Option<ContentItem>>;
    async fn save(&self, item: &ContentItem) -> CmsResult<()>;
    async fn delete(&self, id: uuid::Uuid) -> CmsResult<()>;
    async fn list_published(&self) -> CmsResult<Vec<ContentItem>>;
}

/// Markdown processor abstraction.
pub trait MarkdownProcessor: Send + Sync {
    fn to_html(&self, markdown: &str) -> CmsResult<String>;
}

/// Template engine abstraction.
pub trait TemplateEngine: Send + Sync {
    fn render(&self, template_name: &str, context: &serde_json::Value) -> CmsResult<String>;
    fn register_template_file(&self, name: &str, path: &Path) -> CmsResult<()>;
}

/// Static file writer.
pub trait StaticWriter: Send + Sync {
    fn write_file(&self, relative_path: &str, content: &str) -> CmsResult<()>;
    fn copy_static_dir(&self, src: &Path, dest: &Path) -> CmsResult<()>;
    fn ensure_dir(&self, path: &Path) -> CmsResult<()>;
}

/// Build orchestrator – high-level use case.
#[async_trait]
pub trait SiteBuilder: Send + Sync {
    async fn build(&self, config: &SiteConfig) -> CmsResult<SiteManifest>;
}

/// Event bus.
pub trait EventBus: Send + Sync {
    fn publish(&self, event: DomainEvent);
    fn subscribe(&self, handler: Arc<dyn EventHandler>);
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, event: &DomainEvent);
}
