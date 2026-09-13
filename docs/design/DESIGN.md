# Library Enrichment Service — Design

**Status:** Authoritative. This document is the design.  
**Origin:** `docs/blueprint/IMPLEMENTATION_BLUEPRINT.md`, revision 1.0 of 2026-09-13, frozen.  
**Amended by:** decision records under [`../adr/`](../adr/README.md). See [`README.md`](README.md).

This is a **spine, not a copy**. Each section states what the design is *now*, in one to three
sentences, and cites `blueprint §N.M` for the full text rather than restating it. Where a
decision record governs a section, an inline `> Decision: ADR-NNNN` marker names it.

Section numbers mirror the blueprint's and are **stable citation targets**. `just adr-lint`
resolves every `§` citation in every ADR against a heading here, so a renumbering is a red
build. Insert `§6.2.1`; never renumber.

Every claim carries a charter §D evidence label — `Proposed`, `Interface-checked`,
`Implemented`, `Tested`, `Measured`, `Formally established`. `Proposed` is not a failure state;
an unlabelled claim is. Sections describing phases that have not been built are `Proposed` by
construction and are marked once at the section head rather than per sentence.

**A label is a claim about the tree, not a design decision**, so correcting one as work lands is
maintenance and needs no ADR — only a change to what the design *is* does. This is the part of
the spine that rots: the 2026-09-13 design review's §8 argues these labels would be better
generated from `service_status` and `acceptance.json` than restated here, and register row R-14
carries that question forward.

---

## 1. Mission and deliverable

A standalone, local-first library-evidence service for Rust and Python, exposed over MCP. It
collects exact-release package information, public API structure, documentation navigation,
source and examples, semantic observations, API and release changes, and narrowly scoped
verification results. **The service produces evidence; the calling agent produces architectural
judgments.** blueprint §1.

The operational question is not "what methods exist" but: given a design objective and a
concrete dependency environment, what relevant capabilities exist, what evidence supports their
use, what configuration do they require, and what remains unverified?

A future capability graph may consume the evidence. Implementing that graph is neither a
prerequisite nor in scope.

### 1.1 Binding decisions

Thirteen binding decisions, each a stable citation target. §B1–§B11 are blueprint §1.1;
§B12 and §B13 are blueprint §5.2 and §4.1, which `AGENTS.md` also treats as binding. The
mapping from each to the charter's DM principles and G1–G7 gates is in
[`../design_review/design_principles/ADDENDUM.md`](../design_review/design_principles/ADDENDUM.md) §1.

Changing one requires an ADR with primary-source evidence and tests. A deviation must preserve
the others and say which ones it preserves.

#### B1 Repository boundary

One standalone repository. No service code, environments, caches, or generated indexes inside
working repositories, and **a repository under study is never a subprocess working directory, an
extraction destination, or an install target.** Gate C20 proves it with a filesystem digest over
a canary repo under every enabled execution profile. blueprint §1.1, §2.3.

> Decision: ADR-0003 — development sessions redirect service state to a gitignored `.dev-state/`.  
> Decision: ADR-0008 — two harness path shapes are exempt from the outside-repository write guard.

*Evidence: Tested — gate C19; `just state-leak-check`; `scripts/test-hooks.sh`.*

#### B2 Core ownership

Rust owns identities, package resolution, fetching, normalization, evidence storage, querying,
job state, policy, and publication. blueprint §1.1, §2.1.

*Evidence: Implemented — resolution, fetching, normalization, snapshot storage, querying
and publication all live in Rust (`ops/`, `producer/`, `enrichment-store/`). Job state and the
verification policy paths are Proposed.*

#### B3 Python boundary

A thin FastMCP 4 adapter, a separate Griffe extraction worker, and isolated runtime probes.
**Never a reimplementation of the core in Python.** The adapter validates inputs, calls the
daemon, emits MCP results and maps structured errors; it does not index libraries or own
persistent state. blueprint §1.1, §2.1.

*Evidence: Implemented for the adapter — it forwards the core's envelope and composes one
only for facts the core cannot state. Proposed for the Griffe worker.*

#### B4 Python semantic engine

Astral **ty**, through its CLI and LSP boundary, not its private Rust internals. Not pyrefly,
not pyright, not mypy. If `ty` is unavailable the affected gates are `blocked` and it warrants
an ADR — it is never grounds for substituting another type checker. blueprint §1.1, §5.4.

> Decision: ADR-0005 — ty does implement `textDocument/implementation`; gate P08 is superseded by P08a/P08b, and [S18] must not be used to derive any capability assertion.

*Evidence: Tested — ADR-0005 records the runtime probe against ty 0.0.80.*

#### B5 Rust semantic engine

rust-analyzer over LSP, isolated behind a producer adapter. Neither language server is embedded
as a Rust library in this repository. blueprint §1.1, §9.2.

*Evidence: Proposed — Phase 4.*

