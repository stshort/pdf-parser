//! PDF Reader MCP Server Library
//!
//! A Rust-based MCP Server that provides PDF reading capabilities as a Kiro Power.

pub mod error;
pub mod pdf_reader;
pub mod service;

pub use error::PdfError;
pub use pdf_reader::{PdfInfo, PdfReader};
pub use service::PdfReaderService;
