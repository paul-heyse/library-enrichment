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

> Decision: ADR-0041 — Plan 15 replaces the previous execution/storage mechanism; see §16.

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

*Evidence: Implemented — Rust owns resolution, acquisition, native evidence storage/retrieval,
closed durable jobs, execution policy and publication. Python remains the adapter/static worker boundary.*

#### B3 Python boundary

A thin FastMCP 4 adapter, a separate Griffe extraction worker, and isolated runtime probes.
**Never a reimplementation of the core in Python.** The adapter validates inputs, calls the
daemon, emits MCP results and maps structured errors; it does not index libraries or own
persistent state. blueprint §1.1, §2.1.

*Evidence: Implemented — the thin adapter forwards Rust-owned envelopes; the separate Griffe worker
uses the generated request/response contract with static inspection disabled.*

#### B4 Python semantic engine

Astral **ty**, through its CLI and LSP boundary, not its private Rust internals. Not pyright, not
mypy. If `ty` is unavailable the affected gates are `blocked` and it warrants an ADR — it is never
grounds for substituting pyright or mypy. blueprint §1.1, §5.4.

**pyrefly** is admissible alongside ty: it may be executed, indexed as a subject of study, and may
in a later record become the production engine. It is not the production engine today, and no
`SemanticObservation` originates from it. Decision: ADR-0046.

> Decision: ADR-0005 — ty does implement `textDocument/implementation`; gate P08 is superseded by P08a/P08b, and [S18] must not be used to derive any capability assertion.

*Evidence: Tested — ADR-0005 records the runtime probe against ty 0.0.80.*

#### B5 Rust semantic engine

rust-analyzer over LSP, isolated behind a producer adapter. Neither language server is embedded
as a Rust library in this repository. blueprint §1.1, §9.2.

*Evidence: Tested — `crates/enrichment-daemon/src/lsp/` drives real rust-analyzer 1.98.1 and ty
0.0.80 as subprocesses inside a capsule; neither is linked. Gates P08a, P08b and C17, 2026-09-14.*

#### B6 Context7

A separate MCP connection used directly by the calling agent. No Context7 proxy, no embedded
LLM, no recursive research agent inside this service. A Context7 result with an uncertain source
version is a **discovery lead, never exact-version proof**. blueprint §1.1, §3.3, §11.

*Evidence: Implemented — the service has no Context7 dependency; the skill carries the routing.*

#### B7 Storage

> Decision: ADR-0026 — retain typed execution observations through concrete durable jobs; exact retained reads never trigger execution, and committed results repair interrupted delivery journals.

> Decision: ADR-0023 — immutable relational catalog generations and exact-file snapshot manifests have one durable visibility boundary.
> Decision: ADR-0024 — bounded shared DataFusion execution over admitted providers and typed Arrow batches.

Immutable raw artifacts and Arrow/Parquet evidence snapshots; DataFusion for structured
retrieval. This is an **evidence cache, not a replacement for the future graph/fact store**.
Arrow and DataFusion are pinned to a verified mutually compatible set; one Rust type universe.
blueprint §1.1, §8.4.

*Evidence: Implemented — ten typed evidence relations, native relational catalog, DataFusion
ingestion/selection and one mandatory native admission worker. Plan 12 records current validation.*

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

*Evidence: Implemented — real fixture, producer and client journeys exist on the target architecture.
Current acceptance results and the independent replay are recorded in STATUS.md and the Plan 12 ledger.*

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
**`library-enrichment-worker`**, a Python static extraction worker emitting schema-versioned
JSON that **cannot publish snapshots**. The contained executor handles approved execution; the
mandatory `library-enrichment-native-worker` isolates native parsing and admission allocations
under ADR-0031/ADR-0033. blueprint §2.1.

> Decision: ADR-0006 — the daemon RPC contract.

*Evidence: Implemented — daemon, stdio adapter, Python static extraction worker, contained executor
and mandatory native parser/admission worker are included in the supported build and launch surface.*

### 2.2 Lifecycle

Explicit `start`, `status`, `stop`. The adapter may auto-start the daemon under an OS lock; the
daemon survives adapter exits and retains jobs, and an adapter disconnect does not implicitly
cancel a shared job. **MCP initialization registers the tool catalog and nothing else** — no
language servers, downloads, compilation or registry refreshes. blueprint §2.2 [S05].

*Evidence: Implemented — prompt catalog registration and daemon-owned durable Resolve/Inspect/Verify/
Compare jobs; shared interests survive adapter disconnection and unrelated caller cancellation.*

### 2.3 Filesystem layout

> Decision: ADR-0045 — Plan 15 replaces the previous execution/storage mechanism; see §16.

Implementation in the repository; regenerable data under the cache home
(`downloads/`, `unpacked/`, `capsules/`, `lsp/`); retained evidence and job records under the
data home (`blobs/`, `snapshots/`, native `catalog/`, `jobs/`, `bundles/` and bounded
publication staging). Context selection is a catalog relation, not a parallel contexts directory. Resolved
by an OS-aware directory resolver with overrides, never by assuming Linux paths. blueprint §2.3.

> Decision: ADR-0003 — `.dev-state/` for development sessions, defined once in `scripts/env.sh`.  
> Decision: ADR-0008 — the harness-path exemption, scoped to `plans/`, deliberately not all of `~/.claude/`.
> Decision: ADR-0018 — what `bundles/` holds: a directory with a `sha256sum`-format manifest, carrying the service's own identities and claiming only that a copy is intact.

*Evidence: Tested — `just state-leak-check`; the `pre_edit.sh` service-state deny class. Gate C20
digests a canary repository by path, mode and content hash under every enabled profile, with the
daemon started inside the canary, 2026-09-14.*

## 3. Research identity, versioning, and freshness

### 3.1 Four identities

> Decision: ADR-0022 — semantic observation/producer identities precede snapshot identity; actual attempt attribution and physical digests are separate.

> Decision: [ADR-0015](../adr/0015-immutable-revision-acquisition.md) — explicit immutable repository acquisition.

> Decision: ADR-0014 ([bounded evidence comparison](../adr/0014-bounded-comparison.md)).

> Decision: ADR-0013 ([preserve Python observations](../adr/0013-static-python-evidence.md)).

> Decision: ADR-0012 — preserve declared feature knowledge, absent observed builds and coherent historical snapshot reads.

`release_id` (the actual release or source), `environment_id` (declared or resolved
environment), `context_id` (release + environment + research mode, carrying whether the
environment is `unspecified`, `declared`, `resolved` or `verified`), and `snapshot_id` (an
immutable evidence set). **Enrichment creates a new manifest; it never overwrites a snapshot**,
and resolving previously unknown environment fields yields a *new derived context* rather than
mutating an old ID. rustdoc's local item IDs are never cross-release global IDs. blueprint §3.1.

*Evidence: Implemented — `identity/mod.rs`; content-derived and covered by unit tests.*

### 3.2 Research modes

> Decision: [ADR-0015](../adr/0015-immutable-revision-acquisition.md) — explicit immutable repository acquisition.

`project` (the exact supplied environment, never silently upgraded), `upstream` (the newest
eligible published release as of a recorded registry check), `compare` (two exact contexts, with
environment differences shown as such rather than as library changes), and `revision` (a pinned
development revision, kept distinct from a release and from a mutable branch name).
blueprint §3.2.

*Evidence: Implemented — project, upstream, compare and immutable revision resolution use closed
Rust request types and exact context identities; revision/retention fixtures cover replay.*

### 3.3 Freshness is not one timestamp

> Decision: ADR-0026 — retain typed execution observations through concrete durable jobs; exact retained reads never trigger execution, and committed results repair interrupted delivery journals.

> Decision: ADR-0027 — absent built Python Name/Version headers are nullable declarations; present scalar headers must match exactly and remain required for PyPI acquisition.

> Decision: [ADR-0025](../adr/0025-retained-release-metadata.md) — offline resolution reads qualified retained metadata; request freshness is separate.

> Decision: ADR-0022 — retain valid exact-context facts indefinitely in the target lifecycle; current development evidence is disposable at the explicit pivot reset.

> Decision: ADR-0020 — retain validated evidence indefinitely for its exact release/environment; mutable lookup freshness does not expire facts.

At least `retrieved_at`, `registry_checked_at`, `source_revision`, `artifact_digest`,
`producer_version` and `documentation_version_match` are recorded, with version matching
distinguished as `exact`, `compatible_claimed`, `mismatched` or `unknown`. Freshness options are
`cache_ok`, `revalidate` and `offline`; a request for "latest" either revalidates or returns a
clearly marked inability to verify latest. blueprint §3.3 [S07].

Exact retained contexts are reusable without age-based expiry. A fresh registry lookup that
selects the same release reuses its evidence. New versions create separate evidence and preserve
older pinned snapshots. Only explicit operator cleanup removes retained facts; temporary
execution cleanup has a separate lifetime. Fragments carry acquisition-specific source URIs;
legacy unknowns cite immutable artifact handles rather than unrelated acquisition metadata.

*Evidence: Tested 2026-09-14 — Rust/Python retention and revision fixtures exercise ancient
registry timestamps, zero TTL, restart/offline reuse, unchanged latest selections, repeated
revision revalidation and identical bytes acquired for different releases.*

## 4. Evidence acquisition: Rust

### 4.1 Preferred path

> Decision: ADR-0048. The Rust fact worker and pure renderer use one exact format-61 model;
> stability and default-body facts are preserved independently of signature text.

Exact release and target/features → registry metadata → docs.rs build metadata and hosted
rustdoc JSON → normalize public API and re-exports → overview, exact-symbol search, public-API
diff → targeted docs, examples, release notes → rust-analyzer or a compile probe only as needed.
See §B13. blueprint §4.1 [S07].

*Evidence: Implemented — `ops/resolve.rs` runs this path end to end for a real crate.*

### 4.2 Producers and outputs

> Decision: ADR-0033. Raw rustdoc/public-api parsing runs in the resource-bounded
> native worker; finite producer records feed the single native Arrow ingestion path. The Python
> worker installs Rust-supplied limits before source parsing and retains complete bounded docs. Oversized
> declarations become explicit gaps with the source artifact retained.

Six: registry/manifest reader; `cargo metadata` in a service capsule (matching the project
target and features, never defaulting to `--all-features`); hosted or local rustdoc JSON; the
`public-api` library consuming rustdoc JSON directly (the CLI is a diagnostic adapter, not the
canonical model); a targeted documentation and source reader; and rust-analyzer over LSP with
its analysis scope recorded. blueprint §4.2 [S09, S10].

*Evidence: Implemented — hosted-first native rustdoc/public-api extraction, retained source/docs,
qualified capsule Cargo metadata/local fallback, and recorded rust-analyzer observations.*

### 4.3 Documentation build is not project configuration

> Decision: [ADR-0015](../adr/0015-immutable-revision-acquisition.md) — explicit immutable repository acquisition.

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

*Evidence: Implemented — qualified dated-nightly fallback retains lock, target/features, image and
actual process receipts. R09 uses an identical consumer and dependency closure on stable and nightly.*

### 4.5 Discovery beyond API additions

Capability discovery inspects crate and module overviews, feature descriptions, examples,
migration notes and release notes: behavior can change without new public symbols. Where a
version-to-commit relation cannot be verified it is marked approximate rather than treating the
default branch as released behavior. blueprint §4.5.

*Evidence: Implemented — bounded documentation/example/release-note indexes and native comparison
retain explicit missing coverage. Unavailable migration guidance remains an evidence gap.*

## 5. Evidence acquisition: Python

### 5.1 Preferred path

> Decision: [ADR-0015](../adr/0015-immutable-revision-acquisition.md) — explicit immutable repository acquisition.

> Decision: ADR-0013 ([preserve Python observations](../adr/0013-static-python-evidence.md)).

Exact distribution and interpreter/platform/extras → PyPI metadata and a suitable artifact →
package/import mapping, metadata, inline types and stubs → Griffe static API snapshot →
`objects.inv` and targeted official documentation → ty in a synthetic consumer capsule →
isolated runtime inspection only when needed. blueprint §5.1 [S14, S15].

*Evidence: Implemented — exact Python acquisition, native snapshot/retrieval, selected ty semantics
and separately qualified runtime inspection; actual producer journeys are in tests/e2e/.*

### 5.2 Producers and outputs

> Decision: ADR-0013 ([preserve Python observations](../adr/0013-static-python-evidence.md)).

Six: registry/distribution reader; wheel and source inspector; **Griffe worker (§B12)**;
documentation inventory reader; `ty` process; and a restricted runtime worker whose observations
are separately labeled as executed evidence. Griffe's breaking-change checker is supplemental
interpretation, not a complete feature-addition detector — set and field differences are
computed over our own normalized snapshots. blueprint §5.2 [S12, S13].

*Evidence: Implemented — Rust owns distribution/admission and comparison; the static worker,
documentation inventory reader, ty sessions and explicit runtime producer feed typed evidence.*

### 5.3 Types and dynamic APIs

> Decision: ADR-0013 ([preserve Python observations](../adr/0013-static-python-evidence.md)).

Stubs, inline annotations and runtime objects can disagree: **store separate observations and
report conflicts**, never reconcile them silently. Python publicness is not one Boolean —
`__all__`, documented status, underscore convention, re-export pattern and author declaration
are separate signals. `objects.inv` is a navigation signal, not API extraction. A missing
docstring does not prove an API is private or absent. blueprint §5.3 [S16, S17, S20].