#### B6 Context7

A separate MCP connection used directly by the calling agent. No Context7 proxy, no embedded
LLM, no recursive research agent inside this service. A Context7 result with an uncertain source
version is a **discovery lead, never exact-version proof**. blueprint §1.1, §3.3, §11.

*Evidence: Implemented — the service has no Context7 dependency; the skill carries the routing.*

#### B7 Storage

Immutable raw artifacts and Arrow/Parquet evidence snapshots; DataFusion for structured
retrieval. This is an **evidence cache, not a replacement for the future graph/fact store**.
Arrow and DataFusion are pinned to a verified mutually compatible set; one Rust type universe.
blueprint §1.1, §8.4.

*Evidence: Tested — `enrichment-store` stages, verifies by read-back, renames and repoints
(`snapshot::publish`); DataFusion reads the published tables back
(`symbols_round_trip_through_datafusion`). Pins in `docs/architecture/compatibility-matrix.md`.*

#### B8 MCP transport

Per-client stdio adapters attached to one local Rust daemon over a Unix-domain socket. HTTP is
an optional later deployment profile, not part of the initial topology. No FastAPI, gRPC, or
Redis is added to carry this small local interface. blueprint §1.1, §2.1, §12.4.

> Decision: ADR-0006 — NDJSON-RPC framing, socket path resolution order, and message size limits.

*Evidence: Tested — `crates/enrichment-daemon/tests/rpc_boundary.rs`;
`tests/e2e/test_mcp_daemon_handshake.py`.*

#### B9 Design inference

Architectural judgment belongs in the calling agent's evidence-backed research brief and is
**never silently promoted to extracted fact**. The epistemic class `agent_inferred` exists to
keep it separable. blueprint §1.1, §6.2, §11.2.

*Evidence: Proposed — the class exists in the model; no producer emits it yet.*

#### B10 Completion

Real Rust and Python fixtures must work end to end, **including tests of incomplete evidence and
failure paths**. Required paths cannot be replaced by TODOs, canned evidence, or mock production
responses. blueprint §1.1, §14.4.

*Evidence: Proposed — Phases 1–5.*

#### B11 Excluded mechanisms

Do not add: a graph database, embeddings, a vector database, a GPU workload, a generic workflow
engine, a raw-SQL or arbitrary-shell MCP tool, a whole-repository code-property graph, or
automatic project edits. blueprint §1.1.

*Evidence: Tested — `just deps-policy` fails on the banned dependency classes; `pre_bash.sh`
denies adding them.*

#### B12 Static extraction

Griffe with `allow_inspection=False` and explicit `search_paths`. Without the former, Griffe may
import packages when static sources are unavailable — extraction must never execute the library
under study. Unresolved aliases are preserved rather than guessed. blueprint §5.2 [S12].

*Evidence: Tested — `rules/griffe-static-only.yml` with fixtures in `rule-tests/`.*

#### B13 Rust API source

Hosted rustdoc JSON from docs.rs is obtained **before** any local compilation. `format_version`
is inspected and a compatible parser adapter selected; a local capsule build on a dated pinned
nightly is the fallback, recorded as `locally_built_rustdoc`. blueprint §4.1, §4.4 [S07, S26].

*Evidence: Implemented — `producer/docsrs.rs` acquires hosted JSON and `producer/rustdoc.rs`
probes `format_version` before parsing; `just rustdoc-format-matrix` records what each installed
nightly emits. The local capsule fallback is Proposed.*

### 1.2 Why these boundaries

Context7 is not the authority for a project's actual resolved dependencies, and its tool names
may be namespaced by the client — discover the actual tool definitions rather than hardcoding a
prefix. FastMCP 4 supplies typed tools, resources and current protocol negotiation; use the
standalone `fastmcp` package and `from fastmcp import FastMCP`, pinned exactly. blueprint §1.2
[S01, S02, S04, S21].

*Evidence: Tested — `rules/fastmcp-standalone-import-only.yml`; pins in `uv.lock`.*

## 2. System topology and ownership

Coding agent → Context7 MCP and the `library-research` skill → FastMCP 4 stdio adapter → typed
bounded local RPC → Rust enrichment daemon → producers, LSP session manager, isolated
verification workers → raw artifact cache and immutable evidence snapshots → Arrow/Parquet and
DataFusion → bounded MCP responses. blueprint §2.

### 2.1 Processes

Three: **`library-enrichmentd`**, the single writer and job owner, supporting multiple
simultaneous agents without duplicate builds or a language server per tool call;
**`library-enrichment-mcp`**, a small per-client Python adapter owning no persistent state; and
**`library-enrichment-worker`**, a Python worker for approved Griffe and runtime-inspection
tasks that emits schema-versioned JSON and **cannot publish snapshots**. blueprint §2.1.

> Decision: ADR-0006 — the daemon RPC contract.

