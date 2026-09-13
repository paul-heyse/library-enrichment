# Status

**Phase 0 of 6 — repository and governance complete; compatibility matrix complete; pins
verified and locked.** Last updated 2026-09-13.

## Gates

`0 passed / 0 failed / 0 blocked / 48 not_run` of 48.

48, not 46: P08 was retired in place and replaced by P08a and P08b (ADR 0005). The frozen
`tests/ACCEPTANCE_PLAN.md` still records the original 46 and still verifies.

All 46 acceptance IDs are registered in `tests/gates.toml` with none yet claimed. That is the
correct state: no tool is implemented, so no gate can honestly pass. `just acceptance-check`
enforces that a gate cannot become `passed` without a recorded command and log.

Regenerate with `just acceptance-report`; audit with `/acceptance-report`.

## What exists

- Repository, git, dual MIT/Apache-2.0 license, buildable Cargo workspace and Python package.
- Governance: `AGENTS.md`, `CLAUDE.md`, seven path-scoped rules in `.claude/rules/`, six hooks,
  five ast-grep rules with fixtures, three subagents, five slash commands.
- Enforcement verified: **40/40 hook cases**, **5/5 ast-grep rules**, `provenance-check` 15/15.
- `docs/architecture/compatibility-matrix.md` with the Phase 0 rows established so far.

## Verified 2026-09-13

Measured or executed on this workstation, not read from memory:

| Fact | Consequence |
|---|---|
| rustup's **default toolchain is nightly** (1.100.0, `809936eac`) | `rust-toolchain.toml` pinning stable 1.98.1 is load-bearing; `just toolchain-check` guards it |
| stable **is** 1.98.1 (`rustup check`: "up to date") | The pin is current, not stale. Written as an exact version so it does not float. |
| rustdoc `format_version` spans **57 → 61** across five installed nightlies | The parser adapter must inspect `format_version` at read time. Gate R04 has genuine supported *and* unsupported captures. |
| `fastmcp 4.0.3`, `griffe 2.3.0`, `ty 0.0.80` all resolve and import on CPython **3.14.7** | ADR 0004 accepted; all three pinned exactly with `uv.lock` committed |
| `from fastmcp import FastMCP` → `fastmcp.server.server.FastMCP` | The blueprint's required import form is correct |
| **`griffe.load(allow_inspection=...)` defaults to `True`** | Griffe imports packages unless `False` is passed explicitly. `rules/griffe-static-only.yml` enforces it; gate P02 depends on it. |
| `import fastmcp.tasks` fails in the base install | Confirms [S03]: the default adapter must not depend on native tasks |
| podman, docker and bwrap all present | `build` and `runtime` execution profiles are implementable here, so C20 coverage under those profiles is never legitimately `not_run` |
| `pyrefly` installed and globally plugin-enabled; `pyright-lsp` too | Neither is this project's engine. Guarded at four layers: settings deny, pre-bash hook, ast-grep rule, `just doctor`. |

## Pinned and verified

`docs/architecture/compatibility-matrix.md` has **no unverified rows left**: 40 verified by
execution, 27 read from primary sources, 1 verified absence, 2 contradictions.

| | Pin | How |
|---|---|---|
| datafusion | 55.1.0 | Resolved first; the Arrow stack is derived from it |
| arrow / arrow-schema / parquet | 59.3.0 | Selected by datafusion 55.1.0 |
| object_store | 0.13.2 | Selected by datafusion 55.1.0 |
| public-api | 0.52.2 | Parses captured rustdoc formats 57, 60 **and** 61 |
| fastmcp | 4.0.3 | Imports on 3.14.7; `fastmcp.server.server.FastMCP` |
| griffe | 2.3.0 | Published signature byte-identical to the installed one |
| ty | 0.0.80 | `ty check --python/--venv` is the capsule hook |

`cargo deny check` passes all four (advisories, bans, licenses, sources) with
`multiple-versions = "deny"` and 13 individually reasoned skips, none of them an Arrow-stack
crate. `cargo check --workspace` succeeds with the full set.

## Two contradictions found

**1. ty implements `textDocument/implementation`** — ADR 0005, gate P08 superseded by P08a/P08b.
Shipped in ty 0.0.64 (2026-07-27); the published capability table still says "Not supported" and
links an issue closed 2026-07-26. Probed twice independently. The dangerous part is not the
table being wrong, it is that **Protocol conformance returns a plausible, non-empty, silently
incomplete answer**: `class P(Protocol)` returns only itself, with no error signal. That must be
recorded as an evidence gap, which is what P08b asserts.

**2. docs.rs's own target-specific example URL 404s.** The documented
`…/latest/i686-pc-windows-msvc/json` is dead; `…/4.6.6/x86_64-unknown-linux-gnu/json` works.
Target-qualified JSON exists only for targets actually built with JSON output. Implementation
consequence: a target-qualified 404 means "no JSON for this target", never "crate missing", and
the real target must be read back from `content-disposition`, not assumed from the request.
Covered by existing gates R03 and R06; no new gate needed.

## Open

- **No format negotiation on docs.rs.** Exactly one `/json/{n}` returns 200 per build. Our
  pinned nightly emits 61, but hosted builds go back to at least 53 (serde 1.0.219). The parser
  adapter is load-bearing, and `public-api`'s supported range — not the nightly's — is the
  binding constraint.
- **The Simple API returns HTML with a 200 if you forget the Accept header.** Send the
  q-weighted list and dispatch on the returned `Content-Type`.
- **Prefer the Simple API over the JSON API for file listings.** `releases` is deprecated, field
  names differ (`digests`/`blake2b_256` vs `hashes`; `requires_python` vs `requires-python`), and
  PyPI JSON metadata is frozen at first upload — a claim about a release, not evidence about an
  artifact.
- **`observed_configuration` needs nine docs.rs keys, not five** — add `default-target`,
  `additional-targets`, `rustc-args`, `cargo-args`. The last two change what is *compiled*.
- **Do not derive any further capability assertion from ty's published table** ([S18]); it was
  wrong in the permissive direction. Probe `initialize` at runtime and record the result.
- **`destructiveHint` and `openWorldHint` default to `true`** in MCP annotations. `verify_usage`
  must not set `readOnlyHint=True`; cached reads should set `openWorldHint` explicitly rather
  than relying on a default.
- Native tasks need the separate `fastmcp-tasks` package **and** a client on protocol
  `2026-07-28`. The default adapter must not import it.

## Next

1. Phase 0 gate proper: the minimal FastMCP 4 tool catalog and daemon handshake, with
   `service_status` truthfully reporting absent producers and an unsupported producer format
   returning a typed error. Registers gates C14 and C19.
2. Then Phase 1, the end-to-end static Rust slice.

Do not leave the actual MCP path until the end — the handoff is explicit about this.

## Conventions

`just --list` is the command surface. `direnv allow` once, after which `ty`, `ruff`, `pytest`
and the built binaries work with no prefix. Service state during development lives in the
gitignored `.dev-state/`; `just state-leak-check` proves the real XDG paths stayed untouched.
