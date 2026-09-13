# library-enrichment agent instructions

A local-first library-evidence service for Rust and Python, exposed over MCP. The governing
specification is [`docs/blueprint/IMPLEMENTATION_BLUEPRINT.md`](docs/blueprint/IMPLEMENTATION_BLUEPRINT.md)
and the execution brief is [`docs/blueprint/AGENT_HANDOFF.md`](docs/blueprint/AGENT_HANDOFF.md).
Both are frozen provenance: read them, never edit them. Current state is [`STATUS.md`](STATUS.md).

Build the smallest complete, reproducible evidence service that lets a strong coding agent
discover useful capabilities, retrieve precise support, verify consequential assumptions, and
explain deployment choices. Optimize for correct end-to-end operation and bounded evidence,
not for a universal knowledge platform.

## Binding decisions

These are settled. Changing one requires an ADR with primary-source evidence and tests.

| Decision | Requirement |
|---|---|
| Core ownership | Rust owns identities, resolution, fetching, normalization, evidence storage, querying, job state, policy, and publication. |
| Python boundary | A thin FastMCP 4 adapter and a separate extraction worker. Never reimplement the core in Python. |
| Python semantics | Astral **ty**. Not pyrefly, not pyright, not mypy. |
| Rust semantics | rust-analyzer over LSP, behind a producer adapter. |
| Static extraction | Griffe, with `allow_inspection=False` and explicit `search_paths`. |
| Rust API source | Hosted rustdoc JSON from docs.rs **before** any local compilation. |
| Storage | Immutable artifacts and Arrow/Parquet snapshots; DataFusion for retrieval. |
| Context7 | A separate MCP connection used by the calling agent. No proxy, no embedded LLM. |
| Transport | Per-client stdio adapters over one local Rust daemon. |

Do **not** add: a graph database, embeddings, a vector database, a GPU workload, a generic
workflow engine, a raw SQL or arbitrary-shell MCP tool, a whole-repository code-property
graph, or automatic project edits.

## The boundary rule

Service state, environments, capsules, caches, and outputs live outside every working
repository. **A repository under study is never a subprocess working directory, an extraction
destination, or an install target.** Gate C20 proves it with a filesystem digest over a canary
repo, run under every enabled execution profile.

Production state resolves to XDG paths. Agent sessions are redirected to a gitignored
`.dev-state/` by `scripts/hooks/session_env.sh`, so development never mutates real user state.
`just state-leak-check` fails if the real XDG paths change during a test run.

`skills/library-research/` is the **product** — the skill this service ships. It is not a
development skill, and it is never installed into user configuration without an explicit,
operator-invoked setup command. `.claude/skills/` is the separate (currently empty) directory
for development skills.

## Toolchain and environment identities

**This workstation's rustup default is `nightly`.** `rust-toolchain.toml` pins stable 1.98.1
and is load-bearing; without it every bare `cargo` call compiles against nightly and gate R09
is untestable. Verify with `just toolchain-check`.

The rustdoc-JSON fallback producer does not build our code on nightly. It shells
`cargo +<dated-nightly> rustdoc` against a third-party crate inside a capsule. That nightly is
a recorded **producer identity** in `config/toolchains.toml` — toolchain, rustc release, commit
hash, emitted `format_version` — and it feeds `ProducerRun` provenance. There is no second
`rust-toolchain.toml`. Never invoke a bare `cargo +nightly`; always use the dated pin.

Python is `uv` only, always `uv run <tool>`, dev dependencies in `[dependency-groups]`. `ty` is
pinned in the lockfile, never installed as a global `uv tool`.

`pyrefly` is installed on this workstation and globally plugin-enabled in Claude Code, and
`pyright-lsp` is too. **Neither is this project's engine.** If `ty` is unavailable, the affected
gates are `blocked` and it warrants an ADR — it is never grounds for substituting another
type checker.

## Command surface

Start with `just --list`. Do not run the full suite before every edit.

