---
name: pdf-reader
description: Extract text and metadata from PDF documents
keywords:
  - pdf
  - document
  - text extraction
  - metadata
---

# PDF Reader MCP Server

A Rust-based MCP (Model Context Protocol) server that provides PDF reading capabilities for extracting text and metadata from PDF documents.

> **Note**: This is a source project. To use as a Kiro Power, build the binary first with `cargo build --release`, then configure your MCP settings to point to the built binary.

## Available Tools

### read_pdf

Extract all text content from a PDF file.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| file_path | string | Yes | Absolute path to the PDF file |

**Example:**
```json
{
  "file_path": "/path/to/document.pdf"
}
```

**Response:**
```json
{
  "content": [
    {
      "type": "text",
      "text": "Extracted text content from the PDF..."
    }
  ]
}
```

---

### read_pdf_page

Extract text content from a specific page of a PDF file.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| file_path | string | Yes | Absolute path to the PDF file |
| page | integer | Yes | Page number (1-indexed) |

**Example:**
```json
{
  "file_path": "/path/to/document.pdf",
  "page": 1
}
```

**Response:**
```json
{
  "content": [
    {
      "type": "text",
      "text": "Text content from page 1..."
    }
  ]
}
```

---

### read_pdf_pages

Extract text content from a range of pages in a PDF file. Ideal for distributed parsing workflows where subagents process different sections of a document.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| file_path | string | Yes | Absolute path to the PDF file |
| start_page | integer | Yes | Start page number (1-indexed, inclusive) |
| end_page | integer | Yes | End page number (1-indexed, inclusive) |

**Example:**
```json
{
  "file_path": "/path/to/document.pdf",
  "start_page": 1,
  "end_page": 10
}
```

**Response:**
```json
{
  "content": [
    {
      "type": "text",
      "text": "Text content from pages 1-10..."
    }
  ]
}
```

---

### get_pdf_info

Get PDF document metadata and page count.

**Parameters:**
| Name | Type | Required | Description |
|------|------|----------|-------------|
| file_path | string | Yes | Absolute path to the PDF file |

**Example:**
```json
{
  "file_path": "/path/to/document.pdf"
}
```

**Response:**
```json
{
  "content": [
    {
      "type": "text",
      "text": "{\n  \"page_count\": 10,\n  \"title\": \"Document Title\",\n  \"author\": \"Author Name\",\n  \"subject\": \"Subject\",\n  \"creator\": \"Creator App\"\n}"
    }
  ]
}
```

## Installation

### Prerequisites

- Rust toolchain (1.70 or later)
- Cargo package manager

### Building from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd pdf-reader-mcp-server
   ```

2. Build the release binary:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/pdf-reader-mcp-server`

## Configuration

### Kiro MCP Configuration

Add the following to your `.kiro/settings/mcp.json` file:

```json
{
  "mcpServers": {
    "pdf-reader": {
      "command": "/path/to/pdf-reader-mcp-server",
      "args": [],
      "disabled": false,
      "autoApprove": [
        "read_pdf",
        "read_pdf_page",
        "read_pdf_pages",
        "get_pdf_info"
      ]
    }
  }
}
```

Replace `/path/to/pdf-reader-mcp-server` with the actual path to your built binary.

### Using Cargo Run (Development)

For development, you can configure Kiro to run the server via Cargo:

```json
{
  "mcpServers": {
    "pdf-reader": {
      "command": "cargo",
      "args": ["run", "--release", "--manifest-path", "/path/to/pdf-reader-mcp-server/Cargo.toml"],
      "disabled": false,
      "autoApprove": []
    }
  }
}
```

## Error Handling

The server returns descriptive error messages for common failure scenarios:

| Error | Description |
|-------|-------------|
| File not found | The specified PDF file does not exist |
| Invalid PDF format | The file is not a valid PDF document |
| PDF parsing failed | The PDF could not be parsed (may be corrupted) |
| Page not found | The requested page number exceeds the document's page count |
| Document encrypted | The PDF is password-protected |

## License

MIT

## Steering Guides

This power includes steering files for optimal usage:

- `steering/best-practices.md` - General usage guidelines and tips
- `steering/distributed-parsing.md` - Strategies for processing large PDFs with subagents
