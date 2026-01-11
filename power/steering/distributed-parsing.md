---
inclusion: manual
---

# Distributed PDF Parsing Strategies

When working with large PDFs, distribute the workload across subagents for faster processing and better context management.

## Strategy 1: Naive Distribution

Best for: General documents, reports, manuals where all pages have equal importance.

### Approach

1. **Main agent**: Fetch metadata to get page count
2. **Main agent**: Divide pages into chunks (e.g., 10-20 pages per subagent)
3. **Subagents**: Each processes their assigned page range in parallel
4. **Main agent**: Aggregate results

### Example Workflow

```
Main Agent:
  1. Call get_pdf_info → page_count: 50
  2. Spawn 5 subagents, each handling 10 pages
  
Subagent 1: read_pdf_page for pages 1-10
Subagent 2: read_pdf_page for pages 11-20
Subagent 3: read_pdf_page for pages 21-30
Subagent 4: read_pdf_page for pages 31-40
Subagent 5: read_pdf_page for pages 41-50

Main Agent:
  3. Collect and synthesize results
```

### Subagent Prompt Template

```
Extract and summarize content from pages {start} to {end} of the PDF at {file_path}.

For each page, identify:
- Key topics and concepts
- Important data or findings
- Notable quotes or statements

Return a structured summary of your assigned section.
```

---

## Strategy 2: Research Paper Parsing

Best for: Academic papers, technical documents with structured abstracts and sections.

### Approach

1. **Main agent**: Fetch metadata for page count and document info
2. **Main agent**: Read first 2-3 pages to extract abstract and introduction
3. **Main agent**: Use abstract as context when spawning subagents
4. **Subagents**: Process remaining sections with awareness of paper's purpose
5. **Main agent**: Synthesize findings with abstract context

### Example Workflow

```
Main Agent:
  1. Call get_pdf_info → page_count: 25, title: "Machine Learning in Healthcare"
  2. Call read_pdf_page for pages 1-3 → Extract abstract
  3. Parse abstract to understand paper's thesis and methodology
  4. Spawn subagents with abstract context

Subagent 1 (Methods): pages 4-10, knows paper is about ML in healthcare
Subagent 2 (Results): pages 11-18, knows what outcomes to look for
Subagent 3 (Discussion): pages 19-25, can relate findings to thesis

Main Agent:
  5. Synthesize with understanding of paper's core argument
```

### Subagent Prompt Template

```
You are analyzing a section of a research paper.

Paper Context:
- Title: {title}
- Abstract: {abstract}
- Your section: {section_name} (pages {start}-{end})

Extract from your assigned pages:
- Key findings relevant to the paper's thesis
- Methodology details (if applicable)
- Data and results
- Limitations mentioned

File path: {file_path}
```

### Benefits

- Subagents understand what they're looking for
- Better extraction of relevant information
- More coherent final synthesis
- Reduced hallucination risk

---

## Choosing a Strategy

| Document Type | Recommended Strategy |
|--------------|---------------------|
| Research papers | Research Paper Parsing |
| Technical documentation | Naive Distribution |
| Legal documents | Naive Distribution with section awareness |
| Books/manuals | Naive Distribution |
| Reports with executive summary | Research Paper Parsing (use summary as context) |

## Tips

- **Chunk size**: 10-20 pages per subagent is usually optimal
- **Overlap**: Consider 1-page overlap between chunks to avoid missing context at boundaries
- **Metadata first**: Always start with `get_pdf_info` to plan your approach
- **Error handling**: Have subagents report pages that failed to parse
