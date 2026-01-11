//! PDF Reader MCP Server
//!
//! A Rust-based MCP Server that provides PDF reading capabilities as a Kiro Power.

mod error;
mod pdf_reader;
mod service;

pub use error::PdfError;
pub use pdf_reader::{PdfInfo, PdfReader};
pub use service::PdfReaderService;

use rmcp::{transport::io::stdio, ServiceExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the PDF Reader service
    let service = PdfReaderService::new();

    // Set up stdio transport for MCP communication
    let transport = stdio();

    // Start the server and wait for completion
    let server = service.serve(transport).await?;
    server.waiting().await?;

    Ok(())
}
