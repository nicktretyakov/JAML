use crate::core::{CmsResult, MarkdownProcessor};
use pulldown_cmark::{html, Options, Parser};

/// Concrete MarkdownProcessor – Single Responsibility.
pub struct PulldownMarkdownProcessor;

impl MarkdownProcessor for PulldownMarkdownProcessor {
    fn to_html(&self, markdown: &str) -> CmsResult<String> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}

impl Default for PulldownMarkdownProcessor {
    fn default() -> Self {
        Self
    }
}
