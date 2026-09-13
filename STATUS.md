# Status

**Phase 0 of 6 — complete.** All four clauses of the blueprint's §13 Phase-0 gate hold, and the
three gates scoped to this phase pass. The wire types, the daemon and the MCP adapter exist and
run; a minimal MCP/daemon handshake works end to end. No evidence *retrieval* exists yet — that
is Phase 1. Last updated 2026-09-13.

`3 passed / 0 failed / 0 blocked / 45 not_run` of 48 gates: **C14, C19 and R04**, each carrying
a recorded caveat in the report. Regenerate with `just acceptance-report`.

### The blueprint's Phase-0 gate, clause by clause

> Exact dependency locks; tool input/output schema tests; `service_status` truthfully reports
> absent components; an unsupported producer format returns a typed error.

| Clause | Where |
|---|---|
| Exact dependency locks | `Cargo.lock`, `uv.lock`; `just deps-policy` |
| Tool input/output schema tests | **C19** — one corpus, four boundaries, verdicts compared per document |
| `service_status` truthfully reports absent components | **C14**, plus the daemon-absent path |
| An unsupported producer format returns a typed error | **R04** — `producer::rustdoc::probe_format` |

R04 was scoped to phase 1 and is now phase 0, because the blueprint puts its assertion in the
Phase-0 gate. The gate ID is unchanged; only the `phase` field, which is ours — the frozen
`tests/ACCEPTANCE_PLAN.md` has no phase column.

Both earlier gates were audited by `acceptance-auditor` and the diff by `boundary-reviewer`.
The first audit **downgraded C19**; the review found a **C20 breach**. Both were fixed and a
re-audit confirmed both gates. Do not treat a green tally as a substitute for running them.

---

## Read this first if you are new to the repo

```sh
direnv allow      # once; then ty/ruff/pytest/binaries work with no prefix
just --list       # the command surface
just doctor       # what is actually installed
just ci           # everything that must hold (~1 min)
just gate-phase 0 # the phase gate specifically
```

The governing specification is `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md` (§1–16) and the
execution brief is `docs/blueprint/AGENT_HANDOFF.md`. Both are frozen — read, never edit.
`AGENTS.md` is the working contract; detail lives in `.claude/rules/`, scoped by path.

### What runs today

```sh
cargo build -p enrichment-daemon
library-enrichmentd start      # Unix socket, bounded NDJSON-RPC 2.0
library-enrichmentd status     # truthful component report as JSON
library-enrichmentd stop
library-enrichment-mcp         # nine MCP tools over stdio; normally launched by a client
```

The adapter starts **without** the daemon. `service_status` then returns `partial`, reports the
daemon as unavailable, and names the gap — a stopped daemon is exactly when a caller most needs
a usable answer, so it is reported rather than raised.

Eight of the nine tools return a typed `UNSUPPORTED_CAPABILITY` envelope naming the phase that
implements them. That is deliberate: an empty `ok` would assert the question had been answered.

---

## Next: Phase 1 — the end-to-end static Rust slice

The format adapter R04 needs already exists (`producer::rustdoc`), so Phase 1 starts from a
producer that can already refuse what it cannot read. What it lacks is a *fetcher*: docs.rs
metadata and hosted rustdoc JSON, registry identity, and the normalized public API those feed.


Registry identity, docs.rs metadata and hosted rustdoc JSON, one format adapter, a normalized
public API, immutable artifacts and snapshots, and five real tools: `resolve_library`,
`library_overview`, `search_evidence`, `inspect_symbol`, `read_artifact`.

**Gate:** a real crate resolves cold and then offline from the same snapshot; missing docs.rs
JSON yields `partial` with explicit gaps, never an empty success. Nine gates are registered for
phase 1 (R01–R08, C13).

The groundwork is in place: add a method to `server::dispatch`, a producer behind a
`ProducerSpec`, and the tool's real body in `python/enrichment_mcp/server.py` in place of its
`_not_implemented` call. The framing, the bound, the error mapping, and the envelope are settled.

---

## Verified 2026-09-13 — measured or executed, not read from memory

