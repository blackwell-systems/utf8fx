//! LSP Server for mdfx template syntax
//!
//! Provides language server protocol support for mdfx template syntax,
//! including autocompletion for glyphs, styles, frames, components, and tech badges.
//!
//! Enable with: `cargo install mdfx-cli --features lsp`
//!
//! ## Module Structure
//!
//! - `parser` - Template parsing utilities
//! - `completions` - Completion building and context analysis
//! - `semantic_tokens` - Syntax highlighting through semantic tokens
//! - `diagnostics` - Validation and error reporting
//! - `code_actions` - Quick fixes and suggestions
//! - `color` - Color picker support
//! - `preview` - Hover preview generation with SVG data URIs
//! - `handlers` - LSP protocol handlers

mod code_actions;
mod color;
mod completions;
mod diagnostics;
mod handlers;
mod parser;
mod preview;
mod semantic_tokens;

use completions::CachedCompletions;
use mdfx::Registry;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tower_lsp::lsp_types::Url;
use tower_lsp::{Client, LspService, Server};

/// The mdfx language server
pub struct MdfxLanguageServer {
    pub(crate) client: Client,
    pub(crate) registry: Arc<Registry>,
    /// Cached document contents (URI -> text)
    pub(crate) documents: Arc<RwLock<HashMap<String, String>>>,
    /// Pre-built completion items for fast responses
    pub(crate) cached: Arc<CachedCompletions>,
}

impl MdfxLanguageServer {
    /// Create a new language server instance
    pub fn new(client: Client) -> Self {
        let registry = Registry::new().expect("Failed to load registry");

        // Pre-build all completion items at startup for fast responses
        let cached = CachedCompletions::build(&registry);

        Self {
            client,
            registry: Arc::new(registry),
            documents: Arc::new(RwLock::new(HashMap::new())),
            cached: Arc::new(cached),
        }
    }

    /// Get document content from cache or try to read from disk
    pub(crate) fn get_document_content(&self, uri: &Url) -> Option<String> {
        // First check the cache
        if let Ok(docs) = self.documents.read() {
            if let Some(content) = docs.get(uri.as_str()) {
                return Some(content.clone());
            }
        }

        // Fallback: try to read from disk (handles file:// URIs)
        if uri.scheme() == "file" {
            if let Ok(path) = uri.to_file_path() {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    return Some(content);
                }
            }
        }

        None
    }
}

/// Run the LSP server over stdio
pub async fn run_lsp_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(MdfxLanguageServer::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
