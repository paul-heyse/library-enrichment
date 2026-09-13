# Blueprint artifact validation

Prepared 2026-09-13. These checks concern the delivered specification files, not a running enrichment service.

## Checks executed

- JSON Schema validated with a Draft 2020-12 validator.
- All four synthetic response examples validated, including URI/date-time format checks where applicable.
- Three negative cases correctly rejected: pending result without a job, error result without an error object, and an unknown root field.
- TOML example parsed successfully; the Python semantic-engine setting is `ty`.
- Skill YAML frontmatter parsed successfully; name and description are present.
- Markdown fenced blocks are balanced.
- Source reference IDs resolve to the source catalog.
- Skill reference files exist at their relative paths.

## Not executed

No service has been implemented or started. No package extraction, library compilation, Griffe/ty/rust-analyzer integration, runtime probe, FastMCP client integration, or actual Codex/Claude acceptance test has been performed as part of this blueprint deliverable. Exact dependency pins and all operational acceptance tests remain implementation tasks.

The JSON examples are explicitly synthetic and must not be presented as observations about real packages.