*Evidence: Implemented — source/stub alternatives and typed runtime observations remain distinct;
malformed, absent and unsupported results produce explicit limitations.*

### 5.4 ty integration

The pinned `ty server` executable over negotiated LSP, and a pinned `ty check` against a
generated consumer capsule with its own selected interpreter — **never against this service's own
virtual environment**. Capabilities are probed at runtime and unsupported outcomes preserved
rather than invented. blueprint §5.4 [S18, S19].

> Decision: ADR-0005 — `textDocument/implementation` is supported; [S18]'s table is unreliable and must not ground a capability assertion. Gates P08a and P08b replace P08.

*Evidence: Implemented — qualified capsules drive actual ty LSP and check commands; supported
locations, incomplete protocol results, reuse and retained-capsule integrity have real fixture consumers.*

### 5.5 Supporting parsers and installed metadata

`importlib.metadata` is used only against a service-owned environment whose identity is
retained, and never by importing the target library. Ruff lints the service's own Python; its
internal crates, tree-sitter and a custom source-fact extractor are **not** prerequisites. A
targeted syntax adapter is an optional escalation that emits the same evidence records rather
than starting a parallel fact store. blueprint §5.5.

*Evidence: Implemented — distribution metadata is read from selected artifacts and installed
metadata in qualified capsules. Optional parser platforms remain deferred.*

## 6. Canonical evidence model

A small Rust-owned typed model, not a large ontology. Stable enums and typed relationships cover
observed evidence; free text is for excerpts and explanations, never for essential machine
state. blueprint §6.

### 6.1 Required record types

> Decision: ADR-0022 — separate definitions, public bindings, independent API observations,
typed relationships/fragments and actual producer-attempt associations.

The blueprint's concepts remain, with normalized Arrow relations for definitions, symbols,
API observations, relationships, fragments, producer runs, inputs and coverage. Release,
environment, context and snapshot identities are catalog relations. Lexical path ancestry is a
derived navigation relation, distinct from observed MemberOf edges. Source/stub/runtime values
remain independent qualified observations. JSON sidecars carry the small commit protocol;
queried domain structure is typed. *Evidence: Implemented — typed Arrow relations, native staging joins and incremental canonical
semantic hashing are the single ingestion/query path.*

### 6.2 Separate epistemic classes

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0043 — Plan 15 replaces the previous execution/storage mechanism; see §16.

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

> Decision: ADR-0056 (proposed) — comparison values retain declared native variants through wire delivery.

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0043 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0041 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0040 — immutable native catalog binding and one effective policy (Tested; Plan 14 native and installed consumers).

> Decision: ADR-0037 — Rust domain DTOs generate schemas and Python models; authored Pydantic presentation composes tool output variants without owning domain policy.

> Decision: ADR-0035 makes same-path ambiguity actionable with typed candidates and an exact definition selector.

> Decision: ADR-0034 adds the Rust-owned native query status projection.

> Decision: ADR-0029 — Distinguish selected API projections from complete retained observations.
> Decision: ADR-0031 — Isolate native Parquet admission allocations.

> Decision: ADR-0033. Raw rustdoc/public-api parsing runs in the resource-bounded
> native worker; finite producer records feed the single native Arrow ingestion path. The Python
> worker installs Rust-supplied limits before source parsing and retains complete bounded docs. Oversized
> declarations become explicit gaps with the source artifact retained.

> Decision: ADR-0026 — retain typed execution observations through concrete durable jobs; exact retained reads never trigger execution, and committed results repair interrupted delivery journals.

> Decision: ADR-0027 — absent built Python Name/Version headers are nullable declarations; present scalar headers must match exactly and remain required for PyPI acquisition.

> Decision: [ADR-0025](../adr/0025-retained-release-metadata.md) — `release_metadata` is the ninth evidence relation, with native Rust-docs/Python-distribution structs and lists, semantic identity and input closure.

> Decision: ADR-0022 — one current storage reader/writer (format 5.0 after ADR-0025/0026); no historical readers or data migration. Existing development evidence is disposable at a scoped reset.
> Decision: ADR-0023 — validate complete physical and relational contracts before exposing optimizer key assertions.

> Decision: ADR-0014 ([bounded evidence comparison](../adr/0014-bounded-comparison.md)).

> Decision: ADR-0013 ([preserve Python observations](../adr/0013-static-python-evidence.md)).

> Decision: ADR-0012 — preserve declared feature knowledge, absent observed builds and coherent historical snapshot reads.

**Rust DTOs and the canonical table model are authoritative.** Versioned JSON Schemas are
generated from the Rust wire types, and the small Pydantic boundary DTOs from those schemas.
Arrow projections are declared in the Rust table contracts; target semantic round trips are required.
Upstream raw JSON is preserved in content-addressed blobs. Schema and normalizer versions go
into content keys; **wall-clock timestamps and temporary paths are excluded from semantic
content hashes** and retained in provenance instead. blueprint §6.3.

> Decision: ADR-0007 — ruff does not lint the generated boundary DTOs; `ty` still does.

*Evidence: Implemented — Rust table contracts and generated wire schemas own all target storage
and boundary types. Native admission and schema conformance validate their consumers.*

### 6.4 Claim-oriented source priority

There is no universal best source. Exact signature → the selected artifact, compiler or stubs;
supported usage → version-matched author documentation and examples; runtime behavior → source
plus targeted execution; architectural fit → the calling agent's reasoning with linked premises.
**Contradictions are retained as conflicting observations**; a stub annotation never silently
overwrites a runtime observation. blueprint §6.4.

*Evidence: Implemented — source, stub, semantic and runtime alternatives retain their producer
and environment qualification. Actual runtime disagreement journeys exercise this distinction.*

## 7. MCP tool contracts

Nine tools with one ecosystem-neutral vocabulary for Rust and Python: `resolve_library`,
`library_overview`, `search_evidence`, `inspect_symbol`, `compare_releases`, `verify_usage`,
`read_artifact`, `job_control`, `service_status`. blueprint §7 gives inputs and required results.

*Evidence: Implemented — all nine tools call real daemon methods. Missing capability or an
unqualified execution profile returns an explicit scoped gap or policy error.*

### 7.1 Tool behavior requirements

> Decision: ADR-0036 — One explicit research selection drives native requested-scope assessment and independent aspect pages. ADR-0038 governs operation preparation and budgets.

> Decision: ADR-0035 makes same-path ambiguity actionable with typed candidates and an exact definition selector.

> Decision: ADR-0028 — Compose cold comparisons and reuse exactly qualified retained execution.
> Decision: ADR-0029 — Distinguish selected API projections from complete retained observations.
> Decision: ADR-0030 — Admit complete delivery artifacts before committing successful jobs.

> Decision: ADR-0024 — shared typed eligibility and comparison projections lower into native DataFusion plans; specialized scoring remains a pure Rust batch kernel.

> Decision: ADR-0014 ([bounded evidence comparison](../adr/0014-bounded-comparison.md)).

Resolution returns candidates and an actionable ambiguity rather than guessing, and never reads
a lookup as an upgrade request. Overview represents unindexed sections as **unknown, not empty**.
Search is deterministic lexical and token matching with recorded scoring factors — no embeddings
in v1 — and reports what it covered. Inspection defaults to public API plus a small documentation
section and does not start an LSP for a signature already normalized. Comparison computes observed differences and per-side coverage. A signature representation
change does not establish source incompatibility, and a clean API diff does not mean unchanged behavior.
Verification's caller-supplied profile can only *select* from locally enabled profiles, never
grant one, and reports `not_run` rather than a fabricated success. Artifact reading resolves only
service-issued handles — **never a general filesystem path reader**. blueprint §7.1.

*Evidence: Implemented for all nine tools — native retrieval/comparison, exact bounded artifact
reads, durable pending completion and explicit policy-controlled execution.*

### 7.2 Common response envelope

> Decision: ADR-0056 (proposed) — comparison values retain declared native variants through wire delivery.

> Decision: ADR-0054 — Wire 5.0 native projections measure complete admitted MCP tool/resource stdio responses; application qualification remains open.

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0037 — Research/2.0 replaces the prior root pagination and nested terminal-result contract.

Research/2.0 retains explicit outcome, scope, freshness and source evidence while separating
`delivery` from the native outcome. `ok` means success within the declared scope. Coverage
assessments distinguish indexed, partial, missing and unknown; indexed-empty is meaningful only
within its evaluated domain. Each requested aspect and alternative set has its own page/cursor.
There is no root pagination field. Errors carry a typed cause, stage, rule, affected identities,
limits and recovery actions. Nullable root fields remain required even when their value is null.

The current wire 9.0 contract is generated from Rust into `schemas/generated/`. The older
`contracts/research-v2/research-envelope.schema.json` is frozen provenance, with no compatibility reader.
Authored Pydantic presentation models use generated domain types and validate actual inline,
artifact, pending and error outputs. Individual comparison values are tagged inline/artifact;
artifact variants retain complete JSON, digest, size and original alternative FactSource.
Comparison values remain declared native variants through bounded final delivery. The native
prefix plan selects inline or artifact output; the sink only decodes/encodes that selection.
Schema-derived field facts carry typed paths (including declared parameter ordinals) and exact
canonical native bytes; DataFusion set operators select differences without JSON or hash equality.
Parent paths retain complete-alternative correlation and whole-sequence ordering. The bounded
Arrow lowering kernel is an immutable UDF, avoiding a separate optimizer branch per field.
Plan 19 comparison uses one `ComparisonValue` declaration for native set equality and inline or
artifact output. API observations reuse typed `ApiPayload` callable facts; documentation fields
are excluded from the API axis. Source qualification stays separate. Wire 9.0 / state 23 refuses
obsolete state and has no compatibility decoder. Durable/installed qualification remains open. Indexed result dependency closure is verified and included in
catalog admission/export, with descriptors for offline reads of presentation-only artifacts.

Job, caller-interest and producer-attempt identities have separate 16-byte Arrow domains. Typed
SQL parameters retain their complete Fields; only boundary formatting produces prefixed text.
The producer-attempt change advances the evidence snapshot schema to 12.0. Qualification and
job clocks share declared timestamp meanings. Wire epoch identity comes from one vocabulary.
Telemetry retains the native Diagnostic and tagged recovery actions directly, with the same
sequence semantics as delivery; no separate failure JSON record interprets those fields.


Plan 19 native delivery measurement borrows complete Arrow results, including scalar Structs,
and streams nested JSON text without hydrating an owned result DTO. The envelope field declaration
also generates its native wire projection; a common MCP formatter serves measurement and selected
RPC output. Selected JSON bytes and the complete RPC frame retain separate DataFusion pool
reservations through socket delivery. Serde RawValue preserves the encoded payload without a
semantic JSON-tree round trip, and response metrics use the counting sink.
Research citations, excerpt limits, overview counts/coverage/summary and fragment recovery presence
are native plans over declared records. Fragment completeness travels in the same typed record as
its text/source, and empty-page outcomes are qualified by native coverage before claiming absence.
Inspection source absence, execution limitations and missing kinds are composed in a native relation.

*Evidence: Tested for the cases recorded in the Plan 13 ledger; final client and deployment
qualification remains open.*

### 7.3 Output budgets

> Decision: ADR-0054 — Wire 5.0 native projections measure complete admitted MCP tool/resource stdio responses; application qualification remains open.

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0045 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0036 and ADR-0037 — Independent aspect/alternative pages and complete indexed artifact delivery preserve scope and native outcome.

> Decision: ADR-0029 — Distinguish selected API projections from complete retained observations.

> Decision: ADR-0030 — Admit complete delivery artifacts before committing successful jobs.
> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0024 — keyset cursors bind semantic query/snapshot/sort versions; complete envelope and execution budgets are separate obligations.

Wire 5.0 enforces `max_bytes` against the complete admitted response, including measured MCP
stdio framing and resource-text escaping. Native DataFusion plans select inline or retained views
using the exact format kernel. Required error recovery is native-owned; an impossible cap yields
a minimum-budget refusal. Python validates the native projection against the pinned SDK encoding
and forwards it without selection or trimming. The canonical envelope appears once in tool
structured content. Protocol/availability errors are separate local transport observations.

> Decision: ADR-0014 ([bounded evidence comparison](../adr/0014-bounded-comparison.md)).

Configurable initial limits, explicitly **not measured performance claims**: a 12 KiB inline
response budget, 12 search results per page, an 800-character excerpt, an 80-line source
excerpt, a 2-second inline wait then a job receipt, Rust-configured producer concurrency and warm-session capacity. Machine JSON is never truncated mid-object — fewer complete entries are returned
with a cursor or an artifact handle, and mandatory coverage and error fields are preserved.
Cursors bind the query digest, snapshot, sort and position plus the complete service-policy identity
and a captured runtime witness (effective session options, compiled source/function/codec closure and
resource settings). A mismatched cursor is rejected. Native page plans select counts, sentinel state
and continuation. Requested coverage and comparison confounders/disposition are relational projections;
comparison scopes obtain their evidence mapping from the immutable declaration catalog. A borrowed-Arrow
JSON-size kernel supplies exact value costs, and native cumulative aggregates select each alternative
prefix before artifact writing. Physical sinks check the selected byte witness.
blueprint §7.3.