*Evidence: Implemented for the daemon and adapter; Proposed for the worker.*

### 2.2 Lifecycle

Explicit `start`, `status`, `stop`. The adapter may auto-start the daemon under an OS lock; the
daemon survives adapter exits and retains jobs, and an adapter disconnect does not implicitly
cancel a shared job. **MCP initialization registers the tool catalog and nothing else** — no
language servers, downloads, compilation or registry refreshes. blueprint §2.2 [S05].

*Evidence: Implemented for catalog registration (gate C14); Proposed for job retention.*

### 2.3 Filesystem layout

Implementation in the repository; regenerable data under the cache home
(`downloads/`, `unpacked/`, `capsules/`, `lsp/`); retained evidence and job records under the
data home (`blobs/sha256/`, `snapshots/`, `contexts/`, `jobs/`, `bundles/`, `logs/`). Resolved
by an OS-aware directory resolver with overrides, never by assuming Linux paths. blueprint §2.3.

> Decision: ADR-0003 — `.dev-state/` for development sessions, defined once in `scripts/env.sh`.  
> Decision: ADR-0008 — the harness-path exemption, scoped to `plans/`, deliberately not all of `~/.claude/`.

*Evidence: Tested — `just state-leak-check`; the `pre_edit.sh` service-state deny class.*

## 3. Research identity, versioning, and freshness

### 3.1 Four identities

`release_id` (the actual release or source), `environment_id` (declared or resolved
environment), `context_id` (release + environment + research mode, carrying whether the
environment is `unspecified`, `declared`, `resolved` or `verified`), and `snapshot_id` (an
immutable evidence set). **Enrichment creates a new manifest; it never overwrites a snapshot**,
and resolving previously unknown environment fields yields a *new derived context* rather than
mutating an old ID. rustdoc's local item IDs are never cross-release global IDs. blueprint §3.1.

*Evidence: Implemented — `identity/mod.rs`; content-derived and covered by unit tests.*

### 3.2 Research modes

`project` (the exact supplied environment, never silently upgraded), `upstream` (the newest
eligible published release as of a recorded registry check), `compare` (two exact contexts, with
environment differences shown as such rather than as library changes), and `revision` (a pinned
development revision, kept distinct from a release and from a mutable branch name).
blueprint §3.2.

*Evidence: Implemented for `project` and `upstream`; Proposed for `compare` and `revision`.*

### 3.3 Freshness is not one timestamp

At least `retrieved_at`, `registry_checked_at`, `source_revision`, `artifact_digest`,
`producer_version` and `documentation_version_match` are recorded, with version matching
distinguished as `exact`, `compatible_claimed`, `mismatched` or `unknown`. Freshness options are
`cache_ok`, `revalidate` and `offline`; a request for "latest" either revalidates or returns a
clearly marked inability to verify latest. blueprint §3.3 [S07].

*Evidence: Implemented — the fields are recorded and `source_version_match` is carried on every
envelope. The `revalidate`/`offline` option set is Proposed.*

## 4. Evidence acquisition: Rust

### 4.1 Preferred path

Exact release and target/features → registry metadata → docs.rs build metadata and hosted
rustdoc JSON → normalize public API and re-exports → overview, exact-symbol search, public-API
diff → targeted docs, examples, release notes → rust-analyzer or a compile probe only as needed.
See §B13. blueprint §4.1 [S07].

*Evidence: Implemented — `ops/resolve.rs` runs this path end to end for a real crate.*

### 4.2 Producers and outputs

Six: registry/manifest reader; `cargo metadata` in a service capsule (matching the project
target and features, never defaulting to `--all-features`); hosted or local rustdoc JSON; the
`public-api` library consuming rustdoc JSON directly (the CLI is a diagnostic adapter, not the
canonical model); a targeted documentation and source reader; and rust-analyzer over LSP with
its analysis scope recorded. blueprint §4.2 [S09, S10].

*Evidence: Implemented for the registry reader, hosted rustdoc JSON, the `public-api`
projection and the source reader; Proposed for `cargo metadata` in a capsule and for
rust-analyzer.*

### 4.3 Documentation build is not project configuration

docs.rs lets maintainers set features, targets and rustdoc arguments, so the hosted result is
evidence **for that documented configuration**, not for the project. `observed_configuration` is
recorded separately from `requested_configuration`; a symbol can be `documented_available` and
`project_availability_unverified`. Never blindly compile all feature combinations.
blueprint §4.3 [S08].

*Evidence: Implemented — `observed_configuration` is recorded separately and carries the nine
docs.rs keys; availability reports `project_availability_unverified`.*

### 4.4 Local fallback build

When hosted JSON is absent, unsupported or wrong for the requested configuration, build in a
service-owned capsule on a **dated pinned nightly recorded as a producer identity** in
`config/toolchains.toml` — toolchain, rustc release, commit hash, emitted `format_version` —
feeding `ProducerRun` provenance. A producer derives its command from typed options and never
concatenates an agent-supplied shell command. Nightly-build success is not proof the API
compiles on the project's stable compiler. blueprint §4.4 [S11, S26].