| Change | Useful checks |
|---|---|
| Rust logic | `cargo clippy` on the affected crate, then its tests |
| Wire types / DTOs | `just schemas-generate` then `just schema-conformance` |
| Python adapter or worker | `uv run ruff check`, `uv run ty check`, `uv run pytest tests/contract` |
| Producer or policy | the affected gate, then `just state-leak-check` |
| Docs, skill, rules | spelling, links, and `ast-grep test` |
| Anything frozen | `just provenance-check` |

## Phases and gates

The blueprint sequences six phases in vertical slices (§13). Every phase must leave a runnable
and tested system; a directory scaffold is not an implemented service.

`tests/gates.toml` is the machine-readable registry of all 46 acceptance IDs (R01–R10 Rust,
P01–P10 Python, C01–C20 cross-cutting, A01–A06 client acceptance). `just acceptance-report`
generates `docs/reports/acceptance.json` by joining real test output against that registry;
every gate not matched by an executed test defaults to `not_run`. Use `/phase-gate <n>`.

Gates are additive and phase-scoped. A gate whose target does not exist yet reports `not_run`
with a reason — never a false pass, and never a hard failure that tempts you to weaken it.

## Truthful reporting

Four states, and only four: `passed`, `failed`, `blocked`, `not_run`.

A gate is `passed` only when a command actually ran and its log is recorded. A missing tool,
absent credentials, or an unsupported platform is `blocked` with the prerequisite named. Never
attempted is `not_run`. **A mocked client is never a pass.** Do not claim success from mocks,
canned evidence, or static screenshots, and do not convert gate results into a quality
percentage.

The same discipline applies to the evidence model itself: `ok` means successful within the
declared scope, not complete knowledge. An empty search result is not proof that a capability
is absent. Keep the six epistemic classes distinct and never silently overwrite a runtime
observation with a stub annotation.

## Deviations

The blueprint specifies architecture and behavior; it does not guarantee that every
dependency's latest release composes correctly. When reality differs, record it with `/adr`:
the binding boundary it touches, primary-source evidence with URL and retrieval date, the
tests that prove it, and consequences. Preserve the binding boundaries. Do not silently
reinterpret the specification.

Verify current upstream APIs against primary sources before pinning. `docs/blueprint/SOURCES.md`
catalogs 26 of them. Context7 is a discovery lead, never exact-version proof.

## Navigation

Detailed constraints live in [`.claude/rules/`](.claude/rules/), scoped by path so they load
where they apply: `rust-core-boundary`, `python-boundary`, `execution-policy`,
`evidence-truthfulness`, `frozen-contracts`, `generated-artifacts`, `process-immutable`.

Enforcement has four tiers, chosen by asking what the cheapest reproducible oracle is:
a **hook** when the mistake is visible in the tool call alone; an **ast-grep rule** in `rules/`
with fixtures in `rule-tests/` when it is a code shape; a **`just` gate** when it needs the
whole repo; and **prose** only when there is no mechanical oracle. A hook that needs to parse a
source file is the wrong tier — write a rule instead.

The enforcement layer itself is not editable from inside a session: `AGENTS.md`, `CLAUDE.md`,
`.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and `scripts/env.sh` are denied by
both hooks, shell redirects included. Everything else is yours — the justfile, gate scripts,
the ast-grep corpus, subagents, commands, and `tests/gates.toml`. Adding a rule to `rules/` is
frictionless on purpose, because that is where review findings are supposed to land.

## Conventions

`just`, never `make`. Recipe names are kebab-case and outcome-shaped, annotated with `[doc]`
and `[group]`. Mutating recipes are never dependencies of a validation recipe.

No suppressions: no `# noqa`, no `# type: ignore`, no `#[allow(...)]`, no growing an ignore
list. Fix the cause or record an ADR.

Ruff `line-length = 100`. Rust edition 2024, resolver 3. Generated artifacts are never
hand-edited — fix the Rust wire type and regenerate.

Date your verification claims. "Verified 2026-09-13: rustup default is nightly" is useful a
month later; "verified" is not. Known failures stay visible rather than being quietly skipped.