| Pin | Version | Established by |
|---|---|---|
| datafusion | 55.1.0 | Resolved **first**; the Arrow stack derived from it |
| arrow / arrow-schema / parquet | 59.3.0 | Selected by datafusion 55.1.0 |
| object_store | 0.13.2 | Selected by datafusion 55.1.0 |
| public-api | 0.52.2 | Parses captured rustdoc formats 57, 60 **and** 61 |
| **schemars** | **1.2.2** | Added for schema emission; 6 new crates, no new duplicate major, no new `deny.toml` skip |
| **tokio** | **1.53.1** | Promoted from transitive to direct; `net` adds only `mio` + `socket2` |
| **thiserror** | **2.0.20** | Promoted from transitive; already in the lock via `public-api` |
| fastmcp | 4.0.3 | Imports on 3.14.7 → `fastmcp.server.server.FastMCP` |
| griffe | 2.3.0 | Published signature byte-identical to the installed one |
| ty | 0.0.80 | `ty check --python/--venv` is the capsule hook |
| **datamodel-code-generator** | **0.80.0** | Generates the Pydantic boundary DTOs; resolves on 3.14 |
| **pytest-asyncio** | **1.4.0** | `asyncio_mode = "auto"` for the contract tier |
| **toml** | **1.1.6** | Reads `LIBENR_CONFIG`; `parse` + `serde` only, never `preserve_order` |
| **uuid** | **1.26.1** | Mints request identities in the core, where §1.1 puts identity |
| **rustdoc-types** | **0.59.0** | Dev-only: measures which rustdoc formats actually parse (R04) |
| rustc | 1.98.1 | **Current** stable (`rustup check`), pinned exactly |
| rustdoc producer | `nightly-2026-09-13` | Byte-identical to the rolling nightly |

`cargo deny` passes all four checks with `multiple-versions = "deny"` and 13 individually
reasoned skips, none an Arrow-stack crate. 94 Rust tests, 65 Python tests, 3 doctest targets.

---

## What the reviews caught

Running `acceptance-auditor` and `boundary-reviewer` was not ceremony — between them they found
one real defect that would have shipped, one overstated gate, and one wrong report state.

**The daemon could write a socket into a repository under study.** When `HOME` and every XDG
variable were unset, path resolution fell back to a *relative* `.library-enrichment/run`, which
resolves against the working directory. Reproduced against a canary repo: a live socket appeared
inside it. That is precisely the breach gate C20 exists to catch. Resolution now fails loudly —
`PathError::NoRuntimeDirectory`, or `NotAbsolute` for a relative value from any source — and
`paths::tests` covers all five sources. A daemon that will not start beats one that quietly
writes into someone's checkout.

**C19 was passing on an assertion it had not earned.** The gate says inputs are "rejected
**consistently** through CLI/RPC/MCP boundaries". Three boundaries each rejected *something*,
but no single document ever traversed more than one, so nothing measured agreement — and the
CLI leg had no document-accepting surface at all. Fixed by making consistency the mechanism:
one `enrichment_daemon::validate::validate`, reached through `library-enrichmentd validate`
(CLI) and `wire.validate` (RPC); one corpus in `tests/wire_corpus.py`; and a test that pushes
every document through every boundary and compares verdicts. It found a real disagreement on
its first run — see the next trap.

**A skipped test reported the gate as `failed`.** `acceptance-report.py` mapped every non-`passed`
pytest outcome to a failure, so an unsupported platform or an unbuilt binary produced a claimed
regression. `AGENTS.md` is explicit that a missing prerequisite is `blocked` with the
prerequisite named. Now it is, and `just test-python` builds the daemon binary first so the
e2e tier does not silently skip.

Also fixed from the review: `state-leak-check` and `conftest` now digest
`$XDG_RUNTIME_DIR/library-enrichment` (the one directory Phase 0 actually writes to, previously
watched by neither); the session fixture neutralises `LIBENR_SOCKET`; an unmatched
`service_status` component filter returns `partial` with the gap named rather than an empty `ok`
indistinguishable from "no such producer exists"; and `sandbox.enabled_profiles` now carries
`enabled_profiles_source` saying it is a built-in default, because reporting a hardcoded value
as the operator's configuration is a wrong answer about policy.

Every item the reviews left open has since been closed:

- **Configuration is read.** `enrichment_core::config` loads `LIBENR_CONFIG`; the daemon
  enforces the configured `rpc_message_bytes` and reports the configured `enabled_profiles` with
  an `enabled_profiles_source` saying where they came from. An unparsable file refuses to start
  rather than running with limits the operator did not write.
- **The daemon emits complete envelopes.** `request_id`, `coverage` and `freshness` are built in
  the core, where §1.1 puts the evidence model. The adapter forwards them. It keeps one builder
  of its own, for the daemon-unreachable case, because the core is by definition not around to
  state that fact.
- **The duplicated socket resolver is pinned.** `library-enrichmentd socket-path` exposes the
  Rust answer, and `tests/contract/test_socket_resolution.py` compares both implementations
  across all five branches *and* their precedence. The previous "divergence detector" only ever
  exercised one branch.