*Evidence: Interface-checked — `just rustdoc-format-matrix`; Proposed for the producer.*

### 4.5 Discovery beyond API additions

Capability discovery inspects crate and module overviews, feature descriptions, examples,
migration notes and release notes: behavior can change without new public symbols. Where a
version-to-commit relation cannot be verified it is marked approximate rather than treating the
default branch as released behavior. blueprint §4.5.

*Evidence: Implemented for changelog and release-note fragments; Proposed for migration notes
and the comparison surface.*

## 5. Evidence acquisition: Python

### 5.1 Preferred path

Exact distribution and interpreter/platform/extras → PyPI metadata and a suitable artifact →
package/import mapping, metadata, inline types and stubs → Griffe static API snapshot →
`objects.inv` and targeted official documentation → ty in a synthetic consumer capsule →
isolated runtime inspection only when needed. blueprint §5.1 [S14, S15].

*Evidence: Proposed — Phase 2.*

### 5.2 Producers and outputs

Six: registry/distribution reader; wheel and source inspector; **Griffe worker (§B12)**;
documentation inventory reader; `ty` process; and a restricted runtime worker whose observations
are separately labeled as executed evidence. Griffe's breaking-change checker is supplemental
interpretation, not a complete feature-addition detector — set and field differences are
computed over our own normalized snapshots. blueprint §5.2 [S12, S13].

*Evidence: Tested for the `allow_inspection=False` rule; Proposed for the producers.*

### 5.3 Types and dynamic APIs

Stubs, inline annotations and runtime objects can disagree: **store separate observations and
report conflicts**, never reconcile them silently. Python publicness is not one Boolean —
`__all__`, documented status, underscore convention, re-export pattern and author declaration
are separate signals. `objects.inv` is a navigation signal, not API extraction. A missing
docstring does not prove an API is private or absent. blueprint §5.3 [S16, S17, S20].

*Evidence: Proposed — Phase 2.*

### 5.4 ty integration

The pinned `ty server` executable over negotiated LSP, and a pinned `ty check` against a
generated consumer capsule with its own selected interpreter — **never against this service's own
virtual environment**. Capabilities are probed at runtime and unsupported outcomes preserved
rather than invented. blueprint §5.4 [S18, S19].

> Decision: ADR-0005 — `textDocument/implementation` is supported; [S18]'s table is unreliable and must not ground a capability assertion. Gates P08a and P08b replace P08.

*Evidence: Tested — ADR-0005's runtime probe; Proposed for the session manager.*

### 5.5 Supporting parsers and installed metadata

`importlib.metadata` is used only against a service-owned environment whose identity is
retained, and never by importing the target library. Ruff lints the service's own Python; its
internal crates, tree-sitter and a custom source-fact extractor are **not** prerequisites. A
targeted syntax adapter is an optional escalation that emits the same evidence records rather
than starting a parallel fact store. blueprint §5.5.

*Evidence: Implemented for ruff's own scope; Proposed for the rest.*

## 6. Canonical evidence model

A small Rust-owned typed model, not a large ontology. Stable enums and typed relationships cover
observed evidence; free text is for excerpts and explanations, never for essential machine
state. blueprint §6.

### 6.1 Required record types

Twelve: `Release`, `Environment`, `Context`, `Artifact`, `ProducerRun`, `Symbol`,
`Relationship`, `EvidenceFragment`, `Change`, `Verification`, `SnapshotManifest`, `Job`.
blueprint §6.1 names their core fields.

*Evidence: Implemented — `Release`, `Environment`, `Context`, `Artifact`, `ProducerRun`,
`Symbol`, `Relationship`, `EvidenceFragment` and `SnapshotManifest` exist in
`enrichment-core/src/evidence/`. `Change`, `Verification` and `Job` are Proposed.*

### 6.2 Separate epistemic classes

Six, and they stay distinct: `declared`, `statically_extracted`, `compiler_derived`,
`typechecker_observed`, `runtime_observed`, `agent_inferred`. **These are evidence categories,
not confidence scores, and they are never collapsed into one.** A passing probe establishes only
its tested assertions in its recorded environment. blueprint §6.2.

They are one of three distinct grading vocabularies this repository carries; the other two are
acceptance-gate states and charter §D evidence labels. Conflating them is a defect. See
[`ADDENDUM.md`](../design_review/design_principles/ADDENDUM.md) §3.

*Evidence: Implemented — the enum exists and every `EvidenceFragment` a producer emits carries
one. No producer emits `agent_inferred`, which is correct: that class is the calling agent's.*

### 6.3 Schema ownership

