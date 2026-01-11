//! Error types for the PDF Reader MCP Server

use rmcp::model::ErrorData;
use thiserror::Error;

/// Errors that can occur during PDF operations
#[derive(Debug, Error)]
pub enum PdfError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid PDF format: {0}")]
    InvalidFormat(String),

    #[error("PDF parsing failed: {0}")]
    ParseError(String),

    #[error("Page {0} does not exist (document has {1} pages)")]
    PageNotFound(u32, usize),

    #[error("Document is encrypted and requires a password")]
    EncryptedDocument,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<PdfError> for ErrorData {
    fn from(err: PdfError) -> Self {
        match err {
            PdfError::FileNotFound(path) => {
                ErrorData::invalid_params(format!("File not found: {}", path), None)
            }
            PdfError::InvalidFormat(msg) => {
                ErrorData::invalid_params(format!("Invalid PDF format: {}", msg), None)
            }
            PdfError::ParseError(msg) => {
                ErrorData::internal_error(format!("PDF parsing failed: {}", msg), None)
            }
            PdfError::PageNotFound(page, total) => ErrorData::invalid_params(
                format!("Page {} does not exist (document has {} pages)", page, total),
                None,
            ),
            PdfError::EncryptedDocument => ErrorData::invalid_params(
                "Document is encrypted and requires a password",
                None,
            ),
            PdfError::IoError(e) => {
                ErrorData::internal_error(format!("IO error: {}", e), None)
            }
        }
    }
}
