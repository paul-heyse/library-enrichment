# Status

**Phase 0 of 6.** Repository, governance and compatibility matrix complete. Dependencies
verified and locked. No service code yet. Last updated 2026-09-13.

`0 passed / 0 failed / 0 blocked / 48 not_run` of 48 gates — the honest state for an
unimplemented service. Regenerate with `just acceptance-report`.

---

## Read this first if you are new to the repo

```sh
direnv allow      # once; then ty/ruff/pytest/binaries work with no prefix
just --list       # the command surface
just doctor       # what is actually installed
just ci           # everything that must hold (~1 min)
```

The governing specification is `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` (§1–16) and the
execution brief is `docs/blueprint/AGENT_HANDOFF.md`. Both are frozen — read, never edit.
`AGENTS.md` is the working contract; detail lives in `.claude/rules/`, scoped by path so it
loads when you touch matching files.

### The hooks are live now

They were written mid-session and only take effect from a fresh start, so **this is the first
session where they actually fire.** Expect to be stopped by:

| Denied | Because |
|---|---|
| `pyrefly` / `pyright` / `mypy` | `ty` is the engine (§1.1, §5.4). Both others are installed here and globally plugin-enabled — that is why this is guarded four ways. |
| `uvx x@latest`, unpinned `uvx`/`npx` | §12.1 — resolve at setup, not per invocation |
| bare `cargo +nightly` | This box's rustup default *is* nightly. Use the dated pin from `config/toolchains.toml`. |
| writes outside the repo | §2.3, gate C20 |
| `claude mcp add`, `codex mcp add`, `~/.claude/skills` | Mutates user config. `LIBENR_ALLOW_USER_INSTALL=1` to override, deliberately. |
| `cargo add`/`uv add` of a banned class | §1.1 — no graph db, embeddings, vector db, gRPC, Redis, FastAPI |
| writing the enforcement layer | See below |

A denial names the blueprint section it comes from. **Read it rather than working around it.**
If a guardrail is genuinely wrong, say so and stop — propose the fix in `docs/adr/`.

**What you cannot edit:** `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`,
`scripts/hooks/`, `scripts/env.sh`, plus frozen provenance and contracts. Shell redirects are
blocked too, so `cat > AGENTS.md` will not work either.

**What you can edit freely:** the `justfile`, gate scripts in `scripts/`, the ast-grep corpus in
`rules/` and `rule-tests/`, `.claude/agents/`, `.claude/commands/`, `.claude/skills/`, and
`tests/gates.toml`. Adding a rule to `rules/` is frictionless on purpose — that is where review
findings are meant to land.

---

## Next: finish the Phase 0 gate

Everything below this line is groundwork. The blueprint's Phase 0 gate is not met until a
minimal MCP path actually runs. `AGENT_HANDOFF.md` is explicit: *"do not leave the actual MCP
path until the end."*

1. **Wire types in `enrichment-core`.** The response envelope from
   `contracts/research-envelope.schema.json`: 14 required root fields, 4 statuses, 13 error
   codes, 6 evidence classes, 7 job states, `^library-evidence://` artifact URIs. Add an
   `emit-schemas` binary writing `schemas/generated/`. `just schema-conformance` already checks
   the corpus against the frozen contract and will start checking the generated side the moment
   that binary exists.
2. **Minimal daemon.** `library-enrichmentd start|status|stop` over a Unix socket with bounded
   NDJSON-RPC. Not MCP framing, not LSP framing — a third thing (§2.1).
3. **Minimal FastMCP 4 adapter.** Registers the tool catalog **without** starting an LSP,
   downloading a package, or compiling (gate **C14**). `service_status` truthfully reports
   absent producers. An unsupported producer format returns a typed error.
4. **Register the tests** in `tests/gates.toml` under C14 and C19, then `just acceptance-report`.
   A gate becomes `passed` only when a test actually runs — nothing else can promote it.

Then Phase 1: the end-to-end static Rust slice.

---

## Verified 2026-09-13 — measured or executed, not read from memory

`docs/architecture/compatibility-matrix.md` has **no unverified rows**: 40 verified by
execution, 27 read from primary sources, 1 verified absence, 2 contradictions.

| Pin | Version | Established by |
|---|---|---|
| datafusion | 55.1.0 | Resolved **first**; the Arrow stack derived from it |
| arrow / arrow-schema / parquet | 59.3.0 | Selected by datafusion 55.1.0 |
| object_store | 0.13.2 | Selected by datafusion 55.1.0 |
| public-api | 0.52.2 | Parses captured rustdoc formats 57, 60 **and** 61 |
| fastmcp | 4.0.3 | Imports on 3.14.7 → `fastmcp.server.server.FastMCP` |
| griffe | 2.3.0 | Published signature byte-identical to the installed one |
| ty | 0.0.80 | `ty check --python/--venv` is the capsule hook |
| rustc | 1.98.1 | **Current** stable (`rustup check`), pinned exactly so it cannot float |
| rustdoc producer | `nightly-2026-09-13` | Byte-identical to the rolling nightly: current *and* reproducible |

`cargo deny` passes all four checks with `multiple-versions = "deny"` and 13 individually
reasoned skips, none an Arrow-stack crate.

Other facts about this workstation worth not rediscovering: rustup's default toolchain is
**nightly**, which is why `rust-toolchain.toml` is load-bearing. podman, docker and bwrap are
all present, so `build` and `runtime` execution profiles are implementable here and C20 coverage
under them is never legitimately `not_run`. Context7 is connected as
`mcp__plugin_context7_context7__{resolve-library-id,query-docs}` — confirming §1.2's warning
that clients namespace these and the service must discover tool definitions, not hardcode a
prefix.