**Rust DTOs and the canonical table model are authoritative.** Versioned JSON Schemas are
generated from the Rust wire types, and the small Pydantic boundary DTOs from those schemas.
Arrow projections are declared in the Rust schema registry with tested semantic round trips.
Upstream raw JSON is preserved in content-addressed blobs. Schema and normalizer versions go
into content keys; **wall-clock timestamps and temporary paths are excluded from semantic
content hashes** and retained in provenance instead. blueprint §6.3.

> Decision: ADR-0007 — ruff does not lint the generated boundary DTOs; `ty` still does.

*Evidence: Tested — `just schemas-generate` and `just schema-conformance`; the generated-artifact
deny class in `pre_edit.sh`.*

### 6.4 Claim-oriented source priority

There is no universal best source. Exact signature → the selected artifact, compiler or stubs;
supported usage → version-matched author documentation and examples; runtime behavior → source
plus targeted execution; architectural fit → the calling agent's reasoning with linked premises.
**Contradictions are retained as conflicting observations**; a stub annotation never silently
overwrites a runtime observation. blueprint §6.4.

*Evidence: Proposed — Phase 2.*

## 7. MCP tool contracts

Nine tools with one ecosystem-neutral vocabulary for Rust and Python: `resolve_library`,
`library_overview`, `search_evidence`, `inspect_symbol`, `compare_releases`, `verify_usage`,
`read_artifact`, `job_control`, `service_status`. blueprint §7 gives inputs and required results.

*Evidence: Implemented for `resolve_library`, `library_overview`, `search_evidence`,
`inspect_symbol`, `read_artifact` and `service_status` — six of nine, all wired to real daemon
methods. `compare_releases`, `verify_usage` and `job_control` return a typed
`UNSUPPORTED_CAPABILITY` naming the phase that implements them.*

### 7.1 Tool behavior requirements

Resolution returns candidates and an actionable ambiguity rather than guessing, and never reads
a lookup as an upgrade request. Overview represents unindexed sections as **unknown, not empty**.
Search is deterministic lexical and token matching with recorded scoring factors — no embeddings
in v1 — and reports what it covered. Inspection defaults to public API plus a small documentation
section and does not start an LSP for a signature already normalized. Comparison computes
additive as well as breaking changes, and a clean API diff does not mean no behavioral change.
Verification's caller-supplied profile can only *select* from locally enabled profiles, never
grant one, and reports `not_run` rather than a fabricated success. Artifact reading resolves only
service-issued handles — **never a general filesystem path reader**. blueprint §7.1.

*Evidence: Implemented for the six live tools. An empty search carries the limitation that
absence is not evidence of absence (`ops/search.rs`); a missing evidence kind downgrades the
envelope to `partial`.*

### 7.2 Common response envelope

One stable root schema for every tool, carrying `schema_version`, `request_id`, `status`,
`summary`, `context_id`, `snapshot_id`, `data`, `coverage`, `freshness`, `evidence`, `artifacts`,
`pagination`, `job` and `error`. `status` is `ok`, `partial`, `pending` or `error`, and **`ok`
means successful within the declared scope, not complete knowledge**: a valid empty search is
distinct from an extraction failure and from a missing index. Thirteen stable error codes, each
carrying retryability and a concrete next action. blueprint §7.2.

`contracts/research-envelope.schema.json` is the frozen Phase-0 acceptance target; generated
schemas must match it *behaviourally*, not the other way round.

*Evidence: Tested — `just schema-conformance` against the frozen contract and its four fixtures.*

### 7.3 Output budgets

Configurable initial limits, explicitly **not measured performance claims**: a 12 KiB inline
response budget, 12 search results per page, an 800-character excerpt, an 80-line source
excerpt, a 2-second inline wait then a job receipt, two concurrent expensive workers, two warm
LSP sessions. Machine JSON is never truncated mid-object — fewer complete entries are returned
with a cursor or an artifact handle, and mandatory coverage and error fields are preserved.
Cursors bind the query digest, snapshot, sort and position; a mismatched cursor is rejected.
blueprint §7.3.

*Evidence: Implemented — budgets, pagination cursors bound to the query digest and snapshot,
and excerpt limits are enforced on the search and inspect paths. The limits are configured
defaults, **not measured performance claims**.*

### 7.4 FastMCP implementation

Typed Pydantic request and result models generated from or checked against the wire schemas; a
small human-readable summary plus compact structured data, never the same long document twice.
Four resource templates delegate to the same core read operations as the tools, taking IDs
rather than paths. The default adapter does not depend on MCP background-task support: expensive
operations return an ordinary `pending` envelope and `job_control` works with any client that
supports ordinary tools. Annotations are assigned deliberately — `verify_usage` is not portrayed
as read-only. **All logs go to stderr or daemon logs, never MCP stdout.** blueprint §7.4
[S02, S03, S04].

*Evidence: Tested — `rules/mcp-stdout-protocol-only.yml`; `tests/contract/test_mcp_catalog.py`.*

