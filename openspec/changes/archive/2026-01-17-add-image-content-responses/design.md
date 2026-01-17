## Context

LLMs with vision capabilities (Claude, GPT-4V) can process images in two ways:
1. **Inline**: `ImageContent` in MCP response with base64 data - but stays in context forever
2. **File-based**: Read image from file path - only loaded when needed, doesn't bloat context

Currently, viewpoint-mcp saves screenshots to unpredictable locations and only returns text.

Playwright-MCP's approach:
1. Captures screenshot as PNG/JPEG bytes
2. Scales image down to fit Claude's vision limits (1568px max dimension, 1.15MP max)
3. Returns both `TextContent` (description) and `ImageContent` (base64 image) in the same response
4. Uses timestamp-based filenames: `page-{ISO-timestamp}.{ext}`
5. Has config option `imageResponses: 'omit' | 'include'` to control behavior

## Goals / Non-Goals

**Goals:**
- Save screenshots to a predictable directory so LLMs can read via file tools (default)
- Optionally return `ImageContent` in MCP responses for LLMs that can't read files
- Scale images to fit LLM vision processing limits when using inline mode
- Maintain backward compatibility (text-only responses still work)

**Non-Goals:**
- Streaming large images (out of scope)
- Supporting other image sources (only screenshots for now)
- Auto-cleanup of old screenshots (future work)

## Decisions

### Decision 1: Default screenshot directory `.viewpoint-mcp-screenshots/`

Save all screenshots to `.viewpoint-mcp-screenshots/` in the current working directory by default.

```
.viewpoint-mcp-screenshots/
├── page-2026-01-13T15-30-45-123Z.png
├── page-2026-01-13T15-31-02-456Z.png
└── page-2026-01-13T15-32-18-789Z.png
```

**Rationale:** 
- Predictable location - LLM knows where to find files
- Relative to cwd - works with any project
- Hidden directory (dot prefix) - doesn't clutter project root
- LLM can use `mcp_read` to view images without context bloat

**Alternatives considered:**
- `/tmp/viewpoint-screenshots`: Not project-local, harder to find
- User home directory: Pollutes user space
- Require explicit `--screenshot-dir`: Less convenient default

### Decision 2: `--screenshot-dir` CLI flag for override

```bash
viewpoint-mcp --screenshot-dir /path/to/screenshots
```

**Rationale:** Users may want custom locations (e.g., shared directory, specific project folder).

### Decision 3: Separate `--image-responses` flag for delivery method

```bash
# Default - save to file, return relative path in text
viewpoint-mcp
viewpoint-mcp --image-responses=file

# Save to file AND return base64 in response (for LLMs that can't read files)
viewpoint-mcp --image-responses=inline

# Just confirmation text, no path or image (minimal response)
viewpoint-mcp --image-responses=omit
```

| Mode | File Saved | Text Response | Image Response |
|------|------------|---------------|----------------|
| `file` (default) | Yes | Relative path + description | No |
| `inline` | Yes | Relative path + description | Yes (scaled base64) |
| `omit` | Yes | Confirmation only | No |

**Rationale:**
- Separates image delivery concern from `--caps=vision` (which is for coordinate-based tools)
- Default `file` is most context-efficient - LLM reads when needed
- `inline` available for LLMs that lack file reading with vision
- `omit` for minimal responses when images aren't needed

**Alternatives considered:**
- Overload `--caps=vision`: Conflates two concerns (coordinate tools vs image delivery)
- Per-call parameter: More flexible but adds complexity to every screenshot call

### Decision 4: Enum-based `ContentItem` type

```rust
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ContentItem {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image")]
    Image { 
        data: String,
        #[serde(rename = "mimeType")]
        mime_type: String 
    },
}
```

**Rationale:** Matches MCP protocol spec. Tagged enum with `type` field serializes correctly.

### Decision 5: Scale images before returning inline

Scale to fit Claude's documented limits (only for inline mode):
- Max 1568px on any dimension
- Max 1.15 megapixels total
- Use JPEG at quality 80 for ~5x smaller size vs PNG

**Rationale:** Prevents context bloat when inline images are used.

Note: Full-resolution image is always saved to file regardless of mode.

### Decision 6: Return relative file path in text response

```rust
// --image-responses=file (default)
Ok(ToolOutput {
    content: vec![
        ContentItem::Text { 
            text: "Screenshot saved to .viewpoint-mcp-screenshots/page-2026-01-13T15-30-45-123Z.png (viewport)" 
        },
    ],
})

// --image-responses=inline
Ok(ToolOutput {
    content: vec![
        ContentItem::Text { 
            text: "Screenshot saved to .viewpoint-mcp-screenshots/page-2026-01-13T15-30-45-123Z.png (viewport)" 
        },
        ContentItem::Image { data: base64_data, mime_type: "image/jpeg" },
    ],
})

// --image-responses=omit
Ok(ToolOutput {
    content: vec![
        ContentItem::Text { 
            text: "Screenshot captured (viewport)" 
        },
    ],
})
```

**Rationale:** 
- Relative path is shorter in context and more portable
- LLM typically has access to cwd, so relative paths work
- Text describes what was captured (viewport/full page/element description)

### Decision 7: Timestamp-based filenames (like Playwright)

Format: `page-{ISO-timestamp}.{ext}` where timestamp has colons/dots replaced with dashes.

Example: `page-2026-01-13T15-30-45-123Z.png`

**Rationale:**
- Matches Playwright-MCP convention
- Simple, no need to encode context in filename
- Text response describes what was captured
- Chronological ordering when listing directory

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Context bloat from inline images | Scale + JPEG compression; `file` mode is default |
| Breaking existing tool implementations | `ToolResult` change requires updating all tools |
| Serialization mismatch with MCP | Test with actual MCP clients |
| Screenshot directory grows unbounded | Not addressed in this proposal - future cleanup tool |

## Migration Plan

1. Add `--screenshot-dir` and `--image-responses` CLI flags
2. Add `ContentItem` enum and `ToolOutput` struct
3. Update `ToolResult` type alias (breaking change for tool implementations)
4. Update all tools to return `ToolOutput` instead of `String`
5. Update `browser_take_screenshot` to:
   - Save to screenshot directory with timestamp filename
   - Return relative file path in text with description (file/inline modes)
   - Include `ImageContent` when `--image-responses=inline`
6. Add image scaling utility for inline images

Most tools just wrap their string in `ToolOutput::text("...")` helper.