---

## Traps found the hard way

These cost real time to discover. Do not re-derive them.

**ty's `textDocument/implementation` works, and the docs say it does not.** ADR 0005. Gate P08's
premise was false; it is retired in place and replaced by P08a/P08b. The sharper half: a
`typing.Protocol` returns **only itself** — structural implementors are missing with **no error
signal**. A plausible, non-empty, silently incomplete answer is more dangerous than an
unsupported one. Record it as an evidence gap (§6.2). And do not derive any further capability
claim from ty's published table [S18]; it was wrong in the permissive direction. Probe
`initialize` at runtime.

**No format negotiation on docs.rs.** Exactly one `/json/{n}` returns 200 per build. Our nightly
emits 61; hosted builds go back to at least 53 (serde 1.0.219). The parser adapter is
load-bearing, and `public-api`'s supported range — not the nightly's — is the binding
constraint.

**docs.rs's own target-specific example URL 404s.** A target-qualified 404 means "no JSON for
this target", never "crate missing". Read the real target back from `content-disposition`, not
from what you requested.

**`griffe.load(allow_inspection=...)` defaults to `True`.** Griffe imports the package when
static sources are unavailable unless you pass `False`. `force_inspection` exists too and must
stay `False`. Gate P02 depends on this; `rules/griffe-static-only.yml` enforces it.

**The PyPI Simple API returns HTML with a 200 if you omit the Accept header.** Send the
q-weighted list and dispatch on the returned `Content-Type`. Prefer it over the JSON API for
file listings: `releases` is deprecated, field names differ (`digests`/`blake2b_256` vs
`hashes`; `requires_python` vs `requires-python`), and PyPI JSON metadata is frozen at first
upload — a claim about a release, not evidence about an artifact.

**`destructiveHint` and `openWorldHint` default to `true`** in MCP annotations. So the risk is
the reverse of the obvious one: `verify_usage` must not set `readOnlyHint=True`, and cached
reads should set `openWorldHint` explicitly rather than trusting a default.

**Native tasks need the separate `fastmcp-tasks` package *and* a client on protocol
`2026-07-28`.** `import fastmcp.tasks` fails in the base install. The default adapter must not
depend on it (§7.4); expensive work goes through core jobs and `job_control`.

**`observed_configuration` needs nine docs.rs keys, not five** — add `default-target`,
`additional-targets`, `rustc-args`, `cargo-args`. The last two change what is *compiled*.

**`cargo nextest` does not run doctests.** `just test-rust` runs `cargo test --doc` too. Keep it
that way, or §6.3's round-trip examples are silently untested.

---

## How the guardrails actually work

Four tiers, chosen by asking what the cheapest reproducible oracle is. A hook that needs to
parse a source file is the wrong tier — write an ast-grep rule instead.

| Tier | Where | Covers |
|---|---|---|
| PreToolUse hook | `scripts/hooks/` | Visible in the tool call alone. **64 tested cases.** |
| ast-grep rule | `rules/` + `rule-tests/` | Code shapes. **5 rules, all with fixtures.** Run against the single edited file on every write. |
| `just` gate | `justfile` | Needs the whole repo |
| Prose | `AGENTS.md`, `.claude/rules/` | Only what has no mechanical oracle |

**Truthful reporting is mechanical, not a promise.** `tests/gates.toml` registers all 48 IDs.
`acceptance-report` defaults every gate not matched by an executed test to `not_run`.
`acceptance-check` refuses any gate marked `passed` without a recorded command and log, rejects
an invented ID, and rejects a successor naming a gate that is not actually retired. The Stop
hook re-checks the first of those. All tested.

**Frozen contracts enforce themselves.** `contracts/`, `config/service.example.toml` and
`tests/ACCEPTANCE_PLAN.md` sit at their manifested paths, so `just provenance-check` (15/15)
fails if one is edited. The only legitimate route is an ADR re-freezing under a new dated
bundle.

**Retiring a gate:** never renumber, delete or reuse an ID. Mark `superseded` in
`tests/gates.toml` with an ADR reference and register successors with `supersedes` + `adr`. P08
→ P08a/P08b is the worked example.

---

## Where things are

| | |
|---|---|
| `docs/blueprint/` | The governing spec — frozen |
| `docs/provenance/` | Delivered digests + PATHMAP — frozen |
| `docs/architecture/compatibility-matrix.md` | Every pin, with evidence and retrieval dates |
| `docs/adr/` | 5 ADRs; `/adr <slug>` to add |
| `docs/operations/` | Install, register, run |
| `contracts/`, `schemas/frozen/` | The Phase-0 acceptance target |
| `crates/`, `python/` | Implementation (skeletons) |
| `skills/library-research/` | **The product skill**, not a dev skill. Installed only by explicit `just install-skill --apply`. |
| `.claude/skills/` | Development skills — empty; write one when a workflow has actually repeated |

Subagents: `upstream-verifier`, `acceptance-auditor`, `boundary-reviewer`.
Commands: `/phase-gate`, `/adr`, `/verify-upstream`, `/acceptance-report`, `/handoff`.

Development service state is the gitignored `.dev-state/`; production resolves to XDG.
`just state-leak-check` proves the real paths stayed untouched. `just state-reset` clears it.

---

## Conventions

`just`, never `make`. Mutating recipes are never dependencies of validation recipes. No
suppressions — no `# noqa`, no `# type: ignore`, no `#[allow(...)]`. Ruff line length 100. Rust
edition 2024. Generated artifacts are never hand-edited.

Date your verification claims. "Verified 2026-09-13: rustup default is nightly" stays useful a
month later; "verified" does not. Known failures stay visible rather than being quietly skipped.
