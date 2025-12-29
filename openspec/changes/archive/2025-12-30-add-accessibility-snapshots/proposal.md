# Change: Add Accessibility Snapshots

## Why
The key innovation of playwright-mcp is using accessibility tree snapshots instead of screenshots. This enables LLMs to interact with pages using structured data rather than vision models, making interactions deterministic and fast.

## What Changes
- Add accessibility tree capture from Viewpoint pages
- Implement element reference system (`ref` strings like `s1e2`)
- Add snapshot formatting for LLM consumption
- Implement element lookup by reference for tool targeting

## Impact
- Affected specs: `accessibility-snapshots` (new capability)
- Affected code: `viewpoint-mcp/src/snapshot/`
- Dependencies: `viewpoint-core` (Page accessibility APIs)