*Evidence: Implemented — budgets, pagination cursors bound to the query digest and snapshot,
and excerpt limits are enforced on the search and inspect paths. The limits are configured
defaults, **not measured performance claims**.*

### 7.4 FastMCP implementation

> Decision: ADR-0054 — Wire 5.0 native projections measure complete admitted MCP tool/resource stdio responses; application qualification remains open.

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0037 — FastMCP 4.0.3 validates original inputs and emitted tool variants; optional progress cannot replace a completed outcome.

Typed Pydantic request and result models generated from or checked against the wire schemas; a
small human-readable summary plus compact structured data, never the same long document twice.
One Rust resource declaration generates the four resource templates and static workflow resource,
including names, URIs, operation bindings and parameter schemas. Templates delegate to the same
core read operations as the tools, taking IDs rather than paths. Python only constructs FastMCP
components from those bindings. The default adapter does not depend on MCP background-task support: expensive
operations return an ordinary `pending` envelope and `job_control` works with any client that
supports ordinary tools. Annotations are assigned deliberately — `verify_usage` is not portrayed
as read-only. **All logs go to stderr or daemon logs, never MCP stdout.** blueprint §7.4
[S02, S03, S04].

*Evidence: Tested — `rules/mcp-stdout-protocol-only.yml`; `tests/contract/test_mcp_catalog.py`.*

## 8. Orchestration, caching, and long-running work

### 8.1 Producer plans, not bespoke scripts

> Decision: ADR-0044 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0038 — Foreground operations and durable jobs have distinct deadline, admission, correlation and retained-output owners.

Typed `ProducerSpec` and `ProducerPlan` records over a small explicit dependency graph for the
known producers — **not a generic orchestration platform** (§B11). A tool requests missing
evidence kinds and the daemon plans the minimum additional producers; work already done for
matching content and configuration is reused; when one source fails, usable evidence from others
is retained with a `partial` status. blueprint §8.1.

*Evidence: Implemented — typed producer specs drive the Rust plan; a failed source retains the
other producers' evidence with a `partial` status.*

### 8.2 Single-flight and publication

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0042 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0037 — Complete indexed result closure is admitted before job success; recovery reads committed bytes without generating replacement results.

> Decision: ADR-0030 — Admit complete delivery artifacts before committing successful jobs.
> Decision: ADR-0031 — Isolate native Parquet admission allocations.
> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0021 — reserve host output space before writes; admit complete manifest-bound generations only after validated handoff and confirmed cleanup.

> Decision: ADR-0023 — in-process commit coordination, expected-base same-context rebase and one durable relational catalog generation.

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

> Decision: ADR-0012 — preserve declared feature knowledge, absent observed builds and coherent historical snapshot reads.

Producer work is deduplicated by `(producer_version, normalized_options, input_digests,
environment_id, policy_profile)`, and cancelling one caller's interest must not kill work another
still needs. **Only the daemon publishes.** Workers write staged outputs under job-specific
paths; schema, references, digest integrity and expected coverage are validated before an
immutable snapshot directory is published and the context pointer atomically updated. Readers
never observe partially written tables. blueprint §8.2.

Publication validates bounded Arrow batches and exact artifact closure, synchronizes immutable
files/manifests, then commits one catalog generation. The daemon ownership lock excludes a
second process; a commit coordinator merges concurrent additions against the latest generation.
Same-context additions require the expected base or validated rebase, never stale last-writer
selection. Read/export leases protect selected files. *Evidence: Implemented — native catalog commit/rebase, exact closure and leases; process-kill
publication, same-context distinct snapshots and failed precommit delivery have executable oracles.*

### 8.3 Job state

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0044 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0042 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0037 — Job/5 exposes a compact outcome and readable delivery descriptor, not recursive full research envelopes.

> Decision: ADR-0030 — Admit complete delivery artifacts before committing successful jobs.

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

Seven persisted states: `queued`, `running`, `succeeded`, `partial`, `failed`,
`cancel_requested`, `cancelled`. A pending result carries a job ID, a progress stage and an
advisory next-poll delay — **not a guarantee of completion time**. After a restart, unfinished
committed job results are recovered from their admitted immutable delivery. Precommit work is
marked interrupted with an actionable error; restart does not silently re-execute target code. blueprint §8.3.

*Evidence: Implemented — closed durable jobs, committed-result recovery, explicit interruption
of precommit work, cleanup supervision and retained leases through confirmed resource removal.*

### 8.4 Cache invalidation

> Decision: ADR-0055 (proposed) — native shared cache ownership; see §17 for current implementation and open qualification.

> Decision: ADR-0045 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0041 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0040 — immutable native catalog binding and one effective policy (Tested; Plan 14 native and installed consumers).

> Decision: ADR-0023 — coherent catalog and snapshot lifetimes; cleanup honors read/export leases.
> Decision: ADR-0024 — bounded reusable admission/providers over exact content dependencies.

Validated exact-context evidence has no age expiry. Mutable lookup TTLs do not expire facts.
Producer/environment/policy/normalizer semantics determine reuse; actual attempts and clocks
remain provenance. Query provider/metadata eviction releases memory without reacquisition.
Changed files or validation contracts require re-admission; paths/mtimes are only witnesses.
One bounded RuntimeEnv serves scoped immutable request catalogs. *Evidence: Implemented — exact admitted providers and qualified retained execution reuse share
one bounded native runtime; eviction releases caches without deleting evidence.*

> Decision: ADR-0038 — Request, policy, snapshot and resource ownership span all native child work.

An operation binds its request/effective-policy digests and admitted snapshot file identities,
catalog generations and retention leases. Bounded query diagnostics record that binding alongside
the actual native plan and function inventory. A blocking Arrow result sink retains the existing
query permit through worker exit even after a caller timeout. Retained output and streamed
artifact bytes have separate cumulative operation limits; neither is a process RSS measurement.
No cross-operation result cache is implied by these diagnostic bindings. Operation-local reuse
remains a measured W11 choice.

## 9. LSP and verification implementation details

### 9.1 Synthetic consumer capsules

> Decision: ADR-0044 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0021 — keep admitted inputs read-only; execute in bounded tmpfs and validate exact retained inventories before reuse.

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

A capsule is a service-owned workspace holding exact dependency resolution inputs, a selected
toolchain or interpreter, a small consumer file and analysis settings; its manifest records what
was reproduced from the working project **and what was not**. A library API scan and a
project-compatible consumer check are different operations, and an API scan is never labeled a
successful integration test. blueprint §9.1.

*Evidence: Implemented — selected-artifact capsules and Rust-owned Python closure under ADR-0017.
Focused real MCP probes retain actual environment/lock identity; project locks, native/source
dependency builds and unsupported closure cases are not reproduced. See the resumption handoff.*

### 9.2 LSP session manager

> Decision: ADR-0028 — Compose cold comparisons and reuse exactly qualified retained execution.
> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0021 — initialize each language server from validated read-only inputs into bounded scratch while retaining its continuous execution lease.

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

The daemon owns warm sessions keyed by server version and capsule digest, implementing framing,
initialization, capability negotiation, document lifecycle, diagnostics, cancellation and
shutdown. The caller supplies a symbol ID or a snippet and never manufactures line/column
coordinates; negotiated position encodings are respected rather than assuming byte offsets match
UTF-16. **An empty result, an unsupported method, an unresolved dependency and incomplete
indexing are four distinct outcomes.** blueprint §9.2.

*Evidence: Tested — warm sessions keyed by `{server, image_id, capsule_digest}`, with negotiated
position encoding and the four outcomes kept distinct in `SemanticOutcome`. Gate C17 asserts the
start counter is unchanged across a signature-only inspection and increments exactly once across
two semantic inspections sharing a capsule, 2026-09-14.*

### 9.3 Probe classes

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

Three: a Rust compile probe (`cargo check` or `cargo test --no-run` on the project compiler and
lock), a Python typecheck probe (ty against the recorded interpreter and stub environment), and
a runtime probe executing bounded code with explicit assertions. Type correctness is not runtime
correctness. Exceptions and missing native dependencies are preserved as **evidence gaps, never
as "the API does not exist"**. The daemon does not auto-generate invented successful examples;
an agent-supplied snippet is stored as `snippet_origin=agent`. blueprint §9.3.

*Evidence: Implemented and exercised by the actual verification, runtime-object and semantic
fixture journeys. Compiler/typechecker success and runtime failure remain distinct. The exact
supported environment and source/lock scope accompany each observation; this is not blanket
project integration certification. Current integrated results are recorded in the Plan 12 ledger.*

## 10. Execution policy and source handling

> Decision: ADR-0039 — Revision archives omit only explicitly safe link entries and retain a typed extraction/source-closure receipt; missing required inputs remain incomplete.

> Decision: ADR-0031 — Isolate native Parquet admission allocations.
> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0033. Raw rustdoc/public-api parsing runs in the resource-bounded
> native worker; finite producer records feed the single native Arrow ingestion path. The Python
> worker installs Rust-supplied limits before source parsing and retains complete bounded docs. Oversized
> declarations become explicit gaps with the source artifact retained.

> Decision: ADR-0021 — one Rust broker description controls explicit bounded scratch, executor identity, validated output handoff and qualification invalidation.

> Decision: ADR-0017 (supersedes ADR-0016) — retain isolated producers and durable interests; Rust admits Python dependency metadata before following registry edges.

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

*Evidence: Tested — profiles are enforced in the core and reported per image by `service_status`,
which keeps *enabled*, *runtime present* and *qualified* as three separate facts. Gate C20 digests
a canary repository by path, mode and content hash under every enabled profile; R10 proves a
configured `build` profile without isolation is `POLICY_DENIED` rather than a host fallback,
2026-09-14.*

## 11. Companion skill: Context7 + enrichment workflow

`skills/library-research/SKILL.md` is the **product** — the skill this service ships. It is
governed by this design, not by the development harness, and is never installed into user
configuration without an explicit operator-invoked setup command. `.claude/skills/` is the
separate directory for development skills. blueprint §11.

*Evidence: Implemented — the frozen product skill installs through the generational installer;
live MCP instructions clarify that absent cfg hints cannot prove project availability.*

### 11.1 Required research loop

Eight steps: frame the question; resolve the environment; discover broadly enough; retrieve
explanations narrowly through Context7; deepen selected candidates; **verify only material
uncertainty**; produce a compact decision brief separating factual support from architectural
inference; and stop. The agent is the orchestrator — the service does not call Context7 itself
to enforce this. blueprint §11.1.

*Evidence: Implemented — actual native Codex/Claude event correlation and independent daemon
witnesses validate completed research. Qualitative brief assessment remains part of final acceptance.*

### 11.2 Capability discovery without a graph

Two passes: a small breadth pass over modules, features and documentation headings, then depth
on a shortlist, searching by desired outcome as well as by likely mechanism. A name-based query
misses unfamiliar capabilities; a full API dump overwhelms the agent; **a faceted overview
bridges that gap** — which is why §B11 excludes a graph and embeddings. Machine facts stay in
snapshots; judgments stay separately labeled. blueprint §11.2.

*Evidence: Implemented — faceted overview followed by selected depth, checked through actual
client research and supported evidence citations rather than tool-name mentions.*

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

*Evidence: Implemented — supported client CLI/configuration contracts, isolated per-scenario
homes, separate Context7, explicit minimal credential reuse and real-user configuration digests.*

### 12.3 Skill installation

One source skill in this repository, installed at user scope rather than copied into every
project, only after checking for an existing installation and refusing to overwrite an unrelated
skill. An installed source/version manifest is stored, with explicit update and uninstall
commands. **No automatic edits to user configuration during ordinary test runs**; setup scripts
state their destinations and support a dry run. blueprint §12.3 [S23, S25].

*Evidence: Tested 2026-09-14 — the preview/apply generational installer exercises install/update/
uninstall and actual interrupted initialization/intent/pointer recovery, refusing foreign or changed
bytes. These checks also passed in final functional CI.*

### 12.4 Optional later remote use

An HTTP adapter can reuse the same tools and core, with authentication and explicit
user/namespace boundaries before any exposure beyond loopback. A later deployment profile, not a
blocker. blueprint §12.4.

*Evidence: Proposed.*

## 13. Implementation sequence and acceptance gates

> Decision: ADR-0011 — R08 and P06 belong to Phase 3; their assertions and IDs are unchanged.

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

*Evidence: Implemented with real fixture, contained producer and client scenarios. Current
source-bound integrated acceptance is recorded in `STATUS.md`, the Plan 12 ledger and generated
`docs/reports/acceptance.json`; an old report or architecture-only checks cannot certify it.*

### 14.3 Operational metrics

> Decision: ADR-0041 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0040 — immutable native catalog binding and one effective policy (Tested; Plan 14 native and installed consumers).

> Decision: ADR-0038 — Preparation attempts receive early query identities, stage diagnostics and native rule failures; metrics describe actual execution only.

> Decision: ADR-0034 exposes bounded native query status.

`health.native_queries` reports process-scoped physical execution/completion/interruption totals,
summed planning and elapsed microseconds, current admitted queries, and configured concurrency,
managed-memory, spill and metadata-cache limits. Null means no query runtime is open. Recorded
query totals include preparation attempts, including failures before physical plan creation; overlapping durations are not
whole-process wall time. Limits are configuration, not peak RSS or library completeness.

