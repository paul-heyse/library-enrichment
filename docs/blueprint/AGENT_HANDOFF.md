# Programming-agent handoff

Implement the standalone `library-enrichment` repository described in `IMPLEMENTATION_BLUEPRINT.md`. Treat that document as the governing specification and this file as the execution brief.

## Fixed choices

Use a Rust-owned core and immutable Arrow/Parquet evidence snapshots, with DataFusion for structured retrieval. Expose a thin FastMCP **4** interface. Use hosted rustdoc JSON before local Rust compilation, use Griffe for Python static public API extraction, and use **ty rather than Pyrefly** for Python semantic analysis. Keep Context7 as a separate MCP service called by the agent. Do not embed an LLM or implement the capability graph.

Service implementation, tool environments, extraction workspaces, caches, and outputs belong outside all working repositories. No package execution in the MCP process or the working repository. Use service-owned capsules and enforced execution profiles.

## Deliverables

Produce real working code, not only scaffold files. Implement the nine tools and resource/read equivalents; schema-versioned outputs; provenance and exact-environment handling; jobs, pagination, caching, and partial/error outcomes; fixture/live tests; reproducible dependency locks; installation and client-registration instructions; and the companion `library-research` skill.

The provided skill is a starting artifact. Keep its contract synchronized with the implementation and prove it is discoverable by the supported clients. Do not install it into user configuration without explicit setup invocation.

## Execution order

Start with the compatibility matrix and a minimal tested MCP/daemon handshake. Then deliver a complete static Rust slice, a complete static Python slice, release comparison and bounded search, semantic/verification paths, real client acceptance, and operational hardening. Use the acceptance gates in the blueprint; do not leave the actual MCP path until the end.

Verify current upstream APIs and pin compatible tool versions. The blueprint specifies architecture and behavior, not an assurance that every dependency's latest release composes correctly. Record necessary deviations in an ADR with evidence and tests; preserve the binding boundaries.

## Done means

From an unrelated directory, an agent can discover the installed service, resolve a real Rust crate and a real Python distribution, identify a relevant capability, retrieve exact supporting evidence, and verify a minimal usage pattern in an isolated environment. Both correct and incorrect snippets produce truthful results. Old snapshots remain readable, large outputs remain bounded, and incomplete evidence is never presented as complete.

Run all feasible tests. For external clients, missing credentials, unsupported platforms, or unavailable tools, distinguish unexecuted tests from passing tests and provide exact reproduction steps. Do not claim success from mocks or static screenshots.

The final delivery should state the implemented commit, lockfile/tool versions, commands actually executed, test outcomes, known limitations, and setup commands. Do not replace implementation with another long planning document.
