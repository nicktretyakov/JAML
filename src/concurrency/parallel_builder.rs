use crate::core::{CmsResult, MarkdownProcessor, StaticWriter, TemplateEngine};
use crate::domain::{ContentItem, Renderable, SiteConfig};
use rayon::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

/// Parallel page generator using Rayon (data parallelism).
pub struct ParallelPageGenerator {
    markdown: Arc<dyn MarkdownProcessor>,
    templates: Arc<dyn TemplateEngine>,
    writer: Arc<dyn StaticWriter>,
}

impl ParallelPageGenerator {
    pub fn new(
        markdown: Arc<dyn MarkdownProcessor>,
        templates: Arc<dyn TemplateEngine>,
        writer: Arc<dyn StaticWriter>,
    ) -> Self {
        Self {
            markdown,
            templates,
            writer,
        }
    }

    /// Process all content items in parallel.
    pub fn generate_all(
        &self,
        items: &[ContentItem],
        config: &SiteConfig,
    ) -> CmsResult<(Vec<String>, u64)> {
        let start = Instant::now();

        // Configure Rayon thread pool size from config
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(config.parallel_workers)
            .build()
            .map_err(|e| crate::core::CmsError::Concurrency(e.to_string()))?;

        let results: Vec<CmsResult<String>> = pool.install(|| {
            items
                .par_iter()
                .map(|item| self.generate_one(item, config))
                .collect()
        });

        let mut pages = Vec::new();
        for r in results {
            match r {
                Ok(path) => pages.push(path),
                Err(e) => warn!("Failed to generate page: {}", e),
            }
        }

        let duration = start.elapsed().as_millis() as u64;
        info!(
            "Parallel generation finished: {} pages in {} ms (workers={})",
            pages.len(),
            duration,
            config.parallel_workers
        );
        Ok((pages, duration))
    }

    fn generate_one(&self, item: &ContentItem, _config: &SiteConfig) -> CmsResult<String> {
        let mut processed = item.clone();
        if processed.body_html.is_none() {
            processed.body_html = Some(self.markdown.to_html(&processed.body_markdown)?);
        }

        let html = self
            .templates
            .render(processed.template_name(), &processed.context())?;

        let relative = format!("{}.html", processed.slug);
        self.writer.write_file(&relative, &html)?;
        Ok(relative)
    }
}