- **The enforcement layer has an oracle.** `just guardrails-check` digests `AGENTS.md`,
  `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`, `scripts/hooks/` and `scripts/env.sh`
  against `config/guardrails.sha256`. A guard cannot check itself, so this is the other half:
  detection, not prevention. It is not tamper-proof — anyone who can edit the layer can
  re-record — but it makes a change impossible to make *quietly*, and re-recording is gated
  behind `LIBENR_ALLOW_GUARDRAIL_RERECORD=1`.

---

## Traps found the hard way

These cost real time. Do not re-derive them.

**DataFusion turns on `serde_json/preserve_order`, and Cargo unifies features across whatever
is being built.** So `serde_json::Map` is a `BTreeMap` under `cargo run -p enrichment-core`
(what `just schemas-generate` uses) and an insertion-ordered `IndexMap` under
`cargo nextest run --workspace`. The emitted schema's key order therefore depended on *which
cargo command produced it*, which would have made `schema-conformance.sh`'s
`git diff --exit-code` churn forever. `wire::schema::canonicalize` sorts every object key
explicitly rather than relying on the map type;
`wire_conformance::every_object_key_is_emitted_in_sorted_order` and
`emitted_schema_matches_the_committed_file` are the guards — the second only catches it because
it runs under workspace unification while the file is written under `-p`.

**schemars drops `Option<T>` fields from `required` under its default deserialize contract.**
That would silently lose four of the fourteen root fields the frozen contract demands. Generate
with `SchemaSettings::draft2020_12().for_serialize()`. Do **not** reach for
`#[schemars(required)]` instead: it strips the `null` from the type union and would reject every
fixture's `"job": null`.

**One `///` on a unit enum variant turns the emitted `enum` into `oneOf` + `const`.** schemars
only emits `{"type":"string","enum":[…]}` while every variant carries no doc, title, description
or examples. The documents validated are the same, but `schemas/frozen/enums.json` no longer
matches and a structural conformance check fails with no obvious cause. Document the values in
the enum's own doc comment — container docs are safe. Tests 13–17 in `wire_conformance.rs` are
the oracle.

**`mcp.types.ToolAnnotations` leaves every hint unset, and the MCP spec reads an absent
`destructiveHint`/`openWorldHint` as `true`.** So silence is the *permissive* answer and the
risk runs opposite to intuition. Set them explicitly. Use the snake_case field names
(`read_only_hint`), not the camelCase aliases: both populate the model and both serialize to
camelCase, but `ty` cannot see through the alias generator and flags the aliases as discarded.

**`cargo deny` counts a bare `{ path = ... }` as a wildcard dependency.** `wildcards = "deny"`
is set, so an internal crate needs an explicit `version` alongside its path.

**The generated Pydantic DTO does not enforce the contract's root conditionals.**
`datamodel-codegen` renders properties, types and `additionalProperties: false`, but not the
`allOf` if/then blocks — so the DTO alone accepts a `pending` envelope with a null job, which
every other boundary rejects. `enrichment_mcp.envelope.validate_document` therefore validates
against the generated *schema*, and
`test_the_generated_dto_alone_is_not_a_sufficient_validator` pins the limitation so it cannot
quietly become an assumption. If a future release does emit those validators, that test fails
and the indirection can go.

**A test that returns early passes; a test that skips is `blocked`.** The reporting pipeline
can see a skip and cannot see an early `return`, so a conditional guard inside a test body is a
silent false pass. R04's central measurement had one — it read captures from `$LIBENR_CACHE_HOME`
and returned when unset, reporting PASS having examined nothing. Real captures now live in
`tests/fixtures/rustdoc/` and absent fixtures are a failure. If a test genuinely cannot run,
`#[ignore]` or `pytest.mark.skipif` is the honest mechanism, because both reach the report.

**`git diff --exit-code` over `schemas/generated/` only sees tracked files.** Until that
directory is committed the reproducibility half of `schema-conformance.sh` is a no-op. The Rust
test `emitted_schema_matches_the_committed_file` covers the same invariant without depending on
git state.

**The `pre_bash` hook scans command *text*, so a heredoc containing `//!` or `~/.claude/skills`
trips the outside-repo and user-config guards.** Both are false positives on file *content*.
Use `Write`/`Edit` for file content rather than a heredoc; that is the right tool anyway, and it
still goes through `pre_edit`.

### Still true from before

`ty`'s `textDocument/implementation` works and the docs say it does not (ADR 0005); a
`typing.Protocol` returns only itself, with no error signal. No format negotiation on docs.rs —
exactly one `/json/{n}` returns 200 per build. A target-qualified 404 means "no JSON for this
target", never "crate missing". `griffe.load(allow_inspection=…)` defaults to `True`. The PyPI
Simple API returns HTML with a 200 if you omit the Accept header. Native FastMCP tasks need the
separate `fastmcp-tasks` package *and* a client on protocol `2026-07-28`. `observed_configuration`
needs nine docs.rs keys, not five. `cargo nextest` does not run doctests.