> Decision: ADR-0024 — bounded diagnostics describe the executed plan, admission/scan/hydration costs, memory/spill and actual scope without a second hidden execution.

Cache hits and misses, producer duration, queue depth, fetched and response bytes, LSP startups
and reuse, verification outcomes, evidence gaps. **Engineering diagnostics, not a benchmarking
project**, and never a universal research-quality percentage. blueprint §14.3.

*Evidence: Tested — `crates/enrichment-daemon/src/metrics.rs` and `enrichment_core::wire::status`,
published in `service_status` and written as one structured JSON line per request to stderr.
`tests/e2e/test_operational_metrics.py` asserts each counter tracks the fact it names: a cache hit
is not a revalidation, reused bytes are not downloads, a declared gap is not an error, and a probe
the service could not run is not a failed probe. Scoped to one process, with `uptime_seconds`
beside them so a count is readable; never rolled up, 2026-09-14.*

### 14.4 Definition of done

Working code, dependency locks, generated schemas, sample config, bootstrap and install scripts,
CLI and stdio entry points, the skill, fixture and live tests, an operations guide, decision
records, and a truthful test report — with unsupported libraries, platforms and formats
documented. **Completion requires a demonstrated Rust and Python end-to-end path.** Optional
features may be explicitly deferred; required paths cannot be replaced by TODOs, canned evidence
or mock production responses. blueprint §14.4.

*Evidence: Implemented — locks, generated schemas, configuration, all three native executables,
stdio adapter, worker, generational installer, skill, fixture/live/client tests and operations
instructions are connected. Thirty-five ADRs record the decisions. Final functional CI, contained
and live checks passed; all ten actual client outputs were reviewed, and the final overview repair
passed targeted checks and full CI. The Plan 12 integrated review accepts this functional scope.
The user stopped redundant replay after registry-only corrections; current-registry certification
remains incomplete and older receipts are not promoted. STATUS.md records the exact boundary.
Performance tuning is deferred by explicit user instruction on 2026-09-14.*

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

## 17. Unified native execution and Delta authority


> Decision: ADR-0055 (proposed) — shared committed snapshots, positive contracts and canonical metadata ownership.
> Decision: ADR-0053 (proposed) — typed resident ingredients, restricted binary persistence and portable native export reads.

**Source implementation, 2026-09-17:** one runtime owns bounded native DefaultCache families for
metadata, snapshots, verified contracts and immutable provider ingredients. Native incremental
refresh discovers current heads; exact-version values and acknowledged writer results share
accounted allocations. Changed root incarnations invalidate reuse. Contract manifests are compared
natively before positive insertion; failures never become cached absence. Effective session options
and actual function bindings have one immutable witness shared by cache consumers.

In-process provider reuse never serializes. Every provider entry point checks exact read protection
and rebinds a fresh provider from typed native state. Service search checkpoints persist bounded
CBOR descriptors with raw IPC buffers; native admission checks their bijection and digests. Every provider preparation, including a previously verified descriptor hit, reconstructs exact
metadata-only history under fresh read protection. Exact-version snapshot-registry hits use the
same version/metadata/protocol check. A missing durable commit cannot be supplied by a cached
full snapshot. Full active-file allocations remain shared; metadata-only authority never enters
the scan registry. Restoration reuses native EagerSnapshot reconstruction.
Export checkpoints explicitly select portable Delta log reads and contain no absolute-root provider
payload. Copying or atomically renaming a bundle therefore does not change its read contract.

CDF reader construction receives the session's shared metadata cache and canonical root-store
paths. Unconsumed statistics/listing caches are disabled. Concrete occupancy uses native accounting,
not copying all cache entries or claiming RSS. Snapshot and provider reservations remain held by
live physical consumers after eviction. Read-only captures carry both exact dependency vectors and
a matching immutable-root lock. Private export-stage removal uses the durable object store,
namespace invalidation and a retained cleanup receipt.

Native retention now uses one exact table/version/contract plus optional `RowKey` selection for
cohorts, retained results and definitions. `PendingRow` captures the target contract and field/value
before writing; an exited process's intent is resolved against native Delta state before cleanup.
A missing table is not sufficient evidence of absent output: incomplete files keep the obligation
open. Root removal, product visibility and new-reader admission share the native retention catalog.
Standalone artifact receipts select their own roots atomically with receipt publication. Artifact
reclamation anti-joins live roots/readers/writers, then holds a durable maintenance fence and an
exclusive root lock through acknowledged unlink and directory synchronization.

Lease, temporary-input and quarantine owners reserve release capacity before physical admission.
Closing drains admitted releases, including descendants; surviving physical owners refuse closure
before diagnostics or caches are torn down. Native schema registration/discovery share a bounded
schema-only IPC preflight before Arrow conversion. These are implementation facts; complete physical
restart, partial-failure, corruption and installed qualification remain Plan 19 work.

The complete mutation, retention, replay, external-allocation, fresh-process and installed matrices
remain open in Plan 19. Source implementation and isolated units do not certify those journeys.

> Decision: ADR-0051 — every Delta kernel path uses explicitly owned I/O handlers; separate compute and I/O scheduling prevents synchronous callers from exhausting the pool needed for their awaited filesystem work.

> Decision: ADR-0050 — native maintenance protects retained deletion vectors, admits complete bounded log inventory and preserves supplied commit policies at every stage.

> Decision: ADR-0049 — bounded atomic native listing and controlled Delta factory opening preserve one owned session, store, exact snapshot and semantic scan contract.

> Decision: ADR-0048 qualifies the shared format-61 producer/renderer model.

> Decision: ADR-0047 — one semantic declaration and generated typed boundaries supersede the earlier schema/presentation mechanisms.

> Decision: ADR-0041, ADR-0042, ADR-0043, ADR-0044, ADR-0045.

The owner selected the complete Plan 15 architecture. Where earlier sections describe procedural
normalization/scoring, worker JSON, Parquet catalog manifests, current-pointer publication,
independent job journals or retention of old runtime formats, this section replaces those
mechanisms. Their useful code-insight, ownership, provenance and bounded-delivery outcomes remain.

All operations use native Arrow/DataFusion contracts. Delta supplies purpose-specific evidence
tables and one typed control transaction for claims, publication vectors, expected context heads,
projection offsets and terminal results. Complete logical-input Delta builders share the concrete
native session and retention planner. No unqualified provider DML may bypass admission.
Producer facts enter as Arrow IPC; native plans normalize, select, score, validate and compose
results. Only bounded format/value kernels and owned external mechanisms remain bespoke.
Typed commands and exact inputs are replay authority; current physical plans are reconstructed.
CDF incrementality, native retention/maintenance and structured metrics have actual consumers.
Activation uses fresh state and removes old executable/state/artifact paths without compatibility.

Evidence: Interface-checked for upstream mechanisms; implementation and qualification are tracked
in [Plan 15](../plans/15-unified-datafusion-delta-runtime-hard-pivot.md). Earlier Tested labels
refer to the superseded implementation, not completion of this target.

### Plan 17 semantic contract authority — decision, 2026-09-16

The active target is [Plan 17](../plans/17-schema-governed-unified-runtime-hard-pivot.md), integrating
all remaining runtime obligations with schema engineering. One finite Rust declaration using Arrow
fields, enums and DataFusion expressions supplies variants, domains, requiredness, collection
meaning, scoped references and wire rules. Generate every field-bearing boundary from it. No
independent DTO schema, role-prefix policy, literal variant allowlist or historical epoch reader
remains in the target. Native relations still own all service decisions and result selection.

Full-field UDF contracts and a pre-coercion semantic analyzer enforce domains beyond metadata;
validate derived logical/physical fields as well. Storage uses sound NOT NULL plus generated
parent-aware native predicates and feature-enabled Delta CHECK operations, with native relational
admission for collection/reference conditions. Exact names and field contracts must be checked
before casts that could insert NULL. Typed clocks, digests, IDs and coordinates retain exact value
meaning through canonical identity, storage and generated wire projections.

Native result relations select bounded pages and recovery. Before encoding, establish a checked
escaped-byte bound and reserve allocation; a capped Write after Arrow JSON buffers an entire row
is insufficient. FastMCP receives explicit bounded/empty content and generated structured content,
avoiding its automatic text duplication. Python remains a mechanical transport/extraction boundary.
Complete results and retrieval actions remain precommitted with terminal outcomes.

Evidence: Interface-checked decision, with executed library probes and an accepted document-stage
review. Product implementation is in progress; Plan 17 SC/Q oracles remain unqualified. Activate
one complete fresh epoch and delete replaced runtime/state without compatibility. Earlier dated
implementation evidence below describes its own checkpoint, not completion of this target.

### Exact Arrow metadata boundary — implementation evidence, 2026-09-15

At delta-rs `58f07cd6`, `writer/utils.rs::arrow_schema_without_partitions` builds its schema
without top-level Arrow metadata while `record_batch_without_partitions` retains batch metadata.
`ArrowContract` is a finite logical format adapter that survives optimization and lowers to
DataFusion 55.1's `ProjectionExec::try_new_with_schema_metadata`. It preserves inferred types,
nullability and values at write/derived-plan boundaries. Native DeltaScanConfig now restores the
read schema from the native IPC schema registry. `NativePlanner` composes the format adapter with
the aggregate `DeltaExtensionPlanner`; the
retention planner remains outermost. This does not introduce a value-normalization engine.

Evidence publications now select typed table ID/version/cohort/schema-contract vectors in the
Delta control record. They have no JSON manifest or standalone evidence-Parquet authority.
Fresh bundles re-encode selected cohorts through native Delta builders and record their own
control identity; source control identity is provenance. The bounded snapshot-manifest MCP
resource includes publication-scoped change summaries from exact CDF windows and native set
operations. Search consumes materialized API and fragment surfaces, maintained from bounded CDF
changes through native old/new lineage joins. A typed control checkpoint selects exact output
versions and complete source offsets together. Revision/source changes and native missing-history
conditions select an explicit full recomputation; I/O, identity, schema and resource errors remain
failures. The real publication fixture proves incremental/full multiset equality, fresh export,
missing-history rebuild through a native Delta checkpoint, and isolation of unselected output.
Plan 19 persists an immutable maintenance selection before physical work: exact table identity,
protected versions, log floor, observed clock and cutoff. Native enrollment rejects below-floor
history even after failed or interrupted reclamation. Completed uncommitted writers can be removed
only under native eligibility and a physical root fence, after Delta's log-path parser proves no
committed history exists. Private and incomplete trees share bounded, no-symlink durable deletion.
State 15 is the required hard cutover for this witness; physical fault qualification remains open.

The complete mutation/fault matrix, replay and durable retention remain open Plan 15 work.

Exact-source follow-up, 2026-09-15: the pinned CDF builder clamps an explicit end version to
`get_latest_version(start)` even with out-of-range tolerance disabled. A snapshot checkpoint can
therefore remain readable while its requested CDF end is silently omitted. The native adapter
requires both a readable end commit and a latest version at least that large before constructing
the feed. Production publication holds retention exclusion through preparation and control commit;
preflight alone is not a reclamation-race proof. Explicit Chrono minimum/maximum timestamp bounds
cover the service's supported timestamp domain without epoch/local-clock selection. Malformed
external timestamps outside that domain are outside this qualified local-write route. See the
[pinned CDF implementation](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L203)
and [native log-store contract](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L785).
Projection revisions include the compiled transform/runtime policy definition and dependency lock;
input table identities, semantic contracts, cohorts and versions remain explicit checkpoint fields.



### Native acquisition and identity contracts — implementation evidence, 2026-09-15

Sparse-index records enter bounded Arrow batches with physical source-line provenance. Native
filters and top-k select the exact request, eligible newest versions and nearest candidates.
Python file facts retain their typed hashes, requirements and artifact fields; native joins/UNNEST
and ordering implement wheel tags, interpreter/prerelease/yanked policy and artifact rank. The
only version kernels parse SemVer/PEP 440, expose ordered values and evaluate the pinned library's
specifier predicate. Dependency markers compile to native expressions over explicit environment
and independent extra assignments. The acquisition frontier and durable source cache are still
pending replacement; these native selectors do not imply complete native acquisition.

`core/native_key.rs` declares 17 implemented identity contracts. Their expression graph uses
native ordered sets/maps, the bounded Arrow byte kernel, SHA-256 and hexadecimal encoding. Corpus
plans bind columns to the contract. Bounded DTO constructors use `ExprSimplifier` constant folding
of the identical graph, without another session, runtime or JSON preimage. Target keys use full
64-hex digests; missing environment feature knowledge is rejected rather than inferred by an
old-record decoder. Evidence/provenance identities use the core Arrow fields; policy and result
identities remain Plan 15 work.

The canonical domain is `enrichment/identity/7/<prefix>`; names and variable-length values have
little-endian u64 length frames, typed nulls differ from values, and list order remains meaningful
unless the identity contract explicitly applies native distinct/sort. Snapshot maps use native
`map_entries` and sorting. DataFusion 55.1's `MapEntriesFunc` constructs `key`/`value` struct fields
with nullable values, so the declared input map uses exactly that native layout before invocation.
Independent framing/SHA-256 vectors and identity metamorphism passed the scoped core fixture.
The generated JSON schemas/DTOs carry the new full-hash ID constraints; complete epoch and legacy
fixture cleanup is still part of WP11.

