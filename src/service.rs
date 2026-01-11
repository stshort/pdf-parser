//! MCP Server service implementation for PDF Reader

use crate::pdf_reader::PdfReader;
use rmcp::{
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo, Implementation, ProtocolVersion},
    tool, tool_handler, tool_router,
    ErrorData as McpError,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

/// Parameters for the read_pdf tool
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadPdfParams {
    /// Absolute path to the PDF file (relative paths are not supported)
    pub file_path: String,
}

/// Parameters for the read_pdf_page tool
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadPdfPageParams {
    /// Absolute path to the PDF file (relative paths are not supported)
    pub file_path: String,
    /// Page number (1-indexed)
    pub page: u32,
}

/// Parameters for the read_pdf_pages tool (page range)
#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadPdfPagesParams {
    /// Absolute path to the PDF file (relative paths are not supported)
    pub file_path: String,
    /// Start page number (1-indexed, inclusive)
    pub start_page: u32,
    /// End page number (1-indexed, inclusive)
    pub end_page: u32,
}

/// Parameters for the get_pdf_info tool
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetPdfInfoParams {
    /// Absolute path to the PDF file (relative paths are not supported)
    pub file_path: String,
}

/// Create a custom schema for read_pdf without $schema field
fn read_pdf_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let schema = json!({
        "type": "object",
        "description": "Parameters for the read_pdf tool",
        "properties": {
            "file_path": {
                "type": "string",
                "description": "Absolute path to the PDF file (relative paths are not supported)"
            }
        },
        "required": ["file_path"],
        "title": "ReadPdfParams"
    });
    Arc::new(schema.as_object().unwrap().clone())
}

/// Create a custom schema for read_pdf_page without $schema field
fn read_pdf_page_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let schema = json!({
        "type": "object",
        "description": "Parameters for the read_pdf_page tool",
        "properties": {
            "file_path": {
                "type": "string",
                "description": "Absolute path to the PDF file (relative paths are not supported)"
            },
            "page": {
                "type": "integer",
                "description": "Page number (1-indexed)",
                "minimum": 0,
                "format": "uint32"
            }
        },
        "required": ["file_path", "page"],
        "title": "ReadPdfPageParams"
    });
    Arc::new(schema.as_object().unwrap().clone())
}

/// Create a custom schema for read_pdf_pages (page range) without $schema field
fn read_pdf_pages_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let schema = json!({
        "type": "object",
        "description": "Parameters for the read_pdf_pages tool",
        "properties": {
            "file_path": {
                "type": "string",
                "description": "Absolute path to the PDF file (relative paths are not supported)"
            },
            "start_page": {
                "type": "integer",
                "description": "Start page number (1-indexed, inclusive)",
                "minimum": 1,
                "format": "uint32"
            },
            "end_page": {
                "type": "integer",
                "description": "End page number (1-indexed, inclusive)",
                "minimum": 1,
                "format": "uint32"
            }
        },
        "required": ["file_path", "start_page", "end_page"],
        "title": "ReadPdfPagesParams"
    });
    Arc::new(schema.as_object().unwrap().clone())
}

/// Create a custom schema for get_pdf_info without $schema field
fn get_pdf_info_schema() -> Arc<serde_json::Map<String, serde_json::Value>> {
    let schema = json!({
        "type": "object",
        "description": "Parameters for the get_pdf_info tool",
        "properties": {
            "file_path": {
                "type": "string",
                "description": "Absolute path to the PDF file (relative paths are not supported)"
            }
        },
        "required": ["file_path"],
        "title": "GetPdfInfoParams"
    });
    Arc::new(schema.as_object().unwrap().clone())
}

/// PDF Reader MCP Service that exposes PDF reading tools
#[derive(Clone)]
pub struct PdfReaderService {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl PdfReaderService {
    /// Create a new PdfReaderService instance
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Extract all text content from a PDF file
    #[tool(description = "Extract all text content from a PDF file", input_schema = read_pdf_schema())]
    async fn read_pdf(
        &self,
        params: Parameters<ReadPdfParams>,
    ) -> Result<CallToolResult, McpError> {
        let text = PdfReader::extract_text(&params.0.file_path).map_err(McpError::from)?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    /// Extract text content from a specific page of a PDF file
    #[tool(description = "Extract text content from a specific page of a PDF file", input_schema = read_pdf_page_schema())]
    async fn read_pdf_page(
        &self,
        params: Parameters<ReadPdfPageParams>,
    ) -> Result<CallToolResult, McpError> {
        let text = PdfReader::extract_page_text(&params.0.file_path, params.0.page)
            .map_err(McpError::from)?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    /// Extract text content from a range of pages in a PDF file
    #[tool(description = "Extract text content from a range of pages in a PDF file (inclusive). Ideal for distributed parsing workflows.", input_schema = read_pdf_pages_schema())]
    async fn read_pdf_pages(
        &self,
        params: Parameters<ReadPdfPagesParams>,
    ) -> Result<CallToolResult, McpError> {
        let text = PdfReader::extract_page_range_text(
            &params.0.file_path,
            params.0.start_page,
            params.0.end_page,
        )
        .map_err(McpError::from)?;
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    /// Get PDF document metadata and page count
    #[tool(description = "Get PDF document metadata and page count", input_schema = get_pdf_info_schema())]
    async fn get_pdf_info(
        &self,
        params: Parameters<GetPdfInfoParams>,
    ) -> Result<CallToolResult, McpError> {
        let info = PdfReader::get_info(&params.0.file_path).map_err(McpError::from)?;
        let json = serde_json::to_string_pretty(&info)
            .map_err(|e| McpError::internal_error(format!("JSON serialization failed: {}", e), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl rmcp::ServerHandler for PdfReaderService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "pdf-reader-mcp-server".to_string(),
                title: Some("PDF Reader MCP Server".to_string()),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "PDF Reader MCP Server provides tools for extracting text and metadata from PDF files. \
                Use 'read_pdf' to extract all text, 'read_pdf_page' to extract text from a specific page, \
                'read_pdf_pages' to extract text from a range of pages (ideal for distributed parsing), \
                or 'get_pdf_info' to get document metadata and page count.".to_string()
            ),
        }
    }
}

impl Default for PdfReaderService {
    fn default() -> Self {
        Self::new()
    }
}
