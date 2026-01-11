# PDF Reader MCP Server

A Rust-based MCP (Model Context Protocol) server that provides PDF reading capabilities for extracting text and metadata from PDF documents.

## Features

- Extract all text content from PDF files
- Extract text from specific pages
- Retrieve document metadata (title, author, subject, creator, page count)
- Graceful handling of problematic pages with font encoding issues
- Support for encrypted document detection

## Tools

| Tool | Description |
|------|-------------|
| `read_pdf` | Extract all text content from a PDF file |
| `read_pdf_page` | Extract text from a specific page (1-indexed) |
| `get_pdf_info` | Get document metadata and page count |

All tools require an absolute file path.

## Installation

### Prerequisites

- Rust 1.70+

### Build

```bash
cargo build --release
```

The binary will be at `target/release/pdf-reader-mcp-server`.

## Configuration

### Kiro

Add to `.kiro/settings/mcp.json`:

```json
{
  "mcpServers": {
    "pdf-reader": {
      "command": "/path/to/pdf-reader-mcp-server",
      "args": [],
      "disabled": false,
      "autoApprove": ["read_pdf", "read_pdf_page", "get_pdf_info"]
    }
  }
}
```

### Kiro Power

This project includes a `power/` directory for use as a Kiro Power. See `power/POWER.md` for details.

## Usage Examples

### Extract all text

```json
{
  "file_path": "/home/user/documents/report.pdf"
}
```

### Extract specific page

```json
{
  "file_path": "/home/user/documents/report.pdf",
  "page": 3
}
```

### Get document info

```json
{
  "file_path": "/home/user/documents/report.pdf"
}
```

Returns:
```json
{
  "page_count": 10,
  "title": "Annual Report",
  "author": "Jane Doe"
}
```

## Error Handling

| Error | Description |
|-------|-------------|
| File not found | The specified PDF file does not exist |
| Invalid PDF format | The file is not a valid PDF document |
| PDF parsing failed | The PDF could not be parsed |
| Page not found | Requested page exceeds document page count |
| Document encrypted | The PDF is password-protected |

## License

MIT
