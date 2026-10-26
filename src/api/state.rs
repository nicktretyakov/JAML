use crate::core::SiteBuilder;
use crate::domain::SiteConfig;
use crate::services::ContentService;
use std::sync::Arc;

/// Shared application state injected into handlers (DI).
pub struct AppState {
    pub content: Arc<ContentService>,
    pub builder: Arc<dyn SiteBuilder>,
    pub config: Arc<SiteConfig>,
}

impl AppState {
    pub fn new(
        content: Arc<ContentService>,
        builder: Arc<dyn SiteBuilder>,
        config: Arc<SiteConfig>,
    ) -> Self {
        Self {
            content,
            builder,
            config,
        }
    }
}
