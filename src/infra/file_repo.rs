use crate::core::{CmsResult, ContentRepository};
use crate::domain::ContentItem;
use async_trait::async_trait;
use dashmap::DashMap;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

/// Concurrent in-memory + file-backed repository.
/// Uses DashMap for lock-free concurrent access (parallelism).
pub struct ConcurrentFileRepository {
    items: Arc<DashMap<Uuid, ContentItem>>,
    by_slug: Arc<DashMap<String, Uuid>>,
    storage_path: PathBuf,
}

impl ConcurrentFileRepository {
    pub fn new(storage_path: impl Into<PathBuf>) -> CmsResult<Self> {
        let path = storage_path.into();
        std::fs::create_dir_all(&path)?;
        let repo = Self {
            items: Arc::new(DashMap::new()),
            by_slug: Arc::new(DashMap::new()),
            storage_path: path,
        };
        repo.load_from_disk()?;
        Ok(repo)
    }

    fn load_from_disk(&self) -> CmsResult<()> {
        let index = self.storage_path.join("index.json");
        if !index.exists() {
            return Ok(());
        }
        let data = std::fs::read_to_string(&index)?;
        let items: Vec<ContentItem> = serde_json::from_str(&data)?;
        for item in items {
            self.by_slug.insert(item.slug.clone(), item.id);
            self.items.insert(item.id, item);
        }
        Ok(())
    }

    fn persist(&self) -> CmsResult<()> {
        let items: Vec<ContentItem> = self.items.iter().map(|e| e.value().clone()).collect();
        let data = serde_json::to_string_pretty(&items)?;
        std::fs::write(self.storage_path.join("index.json"), data)?;
        Ok(())
    }
}

#[async_trait]
impl ContentRepository for ConcurrentFileRepository {
    async fn get_all(&self) -> CmsResult<Vec<ContentItem>> {
        Ok(self.items.iter().map(|e| e.value().clone()).collect())
    }

    async fn get_by_slug(&self, slug: &str) -> CmsResult<Option<ContentItem>> {
        Ok(self
            .by_slug
            .get(slug)
            .and_then(|id| self.items.get(&id).map(|e| e.value().clone())))
    }

    async fn get_by_id(&self, id: Uuid) -> CmsResult<Option<ContentItem>> {
        Ok(self.items.get(&id).map(|e| e.value().clone()))
    }

    async fn save(&self, item: &ContentItem) -> CmsResult<()> {
        self.by_slug.insert(item.slug.clone(), item.id);
        self.items.insert(item.id, item.clone());
        self.persist()?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> CmsResult<()> {
        if let Some((_, item)) = self.items.remove(&id) {
            self.by_slug.remove(&item.slug);
            self.persist()?;
        }
        Ok(())
    }

    async fn list_published(&self) -> CmsResult<Vec<ContentItem>> {
        Ok(self
            .items
            .iter()
            .filter(|e| e.value().is_published())
            .map(|e| e.value().clone())
            .collect())
    }
}