### Native contribution and admission — implementation evidence, 2026-09-16

`core/evidence/arrow_model` supplies the evidence schemas and value contracts. Native field
admission traverses those schemas at plan construction, derives vocabularies from core types,
and executes native predicates for tags, references, coordinates and coverage consistency.
Content identities and cross-relation membership have separate native violation plans. Complete
execution/URI/extension/metadata-value and resource admission is still required before retiring
all format-boundary validators.

`publish_native` accepts the ten native relation plans and one contribution scope. Bounded
execution DTOs mechanically encode Arrow batches; streaming producer inputs retain private Arrow
files. Native distinct/union and environment derivation feed complete Delta builders directly.
Admission then checks the private immutable cohort vector that a control transaction can select.
The intermediate native-plan-to-Arrow writer and Rust coverage-set folds are deleted. Native
coverage aggregation supplies both publication and subsequent read validation. Both producer
semantic walkers are deleted; procedural attempt/metadata composition remains implementation work.

The pinned Delta reader relaxes nested Parquet fields, including list items, before restoring
its logical schema. Collection predicates over required nested fields exposed a nullable-to-required
cast failure at that boundary. Core evidence Arrow fields now use the native nested read layout;
`enrichment.null=forbidden` retains required semantic membership, enforced by native predicates
before admission. Provider metadata distinguishes physical nullability from required contract
membership. Parquet filtering remains enabled. Source read on 2026-09-16:
[Delta nested read schema](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/table_provider/next/scan/expr_adapter.rs).

A conservatively nullable native expression is accepted as write input when its declared types
and metadata match; the full Delta writer enforces actual required root values. The focused
write probe passed for a present nullable value and refused an actual NULL without a new version.
[Upstream nullability enforcement tests](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/mod.rs)
were read on 2026-09-16. These are implementations within ADR-0043's preserved semantic contract,
not a relaxation of required evidence.

Private input directories are retained by logical scan leaves, optimized physical plans and
streams; a focused lifetime test passed. Native environment derivation/source preservation and
repeated contribution identity passed. Full cancellation/Delta-write ownership and the final
nine-tool installed qualification remain open.

### Native Python fact execution — implementation evidence 2026-09-16

The core owns one generated Arrow IPC V5 worker contract. PyArrow 25.0.1 only encodes Griffe
facts and file/stream receipts; no response-wide JSON corpus or Python semantic normalizer remains.
The daemon streams stdout under a byte ceiling into an owned private source. DataFusion's native
Arrow source reads it directly, and native inventory/ordinal/tag queries admit the stream. File
receipts select fully extracted files, so partial file observations retain raw provenance without
becoming published declarations.

Recursive native alias closure groups ambiguity before following an edge. A membership relation
and equality join bind own and terminal declarations without constructing wide PythonSymbol
objects. Typed native projections and keys produce definitions, bindings, independent API
observations, relationships and fragments. Native UNNEST expands bases and overloads. The shared
read-only native policy captures closure depth; native failure relations expose missing targets,
ambiguous declarations, cycles and depth exhaustion. Producer component provenance records that
policy value. Source/stub differences are retained as separate evidence.

Physical Arrow contract projections use DataFusion's native CastExpr when physical string-view
rewrites differ from the declared storage type. Metadata and field order remain authoritative.
No native optimizer or filter rule is disabled. The pinned nested-leaf optimizer exposed an invalid
wide UNION projection during development; the final binding-membership relation gives the
optimizer one equality-join path. The real worker/native-plan/five-table Delta readback check and
independent semantic decoders passed; full daemon, scale and installed qualification remain open.

## Revision history


Every amendment adds a row here naming the revision, the date, what changed, and the ADR that
decided it. See [`README.md`](README.md) for the amendment rule.

| Revision | Date | Change | Decided by |
|---|---|---|---|
| 33 | 2026-09-17 | Governed native caches, typed snapshot ownership, shared CDF metadata, binary service replay and portable export reads. | ADR-0053/0055 (proposed) |
| 33 | 2026-09-17 | Closed typed comparison alternatives, full native fields and generated wire 7.0; hard state-14 cutover. | ADR-0056 (proposed) |
| 32 | 2026-09-17 | Native wire 5.0 tool/resource projection, exact SDK framing facts and native inline/retained byte-fit selection. | ADR-0054 (proposed) |
| 31 | 2026-09-17 | Consume restricted immutable Delta descriptors with current read protection and native bounded cache ownership. | ADR-0053 (proposed) |
| 30 | 2026-09-17 | Bind declared operation reuse through CacheFactory and owned native spill execution; application qualification remains open. | ADR-0052 (proposed) |
| 29 | 2026-09-16 | Bind kernel handlers through both LogStore and DataFusion TaskContext, preserving one shared resource/policy runtime with owned compute and I/O lanes. | ADR-0051 |
| 28 | 2026-09-16 | Preserve exact retention and owned commit policy through native Delta maintenance. | ADR-0050 |
| 27 | 2026-09-16 | Compose native providers through bounded discovery, exact owned opening and corrected nested CHECK formatting. | ADR-0049 |
| 26 | 2026-09-16 | Qualify one format-61 model for facts and the patched pure renderer. | ADR-0048 |
| 25 | 2026-09-16 | One native schema/variant/domain authority, generated typed storage and wire, semantic plan checks and preallocation-bounded MCP encoding; Plan 17 hard pivot. | ADR-0047 |
| 1 | 2026-09-13 | Seeded from the frozen blueprint (revision 1.0, 2026-09-13) as the living design spine. §B1–§B13 given stable IDs; the eight existing decision records attached to the sections they govern. | ADR-0009 |
| 2 | 2026-09-13 | Evidence labels corrected to the measured state of the tree after the Phase-1 slice landed. No design change; §B1–§B13 and every section number are unchanged. Prompted by finding F1 of the 2026-09-13 design review, which found the spine describing a system two phases behind the code. | — (maintenance; see the preamble) |
| 3 | 2026-09-13 | Preserve feature knowledge and align comparison gates with Phase 3. | ADR-0011, ADR-0012 |
| 4 | 2026-09-13 | ADR-0013 | Additive Python evidence and static worker contract. |
| 5 | 2026-09-13 | ADR-0014 | Pinned comparison and complete response budget contract. |
| 6 | 2026-09-13 | ADR-0015: repository/root/commit acquisition and scoped static revision evidence |
| 2026-09-13 | ADR-0016 | Specify isolated verification, durable interests and immutable execution inputs at Proposed strength. |
| 2026-09-13 | ADR-0017 | Replace networked uv resolution with bounded Rust-owned metadata admission; retain other ADR-0016 contracts. |
| 7 | 2026-09-14 | Record the shape of a provenance bundle, the surface §13 named in one phrase and §2.3 gave a directory. | ADR-0018 |
| 8 | 2026-09-14 | Evidence labels corrected to the measured state after Phases 4, 5 and 6 landed: §B5, §B10, §2.3, §9.2, §10, §11.1, §11.2, §12.2, §14.2, §14.3 and §14.4. No design change and no section renumbering. Phase 5's sections are labelled **blocked**, naming the missing prerequisite, rather than carried as Proposed or promoted on an untested claim. Prompted by register row R-14, whose trigger is a phase landing without the spine being updated in the same commit. | — (maintenance; see the preamble) |
| 9 | 2026-09-14 | Bind acquisition citations and semantic inputs, retain exact-context evidence without age expiry, and separate storage 3.0 compatibility from wire 1.0. | ADR-0020 |
| 10 | 2026-09-14 | Specify read-only admitted inputs, bounded scratch, quiescent output handoff, conservative reservations and qualification identity. Implementation acceptance remains open. | ADR-0021 |
| 11 | 2026-09-14 | Hard pivot to typed Arrow evidence, one coherent relational catalog and bounded DataFusion execution. Drop development historical compatibility; target contracts accepted at Proposed strength. | ADR-0022, ADR-0023, ADR-0024 |
| 12 | 2026-09-14 | Retain qualified release metadata in Arrow for offline resolution without stored response JSON or repeated extraction. | ADR-0025 |
| 13 | 2026-09-14 | Specify typed retained execution results, canonical coordinates, explicit effects and durable publication/journal reconciliation. Accepted contract; implementation qualification remains open. | ADR-0026 |
| 14 | 2026-09-14 | Preserve absent built Python metadata as null with checked header projections and strict PyPI validation. | ADR-0027 |
| 17 | 2026-09-14 | ADR-0034: bounded native query and admission status. | ADR-0034 |
| 16 | 2026-09-14 | Accept durable comparison/reuse, bounded inspection, precommitted delivery, native allocation and resource contracts after scoped review; actual journeys remain Plan 12. | ADR-0028–ADR-0033 |
| 15 | 2026-09-14 | Extend the native allocation boundary to Rustdoc and Python extraction; remove the daemon raw AST. Scoped architecture review accepted. | ADR-0033 |
| 18 | 2026-09-14 | Select an existing definition within its public path and snapshot; return typed ambiguity candidates. | ADR-0035 |
| 19 | 2026-09-14 | Reconcile implemented native storage, tools, producer ownership, durable jobs and installer evidence labels with Plan 12. | Maintenance |
| 20 | 2026-09-14 | Record completed functional validation and the user-stopped registry-only audit boundary; retain deferred performance work. | Maintenance |

| 21 | 2026-09-15 | Research/2.0 selection, coverage, delivery, native preparation, adapter presentation and revision omission contracts; scoped design acceptance with final qualification open. | ADR-0036, ADR-0037, ADR-0038, ADR-0039 |
| 22 | 2026-09-15 | Record implemented Arrow value streaming, indexed artifact dependency closure and operation-owned input/permit lifetimes. | ADR-0037, ADR-0038 |
| 23 | 2026-09-15 | Immutable catalog/schema binding, one native policy/discovery path and consumed physical facts; final native and installed consumer evidence recorded. | ADR-0040 |
| 24 | 2026-09-15 | Adopt complete native execution, Delta control, semantic mapping, owned effects and fresh-epoch replay/retention. | ADR-0041–ADR-0045 |


### Plan 13 derived comparison publication refinement (2026-09-15)

ADR-0037's precommit result obligation also applies to comparison jobs. A comparison publication
is a separate typed catalog relation binding its request digest, terminal outcome, result descriptor
and both exact context/snapshot pairs. It is derived research, with no producer attempt invented
for the query. Native reference checks admit both inputs and package compatibility. The after
snapshot owns export reachability; bounded native selection follows required before snapshots and
copies their complete closure. Catalog/5 and bundle/5 replace the earlier development candidates.
Committed indexed bytes win a terminal-journal interruption and are verified on recovery without
rerunning acquisition or comparison. Result writing participates in the operation's cumulative
artifact budget, including when called from a blocking publication worker.

Revision receipts additionally retain candidate source roots reached through bounded explicit and
workspace-inherited local dependency manifests. Native omission joins use those roots, while
missing declared files are independent failure facts. Receipt policy revision-extraction/3 and
producer revision-source/4 distinguish this collection scope. Declared presence never establishes
Cargo resolution, generated inputs or complete source/build coverage.


Comparison detail cursors select one side's alternatives. The opposite side retains its observed
presence with an unknown count, without repeating large value hydration. Nonempty alternative pages
may stop at the remaining artifact byte allowance; every continuation advances. Values are measured
from borrowed Arrow before an immutable write. Minimum pages that cannot fit remain capacity failures.
Blocking native publication work retains operation identity and admission through worker exit; its
callback/future is heap-owned before polling. The operation policy digest includes the full configured
limits/freshness/producer policy, excluding where the configuration file was loaded from.


### Native operation-local reuse (2026-09-15)

Search's folded key/score relation, overview's chosen child relation and comparison's changed-key
relation are computed once for their multiple consumers. The private index uses DataFusion 55.1.0
DiskManager/SpillFile quota ownership, Arrow IPC streams and the native StreamingTable provider.
Long search text and comparison alternatives stay in admitted source relations until selection.
Exact count, ordering, joins and page semantics remain DataFusion work. The index is private to
one operation's isolated catalog; its captured request/config identity and snapshot leases stay
alive until the final provider/reader drops. No computation lookup or persistent cache is added.

Each scan batch has the runtime's existing bound. Readers reserve managed decode workspace before
opening the unbuffered stream; writers reserve encoding workspace. Native disk quota remains
authoritative, and an additional counter-based preflight preserves a typed capacity witness when
that limit is already provable. Generic native concurrent I/O errors remain I/O rather than being
guessed from message strings. Arrow wrappers preserve concrete nested I/O/native causes.

Fixed research decoder families now include unsigned counts, namespace kind counts/summary/child
projections and full fragment rendering. Per-operation diagnostics retain attributed child query
counters, queue/planning time, cumulative result/artifact charges and index reads/spill bytes in
a bounded 32-entry completion history. It describes owned lifetime, not total process RSS or
protocol latency. Artifact follow-ups are separate operations. Measurement evidence and remaining
limits are in [the native operation report](../reports/plan13-native-operation-measurements-2026-09-15.md).

