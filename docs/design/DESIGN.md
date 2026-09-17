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

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0037 — Research/2.0 replaces the prior root pagination and nested terminal-result contract.

Research/2.0 retains explicit outcome, scope, freshness and source evidence while separating
`delivery` from the native outcome. `ok` means success within the declared scope. Coverage
assessments distinguish indexed, partial, missing and unknown; indexed-empty is meaningful only
within its evaluated domain. Each requested aspect and alternative set has its own page/cursor.
There is no root pagination field. Errors carry a typed cause, stage, rule, affected identities,
limits and recovery actions. Nullable root fields remain required even when their value is null.

`contracts/research-v2/research-envelope.schema.json` is the current frozen candidate generated
from Rust. The original contract remains preserved provenance and is not a compatibility reader.
Authored Pydantic presentation models use generated domain types and validate actual inline,
artifact, pending and error outputs. Individual comparison values are tagged inline/artifact;
artifact variants retain complete JSON, digest, size and original alternative FactSource.
Comparison values remain native Arrow cells until a bounded final sink selects inline JSON or
streams a complete artifact. Indexed result dependency closure is verified and included in
catalog admission/export, with descriptors for offline reads of presentation-only artifacts.

*Evidence: Tested for the cases recorded in the Plan 13 ledger; final client and deployment
qualification remains open.*

### 7.3 Output budgets

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0045 — Plan 15 replaces the previous execution/storage mechanism; see §16.

> Decision: ADR-0036 and ADR-0037 — Independent aspect/alternative pages and complete indexed artifact delivery preserve scope and native outcome.

> Decision: ADR-0029 — Distinguish selected API projections from complete retained observations.

> Decision: ADR-0030 — Admit complete delivery artifacts before committing successful jobs.
> Decision: ADR-0032 — Configure producer capacity and qualify observed resource limits.

> Decision: ADR-0024 — keyset cursors bind semantic query/snapshot/sort versions; complete envelope and execution budgets are separate obligations.

The core enforces `max_bytes` against the complete service envelope. MCP adds one compact text
summary (at most 160 JSON-encoded UTF-8 bytes) and at most 1024 bytes of standard protocol framing
allowance, measured on actual stdio responses. The canonical envelope appears once in structured
content; tools advertise its generated schema. Client SDK coercion is not wire JSON.

> Decision: ADR-0014 ([bounded evidence comparison](../adr/0014-bounded-comparison.md)).

Configurable initial limits, explicitly **not measured performance claims**: a 12 KiB inline
response budget, 12 search results per page, an 800-character excerpt, an 80-line source
excerpt, a 2-second inline wait then a job receipt, Rust-configured producer concurrency and warm-session capacity. Machine JSON is never truncated mid-object — fewer complete entries are returned
with a cursor or an artifact handle, and mandatory coverage and error fields are preserved.
Cursors bind the query digest, snapshot, sort and position; a mismatched cursor is rejected.
blueprint §7.3.

*Evidence: Implemented — budgets, pagination cursors bound to the query digest and snapshot,
and excerpt limits are enforced on the search and inspect paths. The limits are configured
defaults, **not measured performance claims**.*

### 7.4 FastMCP implementation

> Decision: ADR-0047 — Plan 17 generates semantic, storage/read and wire contracts from one native declaration; see §17.

> Decision: ADR-0037 — FastMCP 4.0.3 validates original inputs and emitted tool variants; optional progress cannot replace a completed outcome.

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

One configured multi-thread Tokio executor runs native startup, request dispatch, planning and
Delta metadata/write operations. Worker count follows native concurrency; blocking-thread count and
stack size are explicit validated settings, reported separately from Arrow memory. Tasks and running
blocking callbacks retain the executor owner and admission until physical exit. Futures are boxed
before task-local/tracing composition to avoid large moves on caller stacks. Background shutdown is
a drop fallback, not evidence of graceful effect cleanup.

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


### Native contained process definitions (Plan 15, 2026-09-16)

Policy configuration, executor operations and process effects use one immutable Delta definition
implementation, with separate finite tables and exact table/version/contract references. Core
Arrow contracts and native SHA expressions replace executor operation and image/containment JSON
identities. Protocol version 3 carries the new operation identity; no old protocol fallback exists.
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

These changes are source implementation. Real container/worker, warm-session/revocation and
installed-client qualification, full derived-environment linkage and retention enrollment remain
required. Native diagnostic state is implemented as described below; final lifecycle and retention
qualification remains open.


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
