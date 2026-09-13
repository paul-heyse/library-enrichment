# Compatibility matrix

The Phase 0 gate. `AGENT_HANDOFF.md`: *"Verify current upstream APIs and pin compatible tool
versions. The blueprint specifies architecture and behavior, not an assurance that every
dependency's latest release composes correctly."*

Every row needs a primary source, a retrieval date, and an exact quote. **A row with an empty
source is not a pin — it is an open question.** Fill rows with `/verify-upstream`, which runs the
`upstream-verifier` subagent against the 26 catalogued sources in `docs/blueprint/SOURCES.md`.

Verdicts: `verified` (read this session from the primary source) · `unverified` (not yet
established; says what was tried) · `contradicted` (upstream differs from a blueprint
assumption — **stop and open an ADR**).

---

## Measured on this workstation

These were produced by running the tools, not by reading about them.

### rustdoc JSON format versions

`just rustdoc-format-matrix`, measured 2026-09-13. Each installed dated nightly was used to
build a trivial crate with `-Z unstable-options --output-format json`, and the emitted
`format_version` recorded.

| Toolchain | rustc | Commit | `format_version` | Verdict |
|---|---|---|---:|---|
| `nightly-2026-02-28` | 1.95.0-nightly | `3a70d0349` | 57 | verified (measured) |
| `nightly-2026-05-23` | 1.98.0-nightly | `54333ff07` | 57 | verified (measured) |
| `nightly-2026-07-15` | 1.99.0-nightly | `da80ed070` | 60 | verified (measured) |
| `nightly-2026-08-18` | 1.100.0-nightly | `8fa1c96cf` | 61 | verified (measured) |
| **`nightly-2026-09-13`** | **1.100.0-nightly** | **`809936eac`** | **61** | **verified (measured) — selected producer** |

Consequences:

- The rustdoc producer identity is recorded in `config/toolchains.toml` as
  `nightly-2026-09-13` / format 61 — the current nightly, written in dated form because
  `nightly-2026-09-13` is byte-identical to this workstation's rolling `nightly` channel
  (both `809936eac`). That makes the pin simultaneously newest-available and reproducible.
- **The parser adapter must inspect `format_version` at read time.** Four installed toolchains
  span three format versions, and docs.rs may serve a version built by any rustdoc release
  (blueprint §4.1 [S07]).
- Acceptance gate R04 ("unsupported rustdoc JSON format → typed error, no silent misparse") has
  genuine supported *and* unsupported captures from this spread, with no synthesised artifact.
  Captures are under `$LIBENR_CACHE_HOME/rustdoc-format-matrix/`.

### Local toolchain

| Tool | Version | Note |
|---|---|---|
| rustc (pinned) | 1.98.1 (`48a229cea` 2026-09-01) | **Current stable.** `rustup check` on 2026-09-13: "stable — up to date: 1.98.1". Written as an exact version, not `stable`, so the pin does not float. Verified active by `just toolchain-check`. |
| rustup default | **nightly** 1.100.0 (`809936eac` 2026-09-12) | Why the stable pin is load-bearing — a bare `cargo` without it targets nightly |
| rustdoc producer | `nightly-2026-09-13` (`809936eac`) | Current nightly, dated for reproducibility |
| rust-analyzer | 1.98.1 (`48a229c`) | Rust semantic evidence over LSP |
| uv | 0.12.13 | The only supported Python package manager |
| just | 1.58.0 | Task surface |
| Python (system) | 3.12.3 | Not the target; see ADR 0004 |
| podman / docker / bwrap | all present | `build` and `runtime` profiles are implementable here |
| `ty` | **absent** | Hard requirement. Python semantic gates are `blocked` until pinned and installed. |
| `cargo-public-api` (CLI) | absent | Intentional — blueprint §4.2 prefers the `public-api` library consuming rustdoc JSON directly |

---

## To verify

Nothing below is pinned yet. Do not add a dependency version to `Cargo.toml` or
`pyproject.toml` until its row is `verified`.

### Python boundary

Rows marked *verified (executed)* were established by resolving and importing the package on
Python 3.14.7 in this repository's own environment on 2026-09-13, not by reading about it.