Qualification state is generation-scoped beside its explicitly selected execution-image root.
Requalification preserves older sidecars and uses the current generation; it never initializes
production XDG state, migrates old evidence or resets prior files.

### Native semantic scope and portable recovery (2026-09-15)

Python nominal-class selection is a qualified native relation, shared by semantic consumers.
A semi join selects the symbol's source observations; aggregate FILTER counts and UNNEST of
declared bases require every selected declaration to establish the supported nominal scope.
Unknown, conflicting or unsupported declarations do not silently become supported. The native
decision has an independent Arrow result-field contract and contributes its implementation
identity to producer provenance. The adapter makes no Python semantic decision.

The namespace overview summary is an indexed native relation whose partition and result-family
contracts are checked before decoding. Search scores/keys, comparison keys and overview indexes
remain private to one operation and are released with its native leases and spill ownership.
The measured relationship endpoint plan retains the semi join: for the sampled high-fanout
symbols, four keyed joins plus union/deduplication cost more despite using hash joins.

For MCP error results, portable text includes a bounded projection of the native recovery
diagnostic. A failed durable job includes its immutable result-read action when one exists.
Canonical structured output and the MCP error flag keep the original meaning. Long explanations
may be omitted from text within the framing allowance; this is explicitly labeled. Optional host
presentation does not become another evidence model or job state machine. Actual installed
launch selection binds the daemon, both native helpers, adapter and worker interpreter together.


### Plan 13 implementation and deployment closure — 2026-09-15

The shared native research architecture described above is implemented and serves research/2.0
from one verified fresh-state generation. Independent final-source and real installed-client
qualification passed; the scoped closing review accepts G1–G7. The [final qualification report](../reports/plan13-final-qualification-2026-09-15.md)
records J01–J20, D01–D12, exact component hashes and the 47 active acceptance gates. Old evidence
is preserved inactive with no compatibility reader. Production execution permissions remain
static-only; separately qualified execution does not silently change that policy. Explicit source,
semantic, budget and measurement limits remain part of the accepted design.

### Plan 14 immutable native binding (2026-09-15)

**Tested — final native and installed consumers.** Existing finite relation declarations and codec schemas feed
an immutable captured inventory. Native catalog/schema providers expose admitted evidence,
transparent domain views and folded service records; raw history and candidates have explicit
internal scopes. Temporary operation tables have a separate mutable schema. Comparison uses
qualified independent pins. Construction succeeds completely before an inventory is installed.

Validated configuration feeds one immutable native policy and its read-only ConfigExtension.
The actual Parquet format consumes that policy; bounded structural/domain metadata describes the
same resolved objects. Exact physical statistics require admitted facts. Completed indexes retain
counts and only established ordering/partitioning. Native string coercion preserves supported
encodings. Rule transitions and managed-pool peaks have bounded storage and truthful scope.

Plan 14's deletion and functional oracles are mapped to the final source in
[the qualification report](../reports/plan14-final-qualification-2026-09-15.md). The measured policy
selects decoder/reorder, observation-ID Bloom, capped whole-file groups and independent storage
layout. Query diagnostics use v2 derived history; canonical state formats remain target-compatible.
No compatibility namespace,
legacy reader, second policy interpreter or speculative remote/catalog platform is selected.


### Native Rust fact execution — implementation evidence 2026-09-16

Rustdoc and public-api syntax extraction runs in the owned native worker and emits seven
bounded Arrow streams: header, items, containment links, canonical path observations, external
crates, individual rendered occurrences and missing renderer inputs. It does not walk the public
API, choose aliases, select one rendering per ID or create evidence identities. Complete raw
rustdoc bytes remain the acquisition evidence. The worker reports its actual compiled definition
revision; parent and decoder identities are recorded separately.

DataFusion recursive queries compute containment and public use paths. Visibility depends on the
raw containing role, so default trait members/enum variants differ from private module or inherent
implementation members. Renamed type reexports retain member paths. Native plans classify generated,
blanket and negative implementations, preserve their relationships and retain renderer alternatives.
Public-api item/parent IDs are candidate associations: one binding permits a symbol subject;
several bindings of one definition permit a definition subject. Other renderings remain library
fragments; ambiguous definition association is an explicit native diagnostic. Core-owned Arrow
expressions construct tagged records, typed inactive fields and all native identities.

The recursive working schemas explicitly use Arrow Utf8View for text. This composes with
DataFusion 55.1's physical string-view rewriting without disabling an optimizer or lying about a
recursive field type. The common native Arrow contract adapter casts final values to their declared
semantic types. Rust and Python contribution plans reach the same Delta publication boundary.
The Rust semantic walker, JSON-line transport and symbol/relationship object-ingress staging are
deleted. Document/provenance ingress now supplies typed Arrow facts and native qualification plans.

A focused fixture passed four private Delta stages, five evidence writes/readbacks and independent
identity/provenance decoding (2.08 s). Live daemon acquisition still encounters native planning
stack overflow on default stacks. An 8 MiB diagnostic reached the producer deadline and exposed a
terminal-delivery settlement gap; that boundary has been fixed but not requalified. Scale/resource
and installed product qualification remain open. No terminal Plan 15 gate is claimed passed.

### Native definition revisions and claim renewal — implementation 2026-09-16

The build derives a framed source digest and an input receipt from workspace Rust source and
manifests, Python worker/adapter contracts, configuration, lockfiles and recorded compiler/build
settings. Search projection revisions include that digest, covering core schema, identity and UDF
changes omitted by the former file list. Durable commands record their definition revision and a
native eligibility join prevents execution under a different compiled definition. Producer components
also carry the definition revision. This supplies build provenance; complete operation/input replay
and codec qualification remain open.

Initial and renewed claim deadlines use one native UTC/interval expression from shared policy.
Renewal requires the same owner and fence, a live lease, owned cleanup state and a running job.
Known Delta conflicts recompute from a new snapshot; failure or lost eligibility cancels the owned
heartbeat's execution channel. Publication and acquired-input pinning require a live claim. Dropping
or finishing an owner aborts its heartbeat. Expiry never grants replacement execution without physical
cleanup evidence. Full timer/crash/recovery qualification, old-owner cleanup reconciliation and the
finite effect-driver architecture remain open.

### Native source projections and terminal settlement — implementation 2026-09-16

Delta readers use the public `DeltaScanNext::new` and `DeltaScanConfig::with_schema` APIs with a
captured snapshot. Semantic metadata and unsigned types are adapted by the native scan. The full
stored schema and its registered contract identity are checked before exposing any override.
DataFusion and Delta builders use the exact same durable root ObjectStore handle; native session
registration alone would not replace DataFusion's implicit file backend. Read views expose no DML.

Documentation is stored once as a top-level Arrow column, while the wire payload remains a
presentation of the selected native fields. Inspection selects the other columns through native
DataFrame projection. At this Delta pin, a nested schema override preserves the requested values
but does not prune the nested I/O. The flattened contract enables the built-in column projection:
leased and unleased reads consumed about 21 KB instead of 1.02 MB with equal rows. Predicates over
transformed types remain post-scan when Delta reports unsupported pushdown.

Document decoders emit bounded Arrow facts; native plans qualify source acquisitions, provenance,
coverage and identities. Execution ingress uses native observation/input/producer closure and
result-to-attempt joins. Native admission replaces semantic DTO/file admission and enforces row and
record-byte bounds. The raw IPC encoder remains mechanical.

Query planning and complete Delta writes retain operation context and shared permits in owned
abort-on-drop tasks. Control views use native window and join expressions. A terminal result's
bounded artifact retention runs inside the same settlement scope as its durable transition, so an
expired producer deadline does not prevent recording failure. Physical cleanup and live daemon
qualification remain open; aborting an async task is not proof that an external effect has ended.

### Native executor and terminal references — implementation 2026-09-16

> Decision: ADR-0052 (proposed; application conformance pending).

Declared operation intermediates use the QueryRuntime CacheFactory and composed native planner.
Planning is pure; first execution starts one binding-owned fill, shared by independently planned
readers. Native SpillManager owns IPC mechanics, with explicit reader/writer reservations and
operation retention through physical exit. Native aggregate/selection plans consume that shared
relation. The binding is an operation capability with exact immutable input/schema/function/policy
witnesses, never a global result key. Cancellation of one waiter cannot cancel another; operation
termination cancels the fill. No cache state is persisted or used as durable publication authority.

One DataFusion RuntimeEnv owns memory, spill, cache and the qualified store registry. Its owned
Tokio compute lane runs startup, request dispatch, planning and synchronous Delta callbacks; a
separate owned I/O lane runs the kernel futures those callbacks await. LogStore::engine and the
DataFusionEngine TaskContext extension select the same configured kernel handlers. Worker counts
follow native concurrency and blocking-thread counts apply per lane. Stack size is reported
separately from Arrow memory. Tasks and running blocking callbacks retain the executor owner until
physical exit. Futures are boxed before task-local/tracing composition. Background shutdown is a
drop fallback, not evidence of graceful effect cleanup. The current-thread transport owns neither
lane. See ADR-0051 for the pinned engine seam and minimum-concurrency oracle.

Native child futures and blocking callbacks carry physical-exit tokens through DataFusion's
JoinSetTracer and the kernel TaskExecutor. Ordinary runtime spawns are tracked; only explicit
startup/export supervisors are exempt so they can own final close. Shutdown first stops and
joins transport roots and reconciles jobs, then drains native work, final ownership releases,
the diagnostic writer and its final kernel children. Release handles and failures remain owned
across a cancelled close; diagnostic close joins its retained actor and preserves lost-ack success
or persistence failure. The service's existing shutdown deadline bounds waiting; timeout does not
certify physical exit. External process/restart/pressure qualification remains open.

Operator cleanup completes native ownership recovery and joins all release/diagnostic work before
capturing its deletion inventory. The final exclusive evidence lock covers that stable inventory,
revalidation and removal. A recovery/drain error prevents deletion and remains in the report.
Replacing the remaining path-list inventory with native control selection remains Plan 17 FP12.

The workstation defaults are 32 GiB of Arrow memory, 64 GiB of spill, 2 GiB of metadata cache and
16 workers/partitions, with 16 blocking threads per lane. ArrowConfig is the single default and
conversion authority. These ceilings follow the operator's performance priority on the 192-GiB
workstation; final complete-journey measurements remain Plan 17 FP14 work.

Structured MetricsReporter/ReportGeneratorLayer events enter the same typed native diagnostic
history as query observations. All twenty pinned event variants retain exact counters, binary
kernel IDs and seconds/nanoseconds durations. Runtime task boundaries carry the tracing dispatcher,
and span-close callbacks retain their creation-time operation. The actor batches reserved typed
events into Arrow, uses an unobserved dispatcher and never recursively observes its own writes.
It keeps the native Delta snapshot returned by append for the next batch; a conflicting writer
discards that snapshot and re-enters the controlled opener. This is one actor-owned DeltaTable,
not a separate metadata authority or a claim of measured end-to-end speedup.
On close, the actor closes channel admission and drains accepted messages, including those queued
after the close barrier. Successful close is acknowledged only after persistence and physical exit;
queue-capacity drops remain explicit and distinct from accepted records.

[The operation/dataflow inventory](../architecture/native-operation-dataflows.md) maps current
public, resource, recovery and maintenance owners to remaining Plan 17 replacements. It is source
navigation; generated Rust declarations remain the contract authority.

A terminal control transition carries the complete typed Artifact descriptor. Atomic publication
projects the same descriptor from its delivery column. Job recovery uses that captured record and
returns a retained delivery; it no longer scans filesystem metadata to identify the terminal result.
Artifact metadata now has native receipt records; remaining procedural result composition is still assigned replacement.

### Native artifact receipts and finite commands — 2026-09-16

> Decision: ADR-0041, ADR-0042, ADR-0044, ADR-0045.

The four durable service job kinds lower through NativeCommand/NativeCommandExec under the
aggregate DataFusion planner. Planning and unpolled streams invoke no driver. Owned job handles
remain live beyond terminal publication until the driver returns and native cleanup is settled.
Query admission is released before polling a command so its native child queries can proceed.
Complete physical subprocess/kernel cleanup and unified policy-grant qualification remain open.

The byte store is a bounded conditional immutable writer and descriptor-based reader. It has no
metadata sidecars, first-retrieval authority, digest-prefix scanning or dependency traversal.
Artifact receipts and retained-result sections/references are typed Delta control record families.
Native joins select a captured handle; recursive plans select result and export dependencies with
explicit depth/cardinality/byte refusals. The final artifact-window driver consumes a native range.
Issued artifact handles carry their exact acquisition receipt and full SHA-256 content identity.
Native retention records select these receipts before delivery or publication. The indexed JSON
body remains a physical encoding; remaining procedural response composition and schema/epoch
qualification are still implementation work.

The active wire contract is **3.0** and the physical retained-result encoding is
**research-result/3**. The Rust emitter supplies the packaged JSON schemas and Python DTOs.
Independent current outcome/vocabulary fixtures are under `tests/fixtures/wire`; runtime
conformance no longer treats the previous research-v2 snapshot as its authority. Gate C19 keeps
its stable ID and behavioral scope across Rust, generated schema, packaged schema and adapter.
No older envelope or retained-result decoding variant is registered.


### Native execution policy and attempt publication (Plan 15, 2026-09-16)

