# library-enrichment

A local-first library-evidence service for Rust and Python, exposed over MCP.

It answers the question a package index cannot: *given a design objective and a concrete
dependency environment, what relevant library capabilities exist, what evidence supports
their use, what configuration do they require, and what remains unverified?*

The service produces **evidence**. The calling agent produces **judgments**. It complements
[Context7](https://github.com/upstash/context7) — Context7 explains concepts, this service
establishes exact-release facts about the dependencies a project actually resolved.

## Status

**Phase 0 of 6.** See [`STATUS.md`](STATUS.md) for the current gate tally and
[`docs/reports/acceptance.md`](docs/reports/) for per-gate results. This repository is under
active construction; the governing specification is complete and frozen, the implementation
is not.

## What it does

Nine MCP tools over a shared evidence core:

| Tool | Purpose |
|---|---|
| `resolve_library` | Establish exact release identity and environment before research |
| `library_overview` | Discover unfamiliar capabilities without knowing symbol names |
| `search_evidence` | Search a bounded set of API, docs, example, source, or release evidence |
| `inspect_symbol` | Characterize a candidate and its deployment requirements |
| `compare_releases` | Discover additions, removals, and non-API changes |
| `verify_usage` | Test a proposed invocation in an isolated capsule |
| `read_artifact` | Retrieve large result sections without flooding context |
| `job_control` | Observe or cancel a long-running operation |
| `service_status` | Inspect readiness and capabilities |

Evidence carries provenance: every material fact traces to an exact artifact, a producer
run, and a recorded environment. Declared availability, configured availability, type-check
success, and runtime behavior are **separate states** — the service never collapses them.

## Architecture

A Rust daemon owns identity, resolution, fetching, normalization, evidence storage,
querying, job state, policy, and publication. Evidence lands in immutable Arrow/Parquet
snapshots queried through DataFusion. A thin FastMCP 4 adapter exposes the tools; a separate
worker runs Griffe extraction and isolated runtime probes.

```
coding agent ──┬── Context7 MCP ─────────► concepts, documentation examples
               │
               ├── library-research skill ► routing, evidence policy, synthesis
               │
               └── FastMCP 4 stdio adapter
                            │ typed, bounded local RPC
                            ▼
                   library-enrichmentd ──► fetchers, Rust/Python API producers,
                            │              LSP sessions, sandboxed verification
                            ▼
                   immutable evidence snapshots (Arrow/Parquet + DataFusion)
```

Service code, tool environments, extraction workspaces, caches, and outputs live **outside
every working repository**. The repository under study is never a subprocess working
directory, an extraction destination, or an install target.

## Documentation

| Path | Contents |
|---|---|
| [`docs/blueprint/`](docs/blueprint/) | The governing specification — frozen, not edited |
| [`docs/architecture/`](docs/architecture/) | Compatibility matrix and design notes |
| [`docs/adr/`](docs/adr/) | Decision records, including every deviation from the blueprint |
| [`docs/operations/`](docs/operations/) | Install, configure, and run |
| [`contracts/`](contracts/) | Frozen response-envelope schema and fixtures |
| [`tests/ACCEPTANCE_PLAN.md`](tests/ACCEPTANCE_PLAN.md) | The 46 acceptance gates |
| [`skills/library-research/`](skills/library-research/) | The companion agent skill |
| [`AGENTS.md`](AGENTS.md) | Instructions for agents working in this repository |

## Development

```sh
just --list        # every recipe
just doctor        # verify the toolchain and required producers are present
just ci            # gates for completed phases, plus the next one
```

Requires a pinned stable Rust toolchain (`rust-toolchain.toml`), `uv`, and `just`.
Build and runtime verification profiles additionally require a working container runtime
or `bwrap`.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed
as above, without any additional terms or conditions.