## 8. Orchestration, caching, and long-running work

### 8.1 Producer plans, not bespoke scripts

Typed `ProducerSpec` and `ProducerPlan` records over a small explicit dependency graph for the
known producers — **not a generic orchestration platform** (§B11). A tool requests missing
evidence kinds and the daemon plans the minimum additional producers; work already done for
matching content and configuration is reused; when one source fails, usable evidence from others
is retained with a `partial` status. blueprint §8.1.

*Evidence: Implemented — typed producer specs drive the Rust plan; a failed source retains the
other producers' evidence with a `partial` status.*

### 8.2 Single-flight and publication

Producer work is deduplicated by `(producer_version, normalized_options, input_digests,
environment_id, policy_profile)`, and cancelling one caller's interest must not kill work another
still needs. **Only the daemon publishes.** Workers write staged outputs under job-specific
paths; schema, references, digest integrity and expected coverage are validated before an
immutable snapshot directory is published and the context pointer atomically updated. Readers
never observe partially written tables. blueprint §8.2.

*Evidence: Tested for publication — stage, read-back verify, single rename, then the pointer
swap; staging is swept and never read as evidence. Single-flight deduplication is Proposed.*

### 8.3 Job state

Seven persisted states: `queued`, `running`, `succeeded`, `partial`, `failed`,
`cancel_requested`, `cancelled`. A pending result carries a job ID, a progress stage and an
advisory next-poll delay — **not a guarantee of completion time**. After a restart, unfinished
jobs are recovered only when their producer is idempotent and policy still permits; otherwise
they are marked interrupted with an actionable error. blueprint §8.3.

*Evidence: Proposed — Phase 4.*

### 8.4 Cache invalidation

Immutable artifacts key on digest; normalized snapshots additionally on schema and producer
versions; search pages on snapshot, query and budget; mutable registry and docs lookups use
validators and finite TTLs. Negative entries get shorter TTLs and preserve *why* something was
unavailable — **a timeout is not a permanent absence**. blueprint §8.4. Pinning is §B7.

*Evidence: Implemented — artifacts key on digest and snapshots additionally on schema and
normalizer versions. The execution-policy profile is **not** a declared input to the snapshot
key; see the 2026-09-13 review, finding F5, and register row R-13.*

## 9. LSP and verification implementation details

### 9.1 Synthetic consumer capsules

A capsule is a service-owned workspace holding exact dependency resolution inputs, a selected
toolchain or interpreter, a small consumer file and analysis settings; its manifest records what
was reproduced from the working project **and what was not**. A library API scan and a
project-compatible consumer check are different operations, and an API scan is never labeled a
successful integration test. blueprint §9.1.

*Evidence: Proposed — Phase 4.*

### 9.2 LSP session manager

The daemon owns warm sessions keyed by server version and capsule digest, implementing framing,
initialization, capability negotiation, document lifecycle, diagnostics, cancellation and
shutdown. The caller supplies a symbol ID or a snippet and never manufactures line/column
coordinates; negotiated position encodings are respected rather than assuming byte offsets match
UTF-16. **An empty result, an unsupported method, an unresolved dependency and incomplete
indexing are four distinct outcomes.** blueprint §9.2.

*Evidence: Proposed — Phase 4.*

### 9.3 Probe classes

Three: a Rust compile probe (`cargo check` or `cargo test --no-run` on the project compiler and
lock), a Python typecheck probe (ty against the recorded interpreter and stub environment), and
a runtime probe executing bounded code with explicit assertions. Type correctness is not runtime
correctness. Exceptions and missing native dependencies are preserved as **evidence gaps, never
as "the API does not exist"**. The daemon does not auto-generate invented successful examples;
an agent-supplied snippet is stored as `snippet_origin=agent`. blueprint §9.3.

*Evidence: Proposed — Phase 4.*

## 10. Execution policy and source handling

Three profiles: `static` (fetching, safe extraction, static parsing, search — enabled by
default), `build` (rustdoc fallback, proc-macro and build-script activity, Cargo and Python
environment setup — enabled only when a configured sandbox is operational), and `runtime`
(executing library code — explicitly enabled only). **Callers select from enabled profiles; they
never grant one.** Absent isolation, the service returns `POLICY_DENIED` rather than silently
running on the host.

Workers run with no home-directory or working-repository mount, a non-root user, bounded
CPU/memory/processes/time, a dedicated scratch directory, no network by default, and an explicit
environment allowlist — no inherited API tokens, SSH agents, cloud credentials, `PYTHONPATH`,
package-manager configuration, `.env` files or project hooks. Archive extraction rejects path
traversal, absolute paths, symlink escapes, decompression bombs and device files; HTTP fetching
enforces size, time and redirect limits and rejects private, loopback and link-local targets.

