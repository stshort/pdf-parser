---
inclusion: manual
---

# PDF Reader Best Practices

## When to Use Each Tool

### get_pdf_info
Use first to understand document structure before reading content.

- Get page count for planning distributed parsing
- Check for title/author metadata
- Validate file exists and is readable

### read_pdf
Use for small documents (< 10 pages) where you need all content.

- Quick extraction of short documents
- When you need to search across entire document
- Simple summarization tasks

### read_pdf_page
Use for targeted extraction or distributed processing.

- Large documents where full extraction would exceed context
- When you only need specific sections
- Distributed parsing with subagents

## Path Requirements

All tools require **absolute paths**. Relative paths are not supported.

```json
// ✓ Correct
{ "file_path": "/home/user/documents/report.pdf" }

// ✗ Incorrect
{ "file_path": "./report.pdf" }
{ "file_path": "~/documents/report.pdf" }
```

## Handling Large Documents

For documents over 20-30 pages:

1. Don't use `read_pdf` - it may exceed context limits
2. Use `get_pdf_info` first to get page count
3. Use `read_pdf_page` iteratively or distribute to subagents
4. See `distributed-parsing.md` for strategies

## Error Recovery

### Page extraction failures
Some pages may fail due to font encoding issues. The tools handle this gracefully:
- `read_pdf` skips problematic pages and notes them
- `read_pdf_page` returns an error for that specific page

If a page fails, try adjacent pages - the content may span multiple pages.

### Encrypted documents
Password-protected PDFs will return an encryption error. There's no workaround without the password.

## Performance Tips

- Batch your page reads when possible
- Use subagents for parallel processing of large documents
- Cache `get_pdf_info` results if making multiple calls to same document
