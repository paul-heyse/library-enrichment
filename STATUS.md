# Status

**Phase 0 of 6 — repository and governance complete; Phase 0 gate partially satisfied.**
Last updated 2026-09-13.

## Gates

`0 passed / 0 failed / 0 blocked / 46 not_run` of 46.

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

## Open

- **`textDocument/implementation` in ty 0.0.80.** Gate P08 asserts `UNSUPPORTED_CAPABILITY`.
  If ty has added support, P08's assertion changes — that is an ADR, not a test edit.
  Unverified; run `/verify-upstream ty implementation support`.
- **Arrow / DataFusion pinned pair.** Nothing pinned yet. Pin DataFusion **first** and derive
  `arrow`, `parquet`, `object_store` from it; `deny.toml` sets `multiple-versions = "deny"`.
- **`public-api` release parsing format_version 61.** Use the library, not the CLI (§4.2).
- docs.rs JSON endpoint shape, PyPI/Simple API shapes, client registration: all unverified.

## Next

1. `/verify-upstream` the remaining matrix rows, DataFusion/Arrow first.
2. Phase 0 gate proper: the minimal FastMCP 4 tool catalog and daemon handshake, with
   `service_status` truthfully reporting absent producers and an unsupported producer format
   returning a typed error. Registers gates C14 and C19.
3. Then Phase 1, the end-to-end static Rust slice.

Do not leave the actual MCP path until the end — the handoff is explicit about this.

## Conventions

`just --list` is the command surface. `direnv allow` once, after which `ty`, `ruff`, `pytest`
and the built binaries work with no prefix. Service state during development lives in the
gitignored `.dev-state/`; `just state-leak-check` proves the real XDG paths stayed untouched.