**Downloaded content is stored as untrusted evidence, never as agent instructions.** Network
restrictions, missing distributions, licensing constraints and sandbox failures produce clear
gaps — no silent unsupported fallback. **These policies are enforced in the Rust core**, not
delegated to tool annotations or to the skill. blueprint §10.

*Evidence: Proposed — Phases 4 and 6; the profile vocabulary is Implemented in
`.claude/rules/execution-policy.md` as a development constraint.*

## 11. Companion skill: Context7 + enrichment workflow

`skills/library-research/SKILL.md` is the **product** — the skill this service ships. It is
governed by this design, not by the development harness, and is never installed into user
configuration without an explicit operator-invoked setup command. `.claude/skills/` is the
separate directory for development skills. blueprint §11.

*Evidence: Implemented — the skill exists and is frozen provenance; Proposed for acceptance.*

### 11.1 Required research loop

Eight steps: frame the question; resolve the environment; discover broadly enough; retrieve
explanations narrowly through Context7; deepen selected candidates; **verify only material
uncertainty**; produce a compact decision brief separating factual support from architectural
inference; and stop. The agent is the orchestrator — the service does not call Context7 itself
to enforce this. blueprint §11.1.

*Evidence: Proposed — Phase 5.*

### 11.2 Capability discovery without a graph

Two passes: a small breadth pass over modules, features and documentation headings, then depth
on a shortlist, searching by desired outcome as well as by likely mechanism. A name-based query
misses unfamiliar capabilities; a full API dump overwhelms the agent; **a faceted overview
bridges that gap** — which is why §B11 excludes a graph and embeddings. Machine facts stay in
snapshots; judgments stay separately labeled. blueprint §11.2.

*Evidence: Proposed — Phase 5.*

## 12. Client installation and deployment

### 12.1 Build/install contract

A reproducible bootstrap that installs the Rust binaries, creates the service's own uv
environment, and installs exact-pinned FastMCP 4, Griffe and ty. `Cargo.lock`, `uv.lock` and the
toolchain configuration are committed. **Nothing is installed or updated into a working
repository's virtual environment**, and an unpinned `uvx ...@latest` is never the startup
command. The installed command is launchable by absolute path from any directory and finds its
own configuration and state. blueprint §12.1.

> Decision: ADR-0001 — the repository lives at `~/library-enrichment`, and no script hardcodes that path.  
> Decision: ADR-0002 — dual `MIT OR Apache-2.0`.  
> Decision: ADR-0004 — Python 3.14, with fastmcp 4.0.3, griffe 2.3.0 and ty 0.0.80 resolving and importing on 3.14.7.

*Evidence: Tested — `just setup`, `just doctor`, `just toolchain-check`.*

### 12.2 MCP registration

User-scoped stdio registration for both Codex and Claude Code, with Context7 registered
separately. **Registration alone does not prove the service connects** — actual tool calls in
both clients are the evidence. blueprint §12.2 [S22, S24].

*Evidence: Proposed — Phase 5.*

### 12.3 Skill installation

One source skill in this repository, installed at user scope rather than copied into every
project, only after checking for an existing installation and refusing to overwrite an unrelated
skill. An installed source/version manifest is stored, with explicit update and uninstall
commands. **No automatic edits to user configuration during ordinary test runs**; setup scripts
state their destinations and support a dry run. blueprint §12.3 [S23, S25].

*Evidence: Implemented — `scripts/install-skill.sh` requires `--apply` and defaults to a dry run.*

### 12.4 Optional later remote use

An HTTP adapter can reuse the same tools and core, with authentication and explicit
user/namespace boundaries before any exposure beyond loopback. A later deployment profile, not a
blocker. blueprint §12.4.

*Evidence: Proposed.*

## 13. Implementation sequence and acceptance gates

Seven phases in vertical slices: 0 compatibility and contracts; 1 end-to-end static Rust slice;
2 end-to-end static Python slice; 3 changes and bounded research; 4 semantic and verification
paths; 5 skill and client acceptance; 6 operational hardening. **Every phase must leave a
runnable and tested system** — a directory scaffold is not an implemented service.
blueprint §13 states each phase's gate clauses.

`tests/gates.toml` is the machine-readable registry of the acceptance IDs (R01–R10 Rust,
P01–P10 Python, C01–C20 cross-cutting, A01–A06 client acceptance, plus the replacements
ADR-0005 registered). Gate IDs are **stable identifiers, never renumbered**; a superseded gate is
retired in place. Gates are additive and phase-scoped: a gate whose target does not exist yet
reports `not_run` with a reason — never a false pass, and never a hard failure that tempts
anyone to weaken it.

Four states, and only four: `passed`, `failed`, `blocked`, `not_run`. A gate is `passed` only
when a command actually ran and its log is recorded. **A mocked client is never a pass.** Gate
results are never converted into a quality percentage.

> Decision: ADR-0005 — gate P08 retired in place; P08a and P08b registered.