Physical execution receipt capture is a bounded decoder. DataFusion evaluates image/root/helper
identity, enabled-profile, resource-observation and cleanup predicates. The native relation serves
status, verification, semantic inspection and local rustdoc builds, with independent qualification
for each ecosystem. Kernel output parsing uses native regex extraction, checked casts and resource
comparisons; the operator probe uses the same parser. This is implemented policy, with durable
claim-bound grant/replay qualification still open.

Actual producer attempts remain native relations during contribution and conflict retries. Native
union/conflict/reference plans feed control publication without reconstructing a Rust attempt
collection. The same Arrow metadata contract supplies native scope compatibility, observed
configuration selection and producer-count aggregation. Tagged control payloads use Delta's
nullable nested read schema, semantic presence predicates, and a native ProjectionExec schema
boundary. Export selects from each typed family, materializes one native Delta candidate and
validates its captured provider before publication. The focused comparison/export journey passed;
full concurrency, fault and installed qualification remain required.

### Native dependency and HTTP acquisition (Plan 15, 2026-09-16)

The Python dependency frontier retains immutable Arrow facts. Native demand/extra joins select
expansion and acquisition steps, enforce constraints and bounds, converge over cycles and emit
ordered lock content. The daemon parses metadata and executes the selected physical acquisitions;
there is no second selected-package map or work queue.

HTTP observations live in a purpose-specific Delta table, with body receipts in the control
catalog and exact bytes in the common immutable store. Native plans own response/Accept selection,
negative-cache freshness, validator choice and 304 metadata merging. No JSON HTTP cache remains.
The URL UDF performs syntax decoding only. One native policy applies endpoint, address and redirect
rules to original URLs and captured DNS answers. A per-hop client is bound to those admitted
addresses; automatic redirects and ambient proxy routing are disabled. The same outer monotonic
deadline covers DNS, requests and body streaming. Claim-bound grant/replay and native retention
maintenance remain required work, beyond these scoped acquisition checks.


### Native resolution and command authority (Plan 15, 2026-09-16)

Captured native joins select retained resolution versus acquisition/offline refusal. Registry
normalization is explicit: Python physical names normalize punctuation; Rust physical acquisition
preserves underscores and native catalog matching normalizes equivalence.

Core Arrow schemas now cover finite command arguments and every serialized configuration field.
Native identity expressions bind policy, command inputs and physical claim. All four daemon job
entry points use this authority; their separate JSON/Debug identity constructions are deleted.
Effective configuration lives in `delta/operation_policies`. Each retained command binds its
exact table ID, version and contract plus native policy identity. Native preconditions combine its exact
context/snapshot/environment with the shared qualification/profile relation. The resulting claim
records selected route/image, command key, policy identity and grant identity.

Native operator scopes and shared runtime tasks carry a privately constructed live grant. HTTP
and decoder entry points require current Delta ownership; HTTP additionally checks its effective
configuration against the grant and rechecks redirects. A correlation handle cannot manufacture
or resurrect a claim. Control encoding follows declared field names, so physical projection order
does not alter stored semantics. The focused Delta ownership fixture passed (92.05 s); complete
contained-process qualification, full environment/input scope, replay faults and installed
qualification remain implementation work.


Native control publication now evaluates its bounded contribution DAG once, retaining Arrow rows
for every constraint and the logical Delta append. One overflow row preserves cardinality refusal;
managed result bounds still apply. This is finite transition materialization, not corpus decoding
or a second semantic evaluator. Shared service shutdown waits for both actual driver cleanup and
durable reconciliation before a state root can be reopened; transient settlement failures keep
physical ownership installed while reconciliation retries under the configured drain deadline.


### Snapshot allocation phases (Plan 19, 2026-09-17)

Snapshot reservations follow full/metadata-only loads, incremental writes, decoded replay and the
last physical provider/reader. A separate native-cache residency token observes eviction without
releasing shared live state. Status exposes fixed-size in-flight, live-value and resident-value
reservation counters; absent measurements for other cache families remain absent. The counters
include the kernel's best-effort owned-heap estimate and declared overhead, and do not measure RSS.

A snapshot larger than the configured per-entry residency limit may execute uncached when the
native pool admits its full reservation. Both snapshot and provider-ingredient caches bypass it;
current head identity/version tracking remains in force. Admission beyond the initial replay
reservation is a checked resize after capture, not proof that every upstream allocation was
pre-reserved. External parser, metadata/predicate and writer accounting still requires completion.

### Native contained process definitions (Plan 19, 2026-09-17)

Policy configuration, executor operations and process effects use one immutable Delta definition
implementation, with separate finite tables and exact table/version/contract references. Core
Arrow contracts and native SHA expressions replace executor operation and image/containment JSON
identities. Executor protocol 5 carries one native launch declaration with exact image, containment, environment,
network and resource/output/deadline values; no old protocol fallback exists.
Input inventories include paths, kinds, permissions, lengths and complete byte digests. Native
`map_entries` and `array_sort` canonicalize maps while preserving command argument order.

Native process admission joins the live claimed command, configured route and current physical
qualification, checks build opt-in and exact image/configuration/containment binding, and retains
the admitted operation and effect. The runner consumes the exact retained operation and checks
live ownership before container creation and start. Observations distinguish command effect/grant
witnesses from operator qualification. Qualification is a private fixed-probe contract with empty
inputs/outputs and no acquisition network. Its authority cannot run target code. Warm language
servers retain resource ownership; reuse requires the new job's native admission, and every
request/notification checks the bound live grant. Cleanup retains responsibility on refusal.

Container creation consumes the retained launch values. The helper clears inherited environment
before target execution. Native actual-value comparisons include every resource and environment
entry; a matching digest alone is insufficient. Process-effect retention depends on the exact
operation definition row. Operator qualification uses this same declaration and fixed probe scope.

Each warm semantic conversation derives new authority from the retained inspection request and
an exact snapshot/context/environment. Native symbol selection and a finite immutable source
constructor produce the document, anchor and methods; retained-result lookup shares the constructor.
A native coordinate function emits explicit UTF-8 byte or UTF-16 code-unit fields. The persisted
conversation binds process/effect/grant, snapshot dependencies and the negotiated encoding. The
transport frames these selected values and exposes only the finite semantic-method vocabulary;
it rechecks current authority and native method membership before dispatch. Physical document
owners retain the exact lease. Server-request replies also recheck authority.

These changes are source implementation. Real container/worker, warm-session/revocation and
installed-client qualification, full derived-environment linkage and all retention consumers remain
required. Native diagnostic state is implemented as described below; final lifecycle and retention
qualification remains open.


### Native producer and recovery plans (Plan 19, 2026-09-17)

Executor protocol 7 carries one finite invocation declaration. DataFusion derives command bytes,
network and output roots; Rust performs the admitted physical handoff. Native admission verifies
complete values and required files and binds the invocation to the current durable command.
Verification additionally compares the captured snippet digest, and rustdoc binds target, feature
selection and opt-in. The private operator qualification path accepts only its fixed probe set.
PreparedCapsule is the shared installed-input declaration for reuse, process operations and warm
sessions. Native rules validate its parent identities, pinned target/image, resolved environment,
lock digest and captured lock file. Exact actual inventory permits only the finite producer's
consumer/helper overlays. Command/claim/selected-context joins bind that parent scope before effect
retention. A shared native encoder produces runtime selection transport and its admission witness;
compiled observer source is owned by core and checked before dispatch. Acquisition lineage and
final physical qualification remain open. Real operator containment fixtures have a private test-only
exact-contract constructor, with no production arbitrary-command API.

One native environment rule serves preparation and retained child selection. Static-input identity
is relational equality of complete digest/locator pairs. Typed process and runtime-object captures
feed native payload/outcome lowering; bounded format parsing occurs at a declared decoder UDF.
Native recovery joins command/publication/result/snapshot/attempt and artifact-role closure, checks
canonical argument values, and retains physical result protection through final delivery. JSON
transcripts are supporting artifacts and are not semantic recovery authority. Bounded native LSP
response decoding and typed protocol-coordinate UDFs feed native outcome/limitation/evidence-class
selection. A file-URI format UDF and native source route select consumer, installed, external or refused
locations. Native admission compares captured text against the installed digest/size and enforces
source closure bounds. The daemon retains bounded physical reads on the owned lane and framing.
Pending protocol I/O polls fresh authority, checks before replies and before accepting an answer,
and drops revoked conversations so the manager awaits physical cleanup. Broader acquisition/source
lineage, handler composition and the complete lifecycle matrix still require work.

### Native diagnostic event authority (Plan 15, 2026-09-16)

Core owns the typed runtime observation schemas. Physical plan traversal captures bounded raw
metrics, rule transitions and plan text into Arrow; it does not maintain query history or totals.
One Delta `native_events` relation contains query, operation-end, index and failure facts with a
runtime ID, sequence and timestamp. Native filters, ordered limits, window joins and aggregates
produce the last eight queries, last 32 operations/failures, failure-to-query support and totals.
Recovery actions retain their opaque MCP wire payload; cause, stage, affected IDs and bounds are
native columns. There is no JSON history file or second Rust summary/queue authority.

Ingress is a bounded 32-entry channel of Arrow batches with shared DataFusion memory reservations.
When capture cannot fit, status exposes dropped observations. A single writer uses the shared
executor, memory and spill pools with a separately reserved admission lane, avoiding a barrier
waiting behind its caller's sole query permit. Diagnostic output has a finite 32-row/8-MiB bound.
Diagnostic queries are unobserved to prevent feedback, and writer ownership does not form a cycle
with observed runtime handles. A read/flush barrier waits for submitted observations to commit.
Transaction versions follow writer commit order, independently of concurrent observation capture.
Persistence failure closes this authority and makes diagnostic reads fail explicitly. The service
shutdown path includes the barrier; complete writer stop/fault and retention gates remain open.

The focused runtime contract target passed all 14 cases (2026-09-16, 0.84 s), including retained
failure restart, operation correlation, memory exhaustion and owned timeout cleanup. This is
source evidence, not installed or full Plan 15 acceptance.


### Native source reads and private acquisition ownership — 2026-09-17

Source consensus, coordinates, line-window policy, archive suffix candidates and member selection
are DataFusion plans over qualified input descriptors and bounded filesystem observations. NULL
ancillary observations do not suppress a unanimous non-NULL value. Rust parses source delimiters
and executes the selected confined read on the shared blocking lane, retaining exact protection.

One durable PrivateDirectory mechanism owns document IPC, source/revision extraction and worker
Arrow output. It records obligations before creating bytes and reserves final release capacity.
Source-bearing worker requests follow enrollment of the observed direct child for input and output.
Native cleanup excludes any directory with a surviving process owner; a parent's death alone is
insufficient. Archive physical-entry bounds include implicit parents. Durable removal rechecks
file/directory identity and refuses links. State 16 drops the prior input-owner grammar and cache
layout. Source implementation is not crash/process/restart qualification; export staging, full
producer grants/supervision and external allocation/lifetime closure remain Plan 19 obligations.

### Native static extraction contract — 2026-09-17

One `source_capture` relation supplies file classification and source roles for Rust package,
revision and Python capture. The physical walker reports filenames; native joins and predicates
select artifacts, document kinds and media types. The previous `source::text_files` and handler
classification loops are deleted.

The static Python extractor consumes a private grant derived from the current Python resolution.
Its immutable Delta effect records job/policy, archive identity, complete physical input inventory,
compiled implementation identity and the native launch/request. Native inventory differences
refuse dispatch after any path/mode/length/digest change. Private source/output owners supply
confinement; the native declaration supplies argv, the complete environment, protocol and bounds.
The installed interpreter path must be absolute. No daemon PATH is inherited. The worker reads its
request-byte ceiling from the generated Arrow contract instead of a second Python constant.

Revocation is checked during parsing and before accepting results. A pre-admitted child cleanup
owner retains the paid producer permit and input/output owners until the actual child is reaped.
Pending Tokio file operations retain output ownership until flush joins them. Unknown reaping
does not become quiescence. Rust decoding shares the async child supervisor; capture and receipt I/O use the shared blocking lane.
These source mechanisms do not prove matching installed package/producer closure, complete
acquisition-to-effect lineage, process failure/restart behavior or the CP11 deletion barrier.


### Primary export staging and immutable sealing — 2026-09-17

Export uses the same durable private-directory owner as native producer inputs. An external
sibling directory records its canonical parent identity and generated child name in the primary
control namespace before creation. Native anti-joins require physical exit and matching dependency
vectors; the mechanical remover checks the actual parent and every inventoried entry. Target-local
obligations are not the only record of an unpublished export. State 18 introduces that dependency.

Native async/blocking descendants and submitted release writes carry the input owner until exit.
After target writers/providers leave scope, a nonclosing release barrier joins their submitted
writes before the final control identity and manifest. Synchronization and rename keep the same
owner; cleanup after an unknown rename acknowledgement can remove only remaining private staging.
Full crash, retention, physical publication and installed qualification stay in Plan 19 CP12.


### Retained resolution and acquisition freshness — 2026-09-17

Complete metadata values reconcile under native canonical-value expressions and consensus, with
qualification separate from value equality. NULL/empty, Map values and ordered repeated headers
remain distinct. Native gap and artifact selection precedes typed output. Rendering time is never
registry observation time: the freshness projection uses recorded acquisition clocks and the
native index/version/mode scope. Retained note replacement addresses its exact policy text and
preserves unrelated limitations. Final byte-fit governs complete artifact handles.

