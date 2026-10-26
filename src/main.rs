//! Solid SSG CMS – Maximum complexity demonstration of:
//! - SOLID principles via traits & DI
//! - Parallelism (Rayon data-parallel + Tokio async)
//! - Static Site Generation
//! - REST API
//! - Frontend JS integration
//! - CMS serving of generated content

mod api;
mod concurrency;
mod core;
mod domain;
mod infra;
mod services;

use api::{create_router, AppState};
use concurrency::ParallelPageGenerator;
use core::SiteBuilder;
use domain::SiteConfig;
use infra::{
    ConcurrentFileRepository, FileSystemWriter, InMemoryEventBus, PulldownMarkdownProcessor,
    SimpleTemplateEngine,
};
use services::{ConcurrentBuildService, ContentService};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "solid_ssg_cms=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Solid SSG CMS starting...");

    // Configuration
    let config = Arc::new(SiteConfig {
        title: "Solid SSG CMS".into(),
        description: "Максимально сложный пример SSG + CMS на Rust с SOLID и параллелизмом".into(),
        base_url: "http://localhost:3000".into(),
        language: "ru".into(),
        output_dir: PathBuf::from("output"),
        content_dir: PathBuf::from("content"),
        templates_dir: PathBuf::from("templates"),
        static_dir: PathBuf::from("static"),
        parallel_workers: std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4),
    });

    // Ensure directories
    std::fs::create_dir_all(&config.output_dir)?;
    std::fs::create_dir_all(&config.content_dir)?;
    std::fs::create_dir_all("data")?;
    std::fs::create_dir_all("public")?;

    // === Dependency Injection (Dependency Inversion) ===
    let markdown: Arc<dyn core::MarkdownProcessor> =
        Arc::new(PulldownMarkdownProcessor::default());
    let writer: Arc<dyn core::StaticWriter> =
        Arc::new(FileSystemWriter::new(config.output_dir.clone()));
    let events: Arc<dyn core::EventBus> = Arc::new(InMemoryEventBus::new());

    // Template engine – load from disk if exists, otherwise empty
    let templates: Arc<dyn core::TemplateEngine> = match SimpleTemplateEngine::new(&config.templates_dir)
    {
        Ok(t) => Arc::new(t),
        Err(e) => {
            tracing::warn!("Templates not loaded: {}. Using empty engine.", e);
            Arc::new(SimpleTemplateEngine::empty())
        }
    };

    // Concurrent repository
    let repo: Arc<dyn core::ContentRepository> =
        Arc::new(ConcurrentFileRepository::new("data")?);

    // Parallel generator
    let generator = Arc::new(ParallelPageGenerator::new(
        markdown.clone(),
        templates.clone(),
        writer.clone(),
    ));

    // Services
    let content_service = Arc::new(ContentService::new(
        repo.clone(),
        markdown.clone(),
        events.clone(),
    ));

    let build_service: Arc<dyn SiteBuilder> = Arc::new(ConcurrentBuildService::new(
        repo.clone(),
        generator,
        writer.clone(),
        events.clone(),
    ));

    // Seed some demo content if empty
    seed_demo_content(&content_service).await?;

    // App state
    let state = Arc::new(AppState::new(
        content_service,
        build_service.clone(),
        config.clone(),
    ));

    // Initial build
    tracing::info!("Performing initial static site generation...");
    match build_service.build(&config).await {
        Ok(m) => tracing::info!(
            "✅ Initial build: {} pages in {} ms",
            m.pages.len(),
            m.build_duration_ms
        ),
        Err(e) => tracing::error!("Initial build failed: {}", e),
    }

    // Router
    let app = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("🌐 Listening on http://{}", addr);
    tracing::info!("📡 API: http://{}/api/health", addr);
    tracing::info!("📄 Static site: http://{}/site/", addr);
    tracing::info!("🖥️  Admin UI: http://{}/", addr);

    tracing::info!("binding...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn seed_demo_content(svc: &crate::services::ContentService) -> anyhow::Result<()> {
    let existing = svc.list().await?;
    if !existing.is_empty() {
        return Ok(());
    }

    tracing::info!("Seeding demo content...");

    let items = vec![
        (
            "welcome",
            "Добро пожаловать в Solid SSG CMS",
            r#"# Привет, мир!

Это **максимально сложный** пример статического генератора на Rust.

## Возможности

- SOLID-принципы через трейты
- Параллельная генерация (Rayon)
- REST API (Axum)
- JS-фронтенд
- CMS-обслуживание

```rust
fn main() {
    println!("Hello from parallel SSG!");
}
```
"#,
            crate::domain::ContentKind::Page,
            "system",
            vec!["intro".into(), "rust".into()],
        ),
        (
            "solid-principles",
            "SOLID в Rust",
            r#"# SOLID на практике

| Принцип | Как реализовано |
|---------|-----------------|
| S | Отдельные сервисы и генераторы |
| O | Трейты Renderable, ContentRepository |
| L | Все реализации взаимозаменяемы |
| I | Узкие интерфейсы (MarkdownProcessor, StaticWriter) |
| D | DI через Arc<dyn Trait> |

Параллелизм достигается через `rayon::par_iter` и `DashMap`.
"#,
            crate::domain::ContentKind::Post,
            "architect",
            vec!["solid".into(), "architecture".into()],
        ),
        (
            "parallel-build",
            "Параллельная генерация",
            r#"# Rayon + Tokio

Генерация страниц выполняется параллельно:

```rust
items.par_iter().map(|item| self.generate_one(item)).collect()
```

Количество воркеров берётся из `available_parallelism()`.
"#,
            crate::domain::ContentKind::Post,
            "engineer",
            vec!["performance".into(), "rayon".into()],
        ),
    ];

    for (slug, title, body, kind, author, tags) in items {
        let item = svc
            .create(
                slug.into(),
                title.into(),
                body.into(),
                kind,
                author.into(),
                tags,
            )
            .await?;
        svc.publish(item.id).await?;
    }

    Ok(())
}