*Evidence: Tested — `just acceptance-report` and `just acceptance-check`; `stop_guard.sh` fails a
session that claims a pass without a command and a log. Measured 2026-09-13: 211 Rust tests run,
208 passed, **3 failed** (`retrieval_fixture.rs` — a re-export edge, an availability
`requested_configuration` that is null where `[]` is expected, and a limitation-wording
mismatch); 67 Python tests passed. The three failures are open work in the Phase-1 slice and are
listed in `STATUS.md`, not skipped.*

## 14. Test strategy and definition of done

### 14.1 Test classes

Five: unit; contract (actual FastMCP schemas equal the declared wire schemas, tested through the
in-process client *and* a real stdio subprocess); fixture integration over small purpose-built
Rust and Python fixture libraries, with core CI never requiring a live internet response;
opt-in live tests recording exact versions at run time and kept distinct from deterministic
regressions; and end-to-end client tests, **explicitly marked unexecuted when clients are
unavailable rather than invented**. No mocking in the contract, integration or e2e tiers.
blueprint §14.1 [S06].

*Evidence: Tested — `rules/no-mocks-in-acceptance-tiers.yml`; `just test`, `just test-live`.*

### 14.2 Minimum acceptance cases

Twenty-five named cases in blueprint §14.2, made concrete in `tests/ACCEPTANCE_PLAN.md` — a
frozen contract. They include the ones most easily skipped: hostile downloaded instructions,
URI and path traversal, project immutability, stub/runtime conflict, static no-import safety,
cancellation, restart recovery, and two-client stdio operation.

*Evidence: Proposed — Phases 1–6.*

### 14.3 Operational metrics

Cache hits and misses, producer duration, queue depth, fetched and response bytes, LSP startups
and reuse, verification outcomes, evidence gaps. **Engineering diagnostics, not a benchmarking
project**, and never a universal research-quality percentage. blueprint §14.3.

*Evidence: Proposed — Phase 6.*

### 14.4 Definition of done

Working code, dependency locks, generated schemas, sample config, bootstrap and install scripts,
CLI and stdio entry points, the skill, fixture and live tests, an operations guide, decision
records, and a truthful test report — with unsupported libraries, platforms and formats
documented. **Completion requires a demonstrated Rust and Python end-to-end path.** Optional
features may be explicitly deferred; required paths cannot be replaced by TODOs, canned evidence
or mock production responses. blueprint §14.4.

*Evidence: Proposed.*

## 15. Design review

Blueprint §15 is a ten-question checklist, asked before declaring a design or implementation
complete. It is not superseded; it is routed onto the charter's gates G1–G7 in
[`ADDENDUM.md`](../design_review/design_principles/ADDENDUM.md) §2, so that answering it and
settling the gates are the same act.

A design review is **evidence, never authority**. It finds defects and cites measurements; it
does not change this document. The ADR that responds to a finding does. Reviews live in
[`../design_review/reviews/`](../design_review/reviews/) and the standard they apply is
[`DATA_MODEL_DESIGN_CHARTER.md`](../design_review/design_principles/DATA_MODEL_DESIGN_CHARTER.md).

Every finding names the executable oracle that would catch a regression — a hook, an `ast-grep`
rule in `rules/`, a `just` gate, or a test — or says explicitly that none exists. That absence is
the most valuable thing a review produces, because `rules/` is where it lands.

*Evidence: Tested — `just adr-lint` validates the corpus and `just adr-lint-test` is 33
behaviour tests over the lint itself; both run in `just ci`. The review loop end to end is
Implemented, not yet Measured. See ADR-0009.*

## 16. Governing instruction

> Implement the smallest complete, reproducible evidence service that lets a strong coding agent
> discover useful Rust/Python capabilities, retrieve precise support, verify consequential
> assumptions, and explain deployment choices. Preserve Rust ownership of the evidence model, use
> FastMCP 4 as a thin interface, use ty for Python semantics, and keep Context7 and design
> reasoning in the calling agent. Optimize for correct end-to-end operation and bounded evidence,
> not for building another universal knowledge platform.

blueprint §16, unchanged.

---

## Revision history

Every amendment adds a row here naming the revision, the date, what changed, and the ADR that
decided it. See [`README.md`](README.md) for the amendment rule.

| Revision | Date | Change | Decided by |
|---|---|---|---|
| 1 | 2026-09-13 | Seeded from the frozen blueprint (revision 1.0, 2026-09-13) as the living design spine. §B1–§B13 given stable IDs; the eight existing decision records attached to the sections they govern. | ADR-0009 |
| 2 | 2026-09-13 | Evidence labels corrected to the measured state of the tree after the Phase-1 slice landed. No design change; §B1–§B13 and every section number are unchanged. Prompted by finding F1 of the 2026-09-13 design review, which found the spine describing a system two phases behind the code. | — (maintenance; see the preamble) |