---

## What Phase 0 built

| | |
|---|---|
| `crates/enrichment-core/src/wire/` | The authoritative envelope types. The status/job/error rule is enforced three times: `Outcome` makes an invalid combination unrepresentable, `TryFrom<RawEnvelope>` re-establishes it from JSON, and `status_conditionals` restates it as the schema's root `allOf`. |
| `crates/enrichment-core/src/bin/emit-schemas.rs` | Writes `schemas/generated/`. Both schema scripts probe `cargo metadata` for this exact binary name. |
| `crates/enrichment-daemon/` | `library-enrichmentd`, bounded NDJSON-RPC 2.0 over a Unix socket. One real method, `service.status`. |
| `python/enrichment_mcp/` | The FastMCP 4 adapter: nine tools, real input schemas, lazy per-call daemon connection. |
| `python/enrichment_mcp/_generated/` | Pydantic boundary DTOs, generated from the emitted schemas. Never hand-edited; ruff is scoped away from it (ADR 0007), `ty` is not. |
| `docs/adr/0006`, `0007` | The RPC details §2.1 left open, and the ruff scoping decision. |

**The generation chain runs end to end**: Rust wire types → JSON Schema → Pydantic DTOs, with
`just schemas-generate`, and `just schema-conformance` proves the result accepts and rejects
exactly what the frozen contract does.

---

## How the guardrails actually work

Four tiers, chosen by asking what the cheapest reproducible oracle is.

| Tier | Where | Covers |
|---|---|---|
| PreToolUse hook | `scripts/hooks/` | Visible in the tool call alone. **70 tested cases.** |
| ast-grep rule | `rules/` + `rule-tests/` | Code shapes. 5 rules, all with fixtures. |
| `just` gate | `justfile` | Needs the whole repo |
| Prose | `AGENTS.md`, `.claude/rules/` | Only what has no mechanical oracle |

`pre_edit.sh` now exempts `$HOME/.claude/plans/*` and `/tmp/claude-*/*` — the harness's own plan
and scratch paths, which are neither service state nor user configuration. The exemption is
narrow and its narrowness is tested: `~/.claude/skills` and `~/.claude/settings.json` are still
denied, because those are exactly what `pre_bash.sh` gates behind `LIBENR_ALLOW_USER_INSTALL=1`.

**Truthful reporting is mechanical.** `tests/gates.toml` registers all 48 IDs.
`acceptance-report` defaults every gate not matched by an executed test to `not_run`, and
downgrades a partially-executed set back to `not_run`. `acceptance-check` refuses any gate marked
`passed` without a recorded command and log. `just test-rust` and `just test-python` now write
`docs/reports/logs/{nextest,pytest}.json` — without those logs no gate can ever be promoted,
which is why every gate read `not_run` before this phase regardless of what passed.

**What you cannot edit:** `AGENTS.md`, `CLAUDE.md`, `.claude/rules/`, `.claude/settings.json`,
`scripts/hooks/`, `scripts/env.sh`, plus frozen provenance and contracts.

---

## Where things are

| | |
|---|---|
| `docs/blueprint/` | The governing spec — frozen |
| `docs/provenance/` | Delivered digests + PATHMAP — frozen |
| `docs/architecture/compatibility-matrix.md` | Every pin, with evidence and retrieval dates |
| `docs/adr/` | 8 ADRs; `/adr <slug>` to add |
| `docs/operations/` | Install, register, run — the daemon section is live |
| `contracts/`, `schemas/frozen/` | The Phase-0 acceptance target |
| `schemas/generated/` | Emitted from the Rust types; never hand-edited |
| `skills/library-research/` | **The product skill.** Installed only by explicit `just install-skill --apply` |

Subagents: `upstream-verifier`, `acceptance-auditor`, `boundary-reviewer`.
Commands: `/phase-gate`, `/adr`, `/verify-upstream`, `/acceptance-report`, `/handoff`.

Development service state is the gitignored `.dev-state/`; production resolves to XDG.
`just state-leak-check` proves the real paths stayed untouched. `just state-reset` clears it.

---

## Conventions

`just`, never `make`. Mutating recipes are never dependencies of validation recipes. No
suppressions — no `# noqa`, no `# type: ignore`, no `#[allow(...)]`; there are none in the tree.
Ruff line length 100. Rust edition 2024. Generated artifacts are never hand-edited.

Date your verification claims. Known failures stay visible rather than being quietly skipped.
