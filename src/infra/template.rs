use crate::core::{CmsError, CmsResult, TemplateEngine};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::path::Path;

/// Minimal template engine – string replace of {{ key }} placeholders.
/// Avoids heavy Tera dependency for older Rust toolchains.
pub struct SimpleTemplateEngine {
    templates: RwLock<HashMap<String, String>>,
}

impl SimpleTemplateEngine {
    pub fn new(templates_dir: &Path) -> CmsResult<Self> {
        let engine = Self {
            templates: RwLock::new(HashMap::new()),
        };
        if templates_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(templates_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().map(|e| e == "html").unwrap_or(false) {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                engine.templates.write().insert(name.to_string(), content);
                            }
                        }
                    }
                }
            }
        }
        Ok(engine)
    }

    pub fn empty() -> Self {
        Self {
            templates: RwLock::new(HashMap::new()),
        }
    }
}

fn render_simple(template: &str, context: &serde_json::Value) -> String {
    let mut out = template.to_string();
    if let Some(obj) = context.as_object() {
        for (k, v) in obj {
            let placeholder = format!("{{{{ {} }}}}", k);
            let replacement = match v {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => String::new(),
                other => other.to_string().trim_matches('"').to_string(),
            };
            out = out.replace(&placeholder, &replacement);
        }
    }
    // crude support for body_html | safe style – already replaced above
    out = out.replace(" | safe", "");
    out
}

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, template_name: &str, context: &serde_json::Value) -> CmsResult<String> {
        let templates = self.templates.read();
        let tpl = templates.get(template_name).ok_or_else(|| {
            CmsError::Template(format!("template '{}' not found", template_name))
        })?;
        Ok(render_simple(tpl, context))
    }

    fn register_template_file(&self, name: &str, path: &Path) -> CmsResult<()> {
        let content = std::fs::read_to_string(path)?;
        self.templates.write().insert(name.to_string(), content);
        Ok(())
    }
}
