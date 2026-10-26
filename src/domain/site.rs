use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub description: String,
    pub base_url: String,
    pub language: String,
    pub output_dir: PathBuf,
    pub content_dir: PathBuf,
    pub templates_dir: PathBuf,
    pub static_dir: PathBuf,
    pub parallel_workers: usize,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "Solid SSG CMS".into(),
            description: "Maximum complexity Rust SSG + CMS".into(),
            base_url: "http://localhost:3000".into(),
            language: "ru".into(),
            output_dir: PathBuf::from("output"),
            content_dir: PathBuf::from("content"),
            templates_dir: PathBuf::from("templates"),
            static_dir: PathBuf::from("static"),
            parallel_workers: num_cpus_fallback(),
        }
    }
}

fn num_cpus_fallback() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteManifest {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub pages: Vec<String>,
    pub total_items: usize,
    pub build_duration_ms: u64,
}