Operator whole-root removal checks live native leases, cleanup obligations and maintenance runs.
Unsettled private directories retain their recovery authority; private recovery precedes removal.
The separate export process coordinates physical root deletion through the existing root fence
until cleanup settles. Exact-version query authority remains native and independent of that lock.


### Native decoder, document and component declarations (Plan 19)

The Rust decoder request/effect/report is generated from `execution::rustdoc_decoder`, including
artifact identity, stream vocabulary, exact launch, output bounds and build witness. A retained
native effect ties dispatch to the current Rust resolve claim; asynchronous revocation and owned
child reaping preserve resource lifetime. Both parser kinds reserve an external address-space
envelope against the shared pool before spawn. This is conservative admission capacity, not RSS.
Kernel stream tasks propagate the same private-input owner as native query/release descendants.

Document facts use generated Arrow and wire fields. Native relations select distinct navigation
pages, inventory-version qualification and Unicode windows while complete fetched artifacts remain
retained. Component declarations likewise feed native readiness over actual execution routes;
image qualification does not independently enable a route. A missing configured Python interpreter
is explicit absence, with no PATH fallback. These source contract changes require fresh state 19.

Metadata cache observation delegates all eviction and TTL to DefaultCache. Counters describe raw
lookup hits/misses and resident accounted size without enumerating values. Reader validation and
Parquet allocations surviving eviction remain distinct and are not inferred from those counters.


### Native inspection and execution publication (Plan 19, 2026-09-18)

Inspection routes and fragment policies are catalog declarations. Native plans derive required
observations, ordered aspect outcomes and pinned recovery, retaining every failure reason instead
of overwriting a previous stage. One semantic-method declaration serves initial production and
retained reuse. Reuse checks exact runtime selectors or every requested method; independent
qualified contexts produce explicit exact-snapshot choices. Native admission selects the actual
public binding, snapshot and required profile before physical readiness and effect admission.

Inspection completeness, payloads, coverage and artifact handles are native relational projections.
A full receipt identity breaks artifact ties without discarding acquisition provenance. Verification
likewise projects initial and settled outcomes from the same captured process facts; only the
physical supervisor supplies settled cleanup. It never rewrites original process observations.

Execution publication shares one native fact constructor and admission relation across inspection,
verification and ingestion. The actual run supplies producer binding; canonical result bytes,
class/profile, intrinsic contracts and cross-field payload scope admit before ordered identities
are returned. Per-row handler fact constructors and the unused procedural producer-plan scaffold
are removed. Public bounded codec checks are still available at external record boundaries.

Consumer dependency captures come from the prepared inventory, using native path/byte/digest
selection and closure bounds. Equal content does not erase independent acquisition origins. The
physical reader reserves its exact buffer, verifies the admitted bytes, and retains its execution
permit and capsule locks through blocking exit. Cancelling the waiting future cannot authorize
input deletion or terminal cleanup. This source mechanism still requires the deferred complete
producer, restart, retention and installed qualification in Plan 19 CP12.


### Shared preparation and producer provenance — 2026-09-18

Producer attempts and semantic runs share one field declaration. Native run composition preserves
qualified acquisitions separately from canonical input meaning and applies the same clocks, logs and
closure rules at control, ingest and attempt admission. Local build configuration identity uses the
native record codec, separate from its stored artifact digest.

Cargo source admission, package/version/library identity, configuration removal, options and consumer
manifest generation are native plans over bounded, segment-valued TOML facts. Python distribution
selection, import mapping, metadata authority, interpreter requirements and wheel-tag agreement share
one native owner between acquisition and execution preparation. Narrow format UDFs decode syntax;
native expressions and set operators decide meaning. Preparation readiness binds the declared argv,
observed producer identity, physical exit and confirmed cleanup. It never grants execution authority.

Physical archive/read callbacks retain cleanup and storage ownership through exit. Exact file buffers
reserve native-pool bytes before allocation; captured metadata text retains a separate reservation.
These charges cover the declared buffers, not allocator RSS or complete parser/worker/kernel memory.
The former unbounded blob read and duplicate Cargo/Python policy implementations are removed.

Semantic admission runs before coercion, after analysis and in the shared native planner after
optimization. Physical traversal checks all child schemas and executable invariants. The complete
operator/mutation/physical-expression and actual worker/storage/client matrices remain unqualified
until Plan 19 CP11 permits terminal integration.


### Explicit immutable bundle and native export policy — 2026-09-18

`bundle_plan` selects exact checksum/file coverage, canonical ordering, catalog scope and the
complete deduplicated blob copy inventory. Physical capture/hashing and copying remain bounded
owned I/O. Live-source export enrolls exact durable artifact dependencies and current source roots;
its blocking copies retain those leases and primary staging ownership through physical exit.

After target writers and releases drain, an exclusive fence seals the target data root. The seal
belongs to the bundle's checksum inventory and `delta-evidence-bundle/2` contract. `ImmutableRoot`
is required by immutable control/evidence opening; the exact vector and seal survive native
provider/plan/stream retention. Normal blob and Delta mutation entry points reject sealed roots,
including already opened writers. Foreign table namespaces are refused before Delta mutation.
Root-lock-only captured-read and generic read-only repository APIs are removed. Control inspection
can read its own records but cannot grant dependent artifact/result reads. Bundles remain bytewise
unchanged by verification. Version 1 has no compatibility route.

Native/physical-mechanism units pass. Complete export/storage/crash/lifetime qualification remains
Plan 19 CP12 after the deletion barrier; this is implemented source, not activated acceptance.


### Shared positive storage and evidence proof cache — 2026-09-18

The runtime's native positive-contract cache also owns successful file, snapshot and attempt
validation proofs. It replaces three repository-local map/threshold implementations and uses
one existing byte budget. Proof keys include physical file/table incarnation, exact descriptor
and catalog identity, runtime meaning and effective validation limits as applicable. Current
read authority stays outside the cache. Warm attempt proofs still validate their physical bytes.
Authority invalidation clears the bounded proof cache; no inventory is copied. Proof/log-vector
reservations survive eviction while a caller owns the proof. Occupancy remains declared native
accounting, separate from allocator/RSS and unfinished external-allocation qualification.


### Native retained validation and owned reader descendants — 2026-09-18

`execution_documents` selects retained payload/document closure and coordinates in native plans;
`semantic_utf8_range` is the bounded syntax kernel. `snapshot_validation` composes native semantic
component, coverage, count and input-size admission. Exact artifact protection travels into hash,
document and retained-result readers; positive proof reuse does not replace it. Contract-verification
providers additionally retain their namespace flight and shared snapshot reservation.

Provider planning/execution and stream polling compose retention with the current physical input
scope. DataFusion async/blocking descendants inherit the owners through its existing task hooks,
including after cancellation of the caller or stream. The shared local object store captures file
ranges into pool-admitted `OwnedBytes`; `Bytes::from_owner` carries reservations through clones and
slices. File metadata/preconditions remain the upstream implementation, invoked on the I/O lane;
owned byte reads and synchronization use that lane's blocking pool. A full-file get is one lazy
admitted buffer, while selective range reads remain selective. Decoded metadata, predicate and
writer allocations are separate unfinished accounting work; none of these figures claims RSS.

### Native assembled results and paid acquisition/Arrow outputs — 2026-09-18

Overview namespace, child and kind-count assembly is relational, including ordering, typed empty
values and paired map entries. The two declared overview materializations retain their shared native
base. A materialization is a logical source whose binding seals the admitted input and session;
its extension planner prepares the physical child without executing it. Consumer optimization
cannot rewrite the captured base. See proposed ADR-0052; complete CF qualification remains open.

Producer binding, qualified input receipt selection and publication fact provenance share one native
owner. Receipt-vector order never resolves conflicting semantic descriptors. Acquisition summaries,
coverage, gaps and ordered handles use one native presentation plan across Rust, Python and revision
inputs. Hosted fallback honors explicit request settings and preserves missing-versus-empty values.
All selected retained result roots use one bounded native dependency union for validation and export.

HTTP response/cache bodies and artifact byte ranges use shared paid bytes. Incremental input growth
reserves both old and replacement capacities before allocating; byte clones/slices share the owner.
Exact artifact leases follow cache/Rustdoc reads through cancelled blocking work. Arrow's `pool`
feature and DataFusion execution's `arrow_buffer_pool` feature provide buffer-level ownership at
native result/fold/cache-output handoff. The adapter prepays the native buffer traversal before
splitting reservations into Arrow's shared buffers. Arrow aliases replace their prior claim, and
retained buffers remain charged after their stream is dropped. This does not bound every upstream
allocation: decoder, metadata/predicate, kernel/writer and external process/transport workspace
remain separately governed and require completion/qualification. No pool figure is an RSS claim.


Source metadata policy is native: typed PyPI/revision facts feed release identity/metadata,
single distinct import-root binding and commit/tree admission. Unknown external source fields
remain available in immutable raw artifacts. `control/admission.rs` owns publication replay,
selection predecessor conflicts, claim scope and transaction-key selection. The physical control
writer interprets its finite disposition and appends admitted Arrow rows. Unknown acknowledgement
reconciliation refreshes the same incarnation and uses complete semantic row fingerprints with
native occurrence counts. It does not infer success from a cache hit or transaction-marker TTL,
and a failed reconciliation cannot initiate a write retry. Actual fault/restart proof remains CP12.


The shared finite Arrow ingress is an immutable captured-batch provider with shared buffer claims.
It exposes no mutable MemTable handle and makes no unproved relational-constraint claim. Comparison
and search consume this same boundary; their real native reconciliation/ranking/count/page consumers
now have isolated Arrow/cache evidence. Comparison registers its typed alternative relations once and
uses one changed-key materialization for every consumer. Native input mutability refusal remains
part of cache admission. Installed, storage and retention qualification remain separate CP12 work.

Field declarations now also own relation-reference targets and scope keys. Evidence/control native
anti-joins, rule discovery and semantic manifests consume that one declaration. Intrinsic admission
uses one parent-aware compiler; presentation roles are descriptive only. Type vocabularies and
contextual field rules cannot overwrite each other. Encoded Arrow values use native decoding/union
selection with full-field projections, preserving semantic children through admission. Set fields
declare uniqueness in both native admission and generated wire schemas; sequences preserve repeats.
These source capabilities are distinct from the deferred complete provider/storage/client matrices.


### Typed immutable definitions and retention selection — 2026-09-18

> Decision: ADR-0047. Native field declarations own identity domains and their storage contracts.

Eight sealed definition families bind their payload, native key/domain, Delta table and identity
column once. `Definitions<D>` materializes one admitted Arrow record and returns a typed binary
identity with its exact binding; policy and execution consumers retain that domain. Policy
retention is a specialization of this owner, with no separate implementation or string reader.

Retention row selection is a tagged native value, with explicit text only for declared text
columns. One compiler admits its complete semantic Field before generating equality predicates.
Only the admitted Delta representation boundary lowers FixedSizeBinary identities to Binary.
The native analyzer verifies collection composition and overlap fields before allowing DataFusion
set/concat/overlap operations on nested identity-bearing dependencies; structural emptiness uses
the concrete native ArrayEmpty implementation. No private collection evaluator is introduced.

Source epochs are state 24, snapshot 14.0, wire 10.0 and executor 9. The decoder subprocess
protocol remains native-rustdoc-arrow/4. Scoped units and compile/schema checks establish the
implemented boundary; storage, restart, reclamation and client qualification remain CP12 work.


### Typed physical ownership and native cleanup — 2026-09-18

> Decision: ADR-0047. Native declarations own lifecycle identities and cleanup scope.

Retention leases, cleanup obligations, maintenance runs, private directories, physical execution
owners and storage reservations each carry a distinct FixedSizeBinary(16) identity. The existing
retention-policy native key produces a FixedSizeBinary(32) RetentionPolicyId over the captured
inline policy. No extra policy table or cache is introduced. Lifecycle labels are descriptive;
selection uses typed identity, exact process witnesses, immutable scope and monotone state.

A private-directory dependency is one declared union: local ID plus purpose, or export ID plus
observed parent path/device/inode. DataFusion joins group owners by directory ID and refuse
conflicting scope, malformed cardinality, live readers and retired-identity resurrection. Child
processes retain separate obligations for the same exact directory. Directory purpose, export
location and cleanup authority are not inferred from labels, path prefixes or dependency positions.
Different lifecycle domains are rendered through the shared native diagnostic projection before
combining removal blockers.

Container supervision and durable physical-owner observations keep the typed owner ID through
exit and recovery. Broker names are derived only when constructing broker arguments or operation
filenames. Quarantine leaves are derived from StorageReservationId; recovery captures canonical
components under the observed owned cache and uses a native reservation/inventory selection.
Redundant stored child/quarantine strings and the old physical-owner/external-directory dependency
variants are deleted. Mechanical filesystem checks and remove/sync-before-release ordering remain.

Source epochs are state 25, snapshot 15.0 and wire 11.0. Executor 9 and decoder
native-rustdoc-arrow/4 remain unchanged. This bounded source slice does not close Plan 19's CP11
deletion barrier or qualify its deferred storage, restart, publication and client journeys.