| Claim | Source | Retrieved | Evidence | Verdict | Pin |
|---|---|---|---|---|---|
| FastMCP 4 current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `fastmcp 4.0.3` (pulls `mcp 2.2.0`, `pydantic 2.13.5`) | **verified (executed)** | `fastmcp==4.0.3` |
| `from fastmcp import FastMCP` is the supported import | executed | 2026-09-13 | resolves to `<class 'fastmcp.server.server.FastMCP'>` | **verified (executed)** | — |
| `ToolResult` is available for output control | executed | 2026-09-13 | `fastmcp.tools.base.ToolResult` | **verified (executed)** | — |
| Native tasks need a separate package | executed | 2026-09-13 | `import fastmcp.tasks` → `ModuleNotFoundError` in the base install | **verified (executed)** | — |
| Griffe current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `griffe 2.3.0` | **verified (executed)** | `griffe==2.3.0` |
| **`griffe.load(allow_inspection=...)` defaults to `True`** | executed | 2026-09-13 | `inspect.signature(griffe.load)` → `allow_inspection` default `True`; `search_paths` default `None`; `force_inspection` default `False` | **verified (executed)** | — |
| ty current release resolving on 3.14 | executed | 2026-09-13 | `uv lock` → `ty 0.0.80`; `.venv/bin/ty --version` → `ty 0.0.80` | **verified (executed)** | `ty==0.0.80` |
| **ty still lists `textDocument/implementation` as unsupported** | [S18] | | | unverified | — |
| `ToolResult` semantics: summary text vs structured data | [S02] | | | unverified | — |
| Native tasks also require client protocol support | [S03] | | | unverified | — |

Two of these change how the code must be written:

- **`allow_inspection` defaults to `True`.** Griffe will import a package when static sources
  are unavailable unless `False` is passed explicitly, which would execute library code inside
  the extraction worker and fail gate P02. `rules/griffe-static-only.yml` enforces the explicit
  argument in the edit loop. Note `force_inspection` also exists and must stay `False`.
- **`fastmcp.tasks` is absent from the base install**, confirming [S03]. The default adapter
  must not import it; expensive work goes through ordinary core jobs and `job_control`
  (blueprint §7.4).

The `textDocument/implementation` row remains load-bearing and unverified: gate P08 asserts an
`UNSUPPORTED_CAPABILITY` result. If ty has since added support, **P08's assertion changes** and
that is an ADR, not a quiet test edit. `ty 0.0.80` is a pre-1.0 version; expect churn.

The `textDocument/implementation` row is load-bearing: acceptance gate P08 asserts an
`UNSUPPORTED_CAPABILITY` result. If ty has since added support, **P08's assertion changes** and
that is an ADR, not a quiet test edit.

The three "supports Python 3.14" rows together decide ADR 0004. If any is `contradicted`, stop.

### Rust core

| Claim | Source | Retrieved | Quote | Verdict | Pin |
|---|---|---|---|---|---|
| DataFusion current release | — | | | unverified | |
| `arrow` major required by that DataFusion | — | | | unverified | |
| `parquet` compatible with that `arrow` | — | | | unverified | |
| `object_store` compatible with that `arrow` | — | | | unverified | |
| `public-api` release parsing format_version 61 | [S10] | | | unverified | |
| docs.rs JSON endpoint shape and compression | [S07] | | | unverified | — |
| docs.rs build metadata fields | [S08] | | | unverified | — |
| rustdoc unstable output flags on the pinned nightly | [S26] | | | unverified | — |

**Pin DataFusion first and derive `arrow`, `parquet` and `object_store` from it.** The reverse
order fails: `deny.toml` sets `multiple-versions = "deny"` because two `arrow` majors in one
graph means two incompatible `RecordBatch` types (blueprint §8.4).

### Registries and clients

| Claim | Source | Retrieved | Quote | Verdict |
|---|---|---|---|---|
| PyPI JSON API response shape | [S14] | | | unverified |
| Simple Repository API: yanked status, compatibility | [S15] | | | unverified |
| intersphinx `objects.inv` format | [S17] | | | unverified |
| Context7 current tool names and arguments | [S21] | | | partially verified |
| Codex stdio MCP registration | [S22] | | | unverified |
| Claude Code stdio MCP registration, user scope | [S24] | | | unverified |

Context7 is *partially* verified by direct observation on 2026-09-13: it is connected to Claude
Code as a plugin MCP exposing `resolve-library-id` and `query-docs`, surfaced under the
namespaced names `mcp__plugin_context7_context7__resolve-library-id` and
`…__query-docs`. This confirms blueprint §1.2's warning that clients namespace these tools and
the service must discover actual tool definitions rather than hardcoding a prefix. The upstream
argument shapes are still unverified.
