# Primary sources and verification notes

Consulted 2026-09-13. These sources establish external-tool behavior, not the correctness of the proposed service implementation. Project-specific architecture and numerical defaults in the blueprint are design decisions, not upstream guarantees. Recheck APIs and resolve exact compatible dependency versions during implementation.

## FastMCP

**[S01] FastMCP installation and versioning.** Standalone import/package, v4 documentation, and exact-version pinning guidance.
```text
https://gofastmcp.com/getting-started/installation
```

**[S02] FastMCP tools.** Typed schemas, structured outputs, explicit `ToolResult` control, annotations, and validation behavior.
```text
https://gofastmcp.com/servers/tools
```

**[S03] FastMCP background tasks.** Optional tasks package/extension and negotiated support; do not assume a decorator alone enables durable jobs for every client.
```text
https://gofastmcp.com/servers/tasks
```

**[S04] What's new in FastMCP 4.** Current protocol behavior and changed server-side APIs. Avoid copying obsolete sampling/roots patterns.
```text
https://gofastmcp.com/getting-started/whats-new
```

**[S05] Running a FastMCP server.** Stdio lifecycle and deployment transports.
```text
https://gofastmcp.com/deployment/running-server
```

**[S06] Testing a FastMCP server.** In-process client and pytest-based contract tests.
```text
https://gofastmcp.com/servers/testing
```

## Rust

**[S07] docs.rs rustdoc JSON.** Hosted downloads, compression, format-version handling, and missing-build caveats.
```text
https://docs.rs/about/rustdoc-json
```

**[S08] docs.rs build metadata.** Maintainer-selected features, targets, and rustdoc settings.
```text
https://docs.rs/about/metadata
```

**[S09] Cargo metadata.** Package, target, dependency, and resolution information with invocation-dependent configuration.
```text
https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
```

**[S10] cargo-public-api / public-api maintainer documentation.** Public API listing/diffing, direct rustdoc JSON consumption, simplification, and compatibility matrix.
```text
https://github.com/cargo-public-api/cargo-public-api
```

**[S11] rust-analyzer configuration.** Initialization options, build-script execution, and procedural-macro support.
```text
https://rust-analyzer.github.io/book/configuration.html
```

## Python

**[S12] Griffe loading.** Explicit search paths, alias handling, source/stub loading, and disabling dynamic inspection.
```text
https://mkdocstrings.github.io/griffe/guide/users/loading/
```

**[S13] Griffe checking.** Supported breaking-change analysis and direct package-version comparisons. The service's additive discovery must not rely solely on breaking-change output.
```text
https://mkdocstrings.github.io/griffe/guide/users/checking/
```

**[S14] PyPI JSON API.** Project/release metadata and distribution information.
```text
https://docs.pypi.org/api/json/
```

**[S15] Python Simple Repository API.** Artifact listings, Python compatibility, yanked status, and metadata links.
```text
https://packaging.python.org/en/latest/specifications/simple-repository-api/
```

**[S16] Python typing specification: distributing type information.** Inline types, stubs, partial stubs, and resolution precedence.
```text
https://typing.python.org/en/latest/spec/distributing.html
```

**[S17] Sphinx intersphinx.** Object inventories as links to documented targets.
```text
https://www.sphinx-doc.org/en/master/usage/extensions/intersphinx.html
```

**[S18] ty language server.** Published capability table; notably, implementation navigation is not currently listed as supported.
```text
https://docs.astral.sh/ty/features/language-server/
```

**[S19] ty CLI.** `ty server`, `ty check`, and version/configuration options.
```text
https://docs.astral.sh/ty/reference/cli/
```

**[S20] Python inspect.** Runtime inspection facilities and limits around signatures/source availability.
```text
https://docs.python.org/3/library/inspect.html
```

## Context7 and agent clients

**[S21] Context7 maintainer documentation.** Current library-resolution and documentation-query tool names and arguments.
```text
https://github.com/upstash/context7
```

**[S22] Official OpenAI Codex MCP documentation.** Stdio server registration and configuration. The first URL redirected to the second during verification.
```text
https://developers.openai.com/codex/mcp/
https://learn.chatgpt.com/docs/extend/mcp?surface=cli
```

**[S23] Official OpenAI skill documentation.** User-level local skill discovery and symlink support. The first URL redirected to the second during verification.
```text
https://developers.openai.com/codex/skills/
https://learn.chatgpt.com/docs/build-skills
```

**[S24] Claude Code MCP documentation.** Stdio registration, user scope, and connection verification.
```text
https://code.claude.com/docs/en/mcp
```

**[S25] Claude Code skill documentation.** Personal skill directories and skill packaging.
```text
https://code.claude.com/docs/en/skills
```

**[S26] Rustdoc unstable features.** Nightly-gated output and command-line options.
```text
https://doc.rust-lang.org/rustdoc/unstable-features.html
```
