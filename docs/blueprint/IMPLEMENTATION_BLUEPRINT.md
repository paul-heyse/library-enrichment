# Library Enrichment Service — Implementation Blueprint

**Version:** 1.0  
**Prepared:** 2026-09-13  
**Status:** Implementation specification; not an implemented or tested server.  
**Audience:** A capable programming agent building the complete system.  
**Repository name:** `library-enrichment`  
**Agent-facing MCP server name:** `library-enrichment`  
**Companion skill:** `library-research`

## 1. Mission and deliverable

Build a standalone, local-first library-evidence service for Rust and Python. It complements Context7 by collecting exact-release package information, public API structure, documentation navigation, source/examples, semantic observations, API/release changes, and narrowly scoped verification results. Expose these capabilities through **FastMCP 4**. Deliver an accompanying skill that coordinates Context7 and this service without requiring agents to read or maintain comprehensive library-reference documents.

The service produces **evidence**. The calling agent produces **architectural judgments**. A future capability graph may consume the evidence, but implementing that graph is not a prerequisite or part of this repository's initial scope.

The operational question is not simply “What methods exist?” It is:

> Given a design objective and a concrete dependency environment, what relevant library capabilities exist, what evidence supports their use, what configuration do they require, and what remains unverified?

### 1.1 Binding decisions

| Decision | Required implementation |
|---|---|
| Repository boundary | One standalone repository; no service code, environments, caches, or generated indexes inside working repositories. |
| Core ownership | Rust owns identities, package resolution, fetching, normalization, evidence storage, querying, job state, policy, and publication. |
| Python boundary | A thin FastMCP adapter; a separate Griffe extraction worker; isolated Python runtime probes. Do not reimplement the core in Python. |
| Python semantic engine | Astral **ty**, not Pyrefly. Use its CLI/LSP boundary, not private Rust internals. |
| Rust semantic engine | rust-analyzer through LSP, isolated behind a producer adapter. |
| Context7 | A separate MCP connection used directly by the calling agent. No Context7 proxy, embedded LLM, or recursive research agent in this service. |
| Storage | Immutable raw artifacts and Arrow/Parquet evidence snapshots; Rust DataFusion for structured retrieval. This is an evidence cache, not a replacement for the future graph/fact store. |
| MCP transport | Per-client stdio adapters attached to one local Rust daemon. HTTP is an optional later deployment profile. |
| Design inference | Belongs in the calling agent's evidence-backed research brief, never silently promoted to extracted fact. |
| Completion | Real Rust and Python fixtures must work end to end, including tests of incomplete evidence and failure paths. |

**Do not add** a graph database, embeddings, vector database, GPU workload, generic workflow engine, raw SQL MCP tool, arbitrary shell MCP tool, full-repository code-property graph, or automatic project edits to the initial implementation.

### 1.2 Why these boundaries

Context7 exposes library resolution and documentation queries; it is not the authority for a project's actual resolved dependencies. Its current tool names are `resolve-library-id` and `query-docs`, but clients may namespace them. Discover the actual available tool definitions rather than hardcoding a client prefix. [S21]

FastMCP 4 supports typed tools, resources, and current protocol negotiation. Its documentation recommends exact dependency pins. Use the standalone `fastmcp` package and `from fastmcp import FastMCP`, not the older SDK-bundled import. Pin a tested v4 release and commit the lockfile. [S01, S02, S04]

## 2. System topology and ownership

```text
Coding agent inside any working repository
    |
    +--> Context7 MCP -----------------> concepts and documentation examples
    |
    +--> library-research skill --------> routing, evidence policy, synthesis
    |
    +--> FastMCP 4 stdio adapter
                    |
                    | typed, bounded local RPC
                    v
             Rust enrichment daemon
                    |
                    +--> package/document fetchers
                    +--> Rust API producer
                    +--> Python API producer (Griffe worker)
                    +--> LSP session manager (rust-analyzer / ty)
                    +--> isolated verification workers
                    |
                    v
             Raw artifact cache + immutable evidence snapshots
                    |
                    +--> Arrow/Parquet + DataFusion queries
                    +--> bounded MCP responses and artifact resources
                    +--> future graph-compatible export
```

### 2.1 Processes

**`library-enrichmentd`** is the single writer and job owner. It must support multiple simultaneous agents without duplicate builds, conflicting publications, or a language-server process per tool call.

**`library-enrichment-mcp`** is a small Python entry point installed in this repository's environment. Each client may start its own adapter. The adapter validates inputs, calls the daemon, emits MCP results, and maps structured errors. It does not index libraries or own persistent state.

**`library-enrichment-worker`** is a Python worker entry point for approved Griffe or runtime-inspection tasks. The daemon launches it with explicit inputs and a restricted environment. It emits schema-versioned JSON records. It cannot publish snapshots.

Use a Unix-domain socket on Linux and macOS for daemon RPC. For v1, use bounded newline-delimited JSON-RPC 2.0, with embedded newlines JSON-escaped and an explicit message size limit. Large payloads travel by content-addressed artifact handles, not inline RPC messages. Do not confuse this internal protocol with MCP or LSP framing. A future Windows transport can implement the same RPC contract.

Avoid adding FastAPI, gRPC, Redis, or an additional daemon solely to carry this small local interface. Future scaling must not determine the initial topology.

### 2.2 Lifecycle

Provide explicit `start`, `status`, and `stop` CLI commands. The adapter may auto-start the daemon when permitted by configuration; use an OS lock to avoid concurrent startup races. The daemon survives adapter exits and retains jobs. An adapter disconnect does not implicitly cancel a shared job.

Normal stdio startup must register the tool catalog promptly. Do not initialize language servers, download packages, run compilation, or refresh registries during MCP initialization. FastMCP stdio processes are client-managed, which is why heavyweight shared state belongs outside them. [S05]

### 2.3 Filesystem layout

```text
~/src/library-enrichment/                 # implementation only
    Cargo.toml
    Cargo.lock
    rust-toolchain.toml
    pyproject.toml
    uv.lock
    crates/
      enrichment-core/                   # identities, DTOs, plans, adapters
      enrichment-store/                  # artifacts, manifests, Arrow/DataFusion
      enrichment-daemon/                 # RPC, workers, scheduling, CLI
    python/
      enrichment_mcp/                    # thin adapter, generated DTOs
      enrichment_worker/                 # Griffe and runtime probes
    schemas/generated/                   # wire contracts and Arrow schema specs
    skills/library-research/
      SKILL.md
      references/
    config/
    scripts/
    tests/{unit,contract,integration,e2e,fixtures}/
    docs/{architecture,operations,adr}/

~/.cache/library-enrichment/              # regenerable data
    downloads/
    unpacked/
    capsules/                            # service-owned Cargo/Python environments
    lsp/

~/.local/share/library-enrichment/        # retained evidence and job records
    blobs/sha256/
    snapshots/
    contexts/
    jobs/
    bundles/
    logs/
```

The implementation must use an OS-aware directory resolver with overrides rather than assume these Linux paths on every platform. Keep the socket path short and private to the user.

The **working repository is never a subprocess working directory or an extraction destination**. The coding agent reads its manifests and lockfile using its normal repository tools and supplies a dependency context. An optional local import CLI may copy allowlisted manifests/lockfiles into service storage; it must not execute project tooling in place or follow arbitrary symlinks. Do not infer workspace access from the MCP client's current directory or require MCP roots callbacks.

## 3. Research identity, versioning, and freshness

### 3.1 Four identities

**`release_id`** identifies the actual library release/source: ecosystem, registry, normalized package name, exact version or immutable Git revision, and selected artifact digest. Python distribution names and import names are different fields. Rust package names, crate names, and re-export paths are different fields.

**`environment_id`** identifies the declared or resolved environment. Rust includes toolchain, target, feature configuration, default-feature setting, dependency graph/lock digest, and relevant compiler configuration. Python includes implementation/version, platform/ABI, extras, dependency-lock digest, and stub providers.

**`context_id`** binds a release to an environment and a research mode. It must say whether that environment is `unspecified`, `declared`, `resolved`, or `verified`. If later evidence resolves previously unknown environment fields, return a new derived context; do not mutate the meaning of an old ID.

**`snapshot_id`** identifies an immutable set of evidence available for a context. Enrichment creates a new manifest; it does not overwrite the old snapshot. Tool calls may omit a snapshot to use the current published one, but every result names the snapshot actually read. Reproducible calls pass a snapshot explicitly.

Do not use rustdoc's local item IDs as cross-release global IDs. Maintain canonical package-qualified public paths, definition identities, and alias/re-export edges. A source line number alone is not stable identity.

### 3.2 Research modes

| Mode | Meaning |
|---|---|
| `project` | Investigate the exact dependency environment supplied by the agent. Never silently upgrade it. |
| `upstream` | Resolve the newest eligible published release as of a recorded registry check. Exclude prereleases and yanked releases unless explicitly requested. |
| `compare` | Compare two exact resolved contexts; show environment/configuration differences rather than treating them as library changes. |
| `revision` | Investigate a pinned development revision. Keep it distinct from a released package and from mutable branch names. |

A project's pinned version can be correctly researched even when it is old. Conversely, a recently fetched `/latest/` documentation page can describe the wrong dependency version.

### 3.3 Freshness is not one timestamp

Record at least `retrieved_at`, `registry_checked_at`, `source_revision`, `artifact_digest`, `producer_version`, and `documentation_version_match`. For remote documents also retain HTTP validators where supplied. Distinguish `exact`, `compatible_claimed`, `mismatched`, and `unknown` version matching.

Implement `freshness` options: `cache_ok`, `revalidate`, and `offline`. For requests explicitly asking for “latest”, perform a registry revalidation or return a clearly marked inability to verify latest. Immutable artifacts are reused by digest. Mutable URLs, inventory files, and registry pointers receive conditional revalidation and a finite configurable TTL.

A Context7 result with an uncertain source version is a discovery lead, not exact-version proof. Even docs.rs JSON can be rebuilt with a newer rustdoc format: capture both crate version and producer format. [S07]

## 4. Evidence acquisition: Rust

### 4.1 Preferred path

```text
Exact release + target/features
    -> registry/package metadata
    -> docs.rs build metadata and hosted rustdoc JSON
    -> normalize public API and re-exports
    -> overview / exact-symbol search / public-API diff
    -> targeted docs, examples, release notes
    -> rust-analyzer or compile probe only as needed
```

**Download hosted rustdoc JSON before compiling.** docs.rs provides compressed structured documentation; some releases or builds may be missing, and JSON formats vary. Inspect `format_version` and use a compatible parser adapter. An illustrative endpoint pattern is:

```text
https://docs.rs/crate/{crate}/{exact-version}/json
```

Use the documented target-specific form when necessary. Handle redirects, compression, missing artifacts, and unsupported formats explicitly. A format-specific download is useful only when docs.rs actually has that format. [S07]

### 4.2 Producers and outputs

| Producer | Required output |
|---|---|
| Registry/manifest reader | Exact version, repository/doc links, crate target names, feature definitions, release metadata, checksums where available. |
| `cargo metadata` in a service capsule | Resolved package graph, target metadata, activated features and dependency identities for the selected invocation. |
| Hosted/local rustdoc JSON | Items, signatures, public paths, docs, impls, traits, bounds, associated items, deprecations, links, raw attributes where present. |
| `public-api` library / `cargo-public-api` adapter | Simplified public API representation and version delta; omit noisy generated impl classes in the default presentation, not from retained evidence. |
| Targeted documentation/source reader | Definition bodies, feature documentation, examples, tests, changelog sections, migration guidance. |
| rust-analyzer LSP | Contextual definitions, hover/type information, references, and supported relationships with analysis scope recorded. |

Cargo metadata reports machine-readable package and resolution information, but its result is invocation-dependent. Match the project target/features rather than defaulting to `--all-features`. [S09]

The `public-api` library can consume rustdoc JSON directly. Prefer that path to avoid unnecessary builds; the CLI is a diagnostic/interoperability adapter, not the canonical data model. Keep its parser compatibility coupled to the rustdoc producer format. [S10]

### 4.3 Documentation build is not project configuration

docs.rs allows maintainers to set features, default-feature behavior, all-features, targets, and rustdoc arguments. Treat the hosted result as evidence for that documented configuration, not as the API definitely enabled in the working project. [S08]

Record `observed_configuration` separately from `requested_configuration`. A symbol can be `documented_available` but `project_availability_unverified`. Do not invent a feature predicate for every symbol: conditional compilation may remove inactive branches before extraction, and documentation annotations are not a complete Boolean model of feature availability.

Never blindly compile all feature combinations. Some feature combinations conflict, depend on platform libraries, or are deliberately mutually exclusive. Start with the project's exact configuration; explore explicitly named additional profiles only when relevant.

### 4.4 Local fallback build

When hosted JSON is absent, unsupported, or not appropriate for the requested configuration, build in a service-owned capsule using a pinned compatible nightly toolchain. A producer must derive its command from typed options, not concatenate a shell command supplied by the agent.

The required shape is `cargo +<pinned-nightly> rustdoc --manifest-path <capsule-manifest> --lib ... -- -Z unstable-options --output-format json`; include locked dependencies, the selected target, and explicit feature/default-feature settings. Use the crate's actual source/manifests or a carefully constructed consumer capsule according to the question. A fabricated dependency-only capsule is not automatically equivalent to a whole project's workspace feature unification. The unstable rustdoc output flags must be checked against the pinned nightly. [S26]

Record provenance as `locally_built_rustdoc` and retain logs. Nightly-build success is not proof that the same API compiles on the project's stable compiler. A separate usage check with the target project toolchain establishes that narrower claim.

Cargo builds can execute build scripts and procedural macros. rust-analyzer also enables build-script and proc-macro activity through its configuration. These operations belong in execution policy and sandboxing, not in the supposedly non-executing metadata tier. [S11]

### 4.5 Discovery beyond API additions

Capability discovery must also inspect the crate/module overview, feature descriptions, examples, migration notes, and relevant release notes. Optimizer changes, new configuration behavior, SQL capabilities, and performance improvements can occur without new Rust public symbols.

Do not promise every example/test is present in a published crate archive. When an exact source revision is needed, fetch it and record its relation to the release. If the version-to-commit relation cannot be verified, mark it as approximate instead of treating the default branch as released behavior.

## 5. Evidence acquisition: Python

### 5.1 Preferred path

```text
Exact distribution + interpreter/platform/extras
    -> PyPI metadata and suitable wheel/source artifacts
    -> package/import mapping, metadata, inline types and stubs
    -> Griffe static API snapshot
    -> objects.inv and targeted official documentation
    -> ty LSP/typecheck in a synthetic consumer capsule
    -> isolated runtime inspection or usage probe only when needed
```

Use PyPI's release JSON and Simple API to inspect releases and distribution metadata. Artifact choice must respect Python requirements, wheel compatibility, yanked status, and requested prerelease policy. A package supporting a Python version does not guarantee a wheel for every platform. [S14, S15]

### 5.2 Producers and outputs

| Producer | Required output |
|---|---|
| Registry/distribution reader | Distribution identity, artifact hashes, Python/platform compatibility, dependencies, extras, entry points, project links. |
| Wheel/source inspector | Import roots, `.py`/`.pyi` files, `py.typed`, package structure, declared exports, source availability. |
| Griffe worker | Modules, classes, functions, signatures, annotations, overloads, docstrings, imports/re-exports, aliases, inheritance, extraction gaps. |
| Documentation inventory reader | Documented objects/labels and canonical target pages; exact inventory provenance. |
| ty process | LSP evidence and type-check diagnostics in the actual configured consumer environment. |
| Runtime worker | Restricted `inspect` observations and small verification probes, separately labeled as executed evidence. |

Default Griffe configuration must include explicit `search_paths` and `allow_inspection=False`. Without the latter, Griffe may import packages when static sources are unavailable. Preserve unresolved aliases rather than guessing them. [S12]

Do not treat Griffe's breaking-change checker as a complete feature-addition detector. Implement set/field differences over your normalized snapshots; use Griffe's compatibility checker as supplemental interpretation for supported breaking-change classes. [S13]

### 5.3 Types and dynamic APIs

Track the provenance and supported runtime version of stub packages. Stubs, inline annotations, and runtime objects can disagree; store separate observations and report conflicts. PEP 561-style type distribution includes inline types, stub-only packages, and partial stubs with specific resolution behavior. Let ty resolve those semantics instead of implementing an ad hoc imitation. [S16]

Python publicness is not a single Boolean. Store signals such as declared `__all__`, documented status, underscore convention, re-export pattern, and author declaration. A missing docstring or inventory entry does not prove an API is private or nonexistent.

`objects.inv` is a navigation/index signal, not a substitute for API extraction or a complete narrative. It can include more than Python functions and classes. Fetch relevant target sections after resolving entries; retain the source inventory version. [S17]

A native extension may have rich stubs but limited source; a dynamic package may generate members at runtime. Return `partial` with explicit missing evidence when static extraction cannot characterize it. Runtime `inspect` is an escalation, not a guarantee of complete signatures or source for every callable. [S20]

### 5.4 ty integration

Launch the pinned `ty server` executable and use negotiated LSP capabilities. For verification, use a pinned `ty check` invocation against a generated consumer capsule and its selected interpreter/dependencies. Do not type-check against the enrichment service's own virtual environment. [S19]

The current ty feature table supports definitions, hover, signatures, references, workspace symbols, call/type hierarchy, and diagnostics, but lists `textDocument/implementation` as unsupported. Probe capabilities at runtime and preserve unsupported outcomes rather than inventing answers. [S18]

Keep the Griffe worker interpreter separate from the analyzed interpreter. Install packages or build sdists only in a service-owned isolated environment. Do not execute a library's Sphinx configuration merely to read its docs, and do not enable unreviewed Griffe extensions automatically.

### 5.5 Supporting parsers and installed metadata

Use `importlib.metadata` only against a service-owned selected environment when installed distribution metadata is useful; retain the environment identity and do not import the target library just to read package metadata. Prefer static distribution metadata when no environment is necessary.

Ruff can lint the service's Python code. Do not make Ruff's internal parser/semantic crates, tree-sitter, or a custom source-fact extractor prerequisites for v1. A targeted syntax adapter is an optional escalation when normalized API data and source spans cannot answer a concrete question. It must emit the same evidence records, not start a parallel fact store.

## 6. Canonical evidence model

Use a small Rust-owned typed model, not a large ontology. Stable enums and typed relationships should cover observed evidence; free text is for excerpts and explanations, not for essential machine state.

### 6.1 Required record types

| Record | Core fields |
|---|---|
| `Release` | `release_id`, ecosystem, registry, package, exact version/revision, distribution/crate names, artifact identities. |
| `Environment` | `environment_id`, language/runtime/toolchain, target/platform, feature/extras selection, resolved dependency digest, resolution status. |
| `Context` | `context_id`, release/environment IDs, research mode, supplied constraints, parent context if derived. |
| `Artifact` | `artifact_id`, SHA-256, MIME type, size, source URI/revision, retrieval/validation metadata, license/provenance hints. |
| `ProducerRun` | Producer name/version/config digest, inputs, execution policy, timestamps, exit status, raw output/log IDs, gaps. |
| `Symbol` | Context, qualified public path, definition identity, kind, normalized signature, publicness signals, observed availability, evidence IDs. |
| `Relationship` | Typed source/target IDs, relation (`reexports`, `implements`, `inherits`, `returns`, `accepts`, `documents`), producer/evidence IDs. |
| `EvidenceFragment` | Kind, artifact locator, bounded extract, subject IDs, source/version match, producer run, epistemic class. |
| `Change` | Before/after snapshot IDs, change kind, symbol/doc/config identity, evidence IDs, compatibility interpretation. |
| `Verification` | Snippet digest, full environment, command recipe, execution policy, assertions, diagnostics, outcome, scope limits. |
| `SnapshotManifest` | Schema/normalizer version, input digests, producer runs, table/blob refs, coverage, publication timestamp. |
| `Job` | Job ID, request key, state, progress events, producer plan, result handle, error, cancellation/refcount information. |

### 6.2 Separate epistemic classes

Use `declared`, `statically_extracted`, `compiler_derived`, `typechecker_observed`, `runtime_observed`, and `agent_inferred`. These are evidence categories, not universal confidence scores.

An API signature can be compiler-derived while the claim “this will eliminate our custom orchestration” is agent-inferred. A passing probe establishes only its tested assertions in its recorded environment. It does not establish universal safety, performance, or architectural superiority.

### 6.3 Schema ownership

Rust DTOs and the explicit canonical table model are authoritative. Generate versioned JSON Schemas from the Rust wire types; generate the small Pydantic boundary DTOs from those schemas where supported. Declare Arrow projections in the Rust schema registry and test semantic round trips. Do not hand-maintain unrelated Python/Rust/Arrow definitions with no consistency checks.

Preserve upstream raw JSON in content-addressed blobs so normalization can be improved without refetching or rebuilding. Persist upstream IDs as local producer identifiers, not as globally meaningful symbols.

Use schema/normalizer versions in content keys. Exclude wall-clock timestamps and temporary paths from semantic content hashes; retain them in provenance. A consumer can then distinguish identical facts retrieved at different times from actually changed evidence.

### 6.4 Claim-oriented source priority

There is no universal “best source” ordering. For an exact signature, prefer the selected artifact/compiler or relevant stubs. For supported usage, prefer version-matched author documentation and examples. For runtime behavior, use source plus targeted execution. For architectural fit, preserve the calling agent's reasoning and explicitly linked premises.

Contradictions are retained as conflicting observations. Never silently overwrite a runtime observation with a stub annotation or a project-configuration result with docs.rs' expanded build.

## 7. MCP tool contracts

Expose **nine tools**, with the same ecosystem-neutral vocabulary for Rust and Python. These names and contracts are proposed interfaces to implement, not existing FastMCP built-ins.

| Tool | Purpose | Principal inputs | Required result |
|---|---|---|---|
| `resolve_library` | Establish exact identity and environment before research. | Ecosystem, package, research mode, version/revision or constraints, environment, freshness. | Release/context IDs, exact artifacts, environment resolution status, initial metadata snapshot, upstream check status, ambiguity/errors. |
| `library_overview` | Discover unfamiliar capabilities without knowing symbol names. | Context/snapshot, requested sections, budget. | Faceted modules/types/traits or classes, feature/extras map, documentation headings, example/release-note index, extraction coverage. |
| `search_evidence` | Search a bounded set of API, docs, examples, source, or release evidence. | Context/snapshot, query, evidence kinds, namespace/filters, cursor, budget. | Ranked matching fragments with source locators, exact version match, counts, truncation, next cursor. |
| `inspect_symbol` | Characterize a known candidate and its deployment requirements. | Context/snapshot, symbol reference, requested aspects, depth `api|source|lsp`, optional consumer snippet. | Signature, docs, relationships, configuration evidence, source/examples, semantic observations, gaps. |
| `compare_releases` | Discover additions/removals and non-API changes. | Before/after contexts and optional snapshots, scopes, budget. | API/doc/config/release deltas, environment differences, comparable/noncomparable status, evidence links. |
| `verify_usage` | Test a proposed invocation or composition in isolation. | Context, bounded snippet, mode `compile|typecheck|runtime`, explicit assertions/test intent, authorized execution profile. | Exact executed environment, assertion/diagnostic results, observed scope, logs, new derived context if needed. |
| `read_artifact` | Retrieve large result sections without flooding context. | Artifact URI, section/line range, cursor, budget. | Immutable content slice, locator, content digest, remaining range. |
| `job_control` | Observe or cancel an explicitly submitted long-running operation. | Job ID, operation `status|result|cancel`, optional bounded wait. | Job state, progress, next poll suggestion, result/error, cancellation acknowledgment. |
| `service_status` | Inspect readiness and capabilities, without indexing. | Optional producer/component filter. | Versions, schema compatibility, sandbox capabilities, installed producers, feature support, queue/cache health. |

### 7.1 Tool behavior requirements

**Resolution:** A distribution name may map to several import roots, or a source selector may be ambiguous. Return candidates and an actionable ambiguity; do not guess. Return unavailable requested versions honestly. The service must not interpret every lookup as an upgrade request.

**Overview:** Include a namespace/module tree and feature descriptions, not an undifferentiated list of thousands of symbols. Represent unindexed sections as unknown, not empty. Offer pointers for interesting areas. Inventory membership is evidence of existence/documentation, not an automatic importance ranking.

**Search:** Exact qualified names outrank fuzzy matches. After exact/prefix/path matching, search retained documentation text and structured facets. Start with deterministic lexical/token matching and recorded scoring factors. Do not require embeddings for v1. Filter by context and snapshot before ranking, and diversify near-duplicate re-exports. Report whether a search covered API-only, documentation-only, or additional sources.

**Inspection:** Default to public API plus a small relevant documentation section. Do not initialize LSP for a signature already present in normalized evidence. An `aspects` selection can include `signature`, `availability`, `relationships`, `documentation`, `examples`, `source`, and `semantics`. Source/LSP depth is explicit and policy-controlled.

**Comparison:** Compute additive as well as breaking changes. Compare signatures, exports, features/extras, meaningful doc changes, and release notes. Do not count HTML boilerplate or rustdoc-generated impl noise as feature changes. Surface changed producer formats/configurations that could explain a delta. A clean API diff does not mean no behavioral change.

**Verification:** A caller-provided execution profile can only select from profiles enabled in local configuration; it cannot grant itself permission. `compile` and `typecheck` do not mean “execute nothing”: Rust build machinery can run code. Runtime mode requires the runtime policy. Report `not_run` rather than a fabricated successful outcome when unavailable.

**Artifact reading:** Only resolve service-issued URI/ID handles. Never expose a general filesystem path reader. Paged reading must be available as a tool even when a client does not surface MCP resources.

### 7.2 Common response envelope

Every tool returns an object with a stable root schema:

```json
{
  "schema_version": "1.0",
  "request_id": "req_<opaque>",
  "status": "ok",
  "summary": "Public API evidence is available; project-specific compilation is not yet verified.",
  "context_id": "ctx_<opaque>",
  "snapshot_id": "snap_<opaque>",
  "data": {},
  "coverage": {
    "scope": "selected release and recorded documentation build",
    "indexed": ["metadata", "public_api"],
    "missing": ["project_compile_probe"],
    "limitations": ["Hosted documentation features may differ from the project."]
  },
  "freshness": {
    "registry_checked_at": null,
    "source_version_match": "exact",
    "latest_verified": false
  },
  "evidence": [],
  "artifacts": [],
  "pagination": {
    "returned": 0,
    "total_matches": null,
    "truncated": false,
    "next_cursor": null
  },
  "job": null,
  "error": null
}
```

`status` is `ok`, `partial`, `pending`, or `error`. `ok` means successful within the declared scope, not complete knowledge of a library. A valid empty search is distinct from an extraction failure or a missing index. Use JSON Schema conditional validation so `pending` requires a job handle and `error` requires a typed error. Root nullability must be explicit for tools such as service status.

Each `evidence` entry must include an ID, evidence class, subject, exact artifact/source locator, source-version match, and producer. Return compact supporting excerpts, not just unexplained numeric confidence. Add build/runtime context where material.

Stable error codes must include `VERSION_NOT_FOUND`, `AMBIGUOUS_PACKAGE`, `ARTIFACT_UNAVAILABLE`, `UNSUPPORTED_FORMAT`, `ENVIRONMENT_UNRESOLVED`, `ENVIRONMENT_MISMATCH`, `POLICY_DENIED`, `UNSUPPORTED_CAPABILITY`, `UPSTREAM_UNAVAILABLE`, `EXTRACTION_FAILED`, `VERIFICATION_FAILED`, `BUDGET_EXCEEDED`, and `INVALID_CURSOR`. Retryability and a concrete next action belong in the error record.

### 7.3 Output budgets

Initial configurable limits—not measured performance claims:

| Limit | Default |
|---|---:|
| Inline serialized response budget | 12 KiB UTF-8, plus a small fixed envelope allowance |
| Search results per page | 12 |
| Excerpt per result | 800 characters |
| Source excerpt | 80 lines |
| Overview child entries per namespace | 20 |
| Verification snippet input | 32 KiB |
| Ordinary inline wait | 2 seconds, then a job receipt |
| Maximum `job_control` wait | 10 seconds |
| Concurrent expensive build/probe workers | 2 |
| Warm LSP sessions | 2, evicted by memory budget and idle policy |

Expose `max_items` and `max_bytes`, with server-enforced maxima. An optional `max_tokens` field is only advisory unless a client-specific tokenizer is configured; never claim a heuristic is an exact token count. Account for text and structured result duplication when measuring context cost.

Do not truncate machine JSON halfway through an object. Select fewer complete entries, preserve mandatory coverage/error fields, and provide a cursor or artifact handle. Pagination cursors bind the query/filter digest, snapshot, sort, and next position; reject mismatched cursors rather than producing inconsistent pages.

### 7.4 FastMCP implementation

Use typed Pydantic request/result models generated from or checked against the wire schemas. FastMCP can derive tool schemas and structured output from typed models; `ToolResult` permits control over summary text and structured data. Ensure the actual output matches the declared schema. [S02]

Prefer a small human-readable summary plus compact structured data. Do not return the entire same long document twice through content and structured content. Resource templates expose:

```text
library-evidence://contexts/{context_id}/overview
library-evidence://snapshots/{snapshot_id}/manifest
library-evidence://artifacts/{artifact_id}
library-evidence://jobs/{job_id}/result
```

Resource handlers delegate to the same core read operations as tools. Inputs are IDs, not paths. Large reads remain bounded and paged.

The default adapter does **not** depend on MCP background-task support. Expensive operations submit durable core jobs and return an ordinary `pending` envelope; `job_control` works with clients supporting ordinary tools. Native FastMCP tasks may be added later as an adapter over the same core job identities, never a second scheduler. In FastMCP 4, native tasks require the tasks extension/package and client protocol support. [S03]

Use tools/resources and a small optional prompt that returns the research workflow. Do not use an embedded LLM or assume server-side sampling/roots APIs from old FastMCP versions remain available; v4 changed those APIs. [S04]

Assign annotations deliberately. Cached reading/search is non-destructive and must advertise its actual external behavior. `verify_usage` must not be portrayed as a pure read-only operation. Cache creation alone is an internal service effect, but package execution and environment construction must be disclosed. All logs go to stderr or daemon logs, never MCP stdout.

## 8. Orchestration, caching, and long-running work

### 8.1 Producer plans, not bespoke scripts

Use typed `ProducerSpec` and `ProducerPlan` records: required input kinds, produced evidence kinds, execution class, supported ecosystems/formats, tool version, configuration digest, and resource limits. Implement a small explicit dependency graph for the known producers; do not build a generic orchestration platform.

Example Rust plan:

```text
resolve release
 -> fetch artifact + docs metadata
 -> obtain compatible rustdoc JSON
 -> normalize
 -> publish snapshot
 -> optional public API projection / targeted docs
```

Example Python plan:

```text
resolve release + suitable artifact
 -> extract wheel safely
 -> identify import roots and stubs
 -> Griffe static extraction
 -> normalize
 -> optional inventory fetch
 -> publish snapshot
```

A tool requests missing evidence kinds; the daemon plans the minimum additional producers. Work already completed for matching content/configuration is reused. When one source fails, retain usable evidence from other sources with a partial status.

### 8.2 Single-flight and publication

Deduplicate producer work by `(producer_version, normalized_options, input_digests, environment_id, policy_profile)`. One shared job may have several interested callers. Cancelling one caller's interest must not kill work still required by another.

Only the daemon publishes. Workers write staged outputs under job-specific paths. Validate schema, references, digest integrity, and expected coverage before publication. Publish an immutable snapshot directory and then atomically update the context's current pointer. Readers never observe partially written tables. Recover incomplete staging areas after crashes without advertising them as complete evidence.

### 8.3 Job state

Persist `queued`, `running`, `succeeded`, `partial`, `failed`, `cancel_requested`, and `cancelled`. A pending tool result includes `job_id`, progress stage, and an advisory next-poll delay, not a guarantee of completion time.

On daemon restart, unfinished jobs are recovered/retried only when their producer is idempotent and policy still permits it; otherwise mark them interrupted with an actionable error. Keep request IDs separate from reusable job/content keys.

Native tasks, a user service manager, or the daemon itself are implementation components. This blueprint does not imply any jobs have been scheduled or started by the assistant.

### 8.4 Cache invalidation

Immutable raw artifacts use digest keys. Normalized snapshots additionally key schema and producer versions. Search pages key snapshot/query/budget. Mutable registry/docs lookups use validators and finite TTLs. Negative cache entries must have shorter TTLs and preserve why an artifact was unavailable; a timeout is not a permanent absence.

Pin Arrow/DataFusion to a **verified mutually compatible dependency set**. Do not force unrelated major versions into one Rust type universe. No LanceDB dependency is required for this service; emit portable Arrow/Parquet/JSON for the future store. Keep external-library analysis dependencies separate from the service's own Cargo dependency graph.

## 9. LSP and verification implementation details

### 9.1 Synthetic consumer capsules

A capsule is a service-owned workspace containing exact dependency resolution inputs, a selected toolchain/interpreter, a small consumer file, and analysis settings. Its manifest records what was reproduced from the working project and what was not.

For Rust, include the selected crate source/dependencies, Cargo resolver context, target, feature/default-feature state, and a consumer snippet. For Python, create an isolated environment with exact selected wheels/dependencies/stubs and point ty at it. Capsules may use cached content, but their identity includes environment inputs.

A library API scan and a project-compatible consumer check are different operations. Do not label an API scan as a successful integration test.

### 9.2 LSP session manager

The daemon owns warm sessions keyed by language server/version and capsule digest. Implement correct LSP framing, initialization, capability negotiation, document open/change/close notifications, diagnostic collection, cancellation, and shutdown.

The MCP caller supplies a symbol ID or a small consumer snippet; it should not need to manufacture line/column coordinates. The service maps immutable symbol/source locations or generated probe anchors to LSP positions. Respect negotiated position encodings rather than assuming byte offsets match UTF-16 positions.

Store each observation with query, capsule identity, document digest/version, scope, and server version. A references result is limited to what the server indexed; it is not a whole-ecosystem call graph. An empty result, unsupported method, unresolved dependency, and incomplete indexing are distinct outcomes.

Use source/API evidence where available instead of implementing a crawler based on repeated LSP completions. Neither ty nor rust-analyzer needs to be embedded as an unstable Rust library in this repository.

### 9.3 Probe classes

**Rust compile probe:** `cargo check` or `cargo test --no-run` for a minimal consumer, using the relevant project compiler and lock/configuration. A runtime assertion uses a separate execution profile.

**Python typecheck probe:** Run ty against the consumer snippet with the recorded interpreter/stub environment. Type correctness is not runtime correctness.

**Runtime probe:** Execute bounded code with explicit test intent and assertions. Where inspecting API objects, prefer static attribute/member inspection when appropriate, but remember importing the package itself executes code. Preserve exceptions and missing native dependencies as evidence gaps rather than “the API does not exist”.

Do not auto-generate invented successful examples inside the daemon. The calling agent can propose a snippet; the service executes and reports. Store `snippet_origin=agent` and distinguish it from author examples.

## 10. Execution policy and source handling

Isolation is required primarily to keep working environments intact and research reproducible. It is not a separate enterprise-security project.

| Profile | Permitted work | Default |
|---|---|---|
| `static` | Registry/doc fetching, safe archive extraction, static API parsing, inventory parsing, searches. | Enabled |
| `build` | Rustdoc fallback, rust-analyzer build/proc-macro activity, Cargo checks, Python environment/build setup. | Enabled only when a configured sandbox is operational |
| `runtime` | Import/introspection and tests executing library code. | Explicitly enabled local profile |

On Linux, implement a rootless container or equivalently constrained worker profile with no home-directory mount, no working-repository mount, a non-root user, bounded CPU/memory/processes/time, and a dedicated scratch directory. Fetch dependencies separately; execution has no network by default. A macOS worker may use an available container runtime; absent isolation, provide static research and return `POLICY_DENIED` for execution rather than silently running on the host.

Do not make GPU access a default. Do not inherit API tokens, SSH agents, cloud credentials, the user's `PYTHONPATH`, arbitrary package-manager configuration, `.env` files, or project hooks. Explicitly allowlist needed environment variables.

Archive extraction rejects path traversal, absolute paths, symlink escapes, oversized decompression, and unexpected device files. HTTP fetching enforces size/time/redirect limits; reject private/loopback/link-local targets except explicitly configured trusted endpoints. Revalidate redirected destinations. Store downloaded content as untrusted evidence, never as agent instructions. Do not run documentation config files or package setup hooks just to read text.

Network availability, robots/access restrictions, missing distributions, licensing constraints, and sandbox failures must produce clear gaps. No silent unsupported fallback. These policies are enforced in the Rust core, not delegated to tool annotations or the skill.

## 11. Companion skill: Context7 + enrichment workflow

The authoritative deployable skill is in `skills/library-research/SKILL.md`. Keep it small and progressively load reference files only for complex research. Do not copy this whole blueprint into the skill.

### 11.1 Required research loop

1. **Frame the question.** State design objective, constraints, candidate libraries, and whether this is project use, upgrade research, or latest-release exploration.
2. **Resolve the environment.** Read the working repo's dependency context with ordinary agent tools. Ask `resolve_library` for exact identities. Do not pass private project text to Context7 unnecessarily.
3. **Discover broadly enough.** For unfamiliar libraries or architecture work, get `library_overview`: module/feature map, documentation headings, relevant release notes. For a known API question, skip broad enumeration.
4. **Retrieve explanations.** Use Context7 narrowly for the candidate concepts; use the correct language binding and library entry. Do not invent Context7 IDs or assume “latest” matches the target release.
5. **Deepen selected candidates.** Use `search_evidence` and `inspect_symbol`. Read targeted official/source fragments when documentation is incomplete or version-mismatched. Identify required types, features/extras, initialization/configuration, and interactions.
6. **Verify only material uncertainty.** Use source or LSP for interpretation and `verify_usage` for important disputed availability/composition. Verify feature/version-sensitive examples before recommending deployment.
7. **Produce a compact decision brief.** Separate factual support, architectural inference, alternatives, requirements, pitfalls, and unresolved issues. Cite versioned evidence handles and source locations.
8. **Stop.** End when the decision has sufficient support. Do not recursively enumerate an entire library or regenerate a prose reference book.

The service need not call Context7 itself to enforce this workflow. The agent is the orchestrator. A user-specific design-principles file can be loaded by the agent; the service remains topic-agnostic.

### 11.2 Capability discovery without a graph

Use two passes: a small breadth pass over modules/features/docs headings, followed by depth on a shortlist. Search by desired outcome and by likely technical mechanism. A purely name-based query can miss an unfamiliar capability; a full API dump overwhelms the agent. A faceted overview bridges that gap.

For each shortlisted capability, the agent's brief should contain:

```text
Capability and owning library/version
Design objective it addresses
Why it could be advantageous (explicitly an assessment)
Concrete API/configuration mechanism
Feature/extras/target/dependency requirements
Minimal deployment pattern
Composition constraints and alternative approaches
Supporting evidence IDs with locators
Verification performed and its scope
Remaining unknowns
```

Preserve this assessment as a small research bundle only when useful. Machine facts remain in service snapshots; judgments remain separately labeled. Future graph ingestion can link the two without reparsing a book.

## 12. Client installation and deployment

### 12.1 Build/install contract

The programming agent must provide a reproducible bootstrap script or documented equivalent that installs the service's Rust binaries, creates its own uv environment, installs exact-pinned FastMCP v4/Griffe/ty dependencies, and records compatible Rust tools. Commit `Cargo.lock`, `uv.lock`, and the selected toolchain configuration.

Do not install or update tools into a working repository's virtual environment. Do not use unpinned `uvx ...@latest` as the operational MCP startup command. Resolve dependencies during setup, not during every agent connection.

The installed command must be launchable by absolute path from an arbitrary current directory and find its own configuration/state. A proposed setup target is:

```text
/absolute/path/library-enrichment/.venv/bin/library-enrichment-mcp
```

### 12.2 MCP registration

Once the implementation and environment exist, use user-scoped registrations. The following are setup templates, not commands that can run before the repository is implemented:

```bash
REPO="$HOME/src/library-enrichment"

codex mcp add library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"

claude mcp add --scope user --transport stdio library-enrichment -- \
  "$REPO/.venv/bin/library-enrichment-mcp"

codex mcp list
claude mcp list
```

Codex supports stdio server commands through `codex mcp add`; Claude supports stdio commands with user scope. Keep Context7 registered separately. Registration alone does not prove the service connects: perform actual tool calls in both clients. [S22, S24]

### 12.3 Skill installation

Keep one source skill in this repository. Install it at user scope rather than copying enrichment code into every project. Current local skill locations are `~/.agents/skills` for Codex and `~/.claude/skills` for Claude Code. Codex documents symlink support; a directory copy is the conservative cross-client installation method. [S23, S25]

Provide `scripts/install-skill` that copies the skill directory only after checking for an existing installation. Refuse to overwrite an unrelated skill. Store an installed source/version manifest and provide explicit update/uninstall commands. For Codex, a symlink option is acceptable; verify Claude's actual discovery behavior before using a symlink there.

No automatic edits to the user's global client configuration or skill folders during ordinary test runs. Setup scripts must state their destinations and support a dry run.

### 12.4 Optional later remote use

An HTTP adapter can reuse the same tools/core for an always-on Ubuntu host and a portable client. Add authentication and explicit user/namespace boundaries before exposing it beyond loopback. Do not assume cloud-hosted agents can access a localhost or Unix socket. This is a later deployment profile, not a blocker for the local stdio system.

## 13. Implementation sequence and acceptance gates

Work in vertical slices. Every phase must leave a runnable and tested system. Do not complete a directory scaffold and claim the service is implemented.

### Phase 0 — Compatibility and contracts

Verify actual current package versions/APIs from primary documentation; select exact compatible pins. Record the FastMCP, Python, Griffe, ty, Rust, rust-analyzer, rustdoc JSON/parser, Arrow, and DataFusion compatibility matrix. Create a minimal FastMCP tool and test it in-process. Build DTO/schema and fixture scaffolding.

**Gate:** Exact dependency locks; tool input/output schema tests; `service_status` truthfully reports absent components; an unsupported producer format returns a typed error.

### Phase 1 — End-to-end static Rust slice

Implement registry identity, docs.rs metadata/JSON acquisition, one supported format adapter, normalized public API, immutable artifacts/snapshots, `resolve_library`, `library_overview`, `search_evidence`, `inspect_symbol`, and artifact reading. Include the daemon/stdio path, not just CLI producers.

**Gate:** A small real crate works from a clean cache, then works offline from the same snapshot. Source/version references resolve. An unavailable docs.rs JSON build returns partial evidence and a planned fallback, not an empty success.

### Phase 2 — End-to-end static Python slice

Implement exact distribution/wheel selection, import roots/stubs, static Griffe worker, inventory navigation, normalized Python evidence, and the same MCP tools. Include packages with re-exports and distribution/import-name differences.

**Gate:** Static extraction does not execute a fixture's import-time side effect. An extension-only fixture produces useful stub/docs evidence plus honest gaps. The worker never imports into the MCP environment.

### Phase 3 — Changes and bounded research

Implement normalized API additions/removals, selected compatibility interpretation, doc/config/release comparisons, deterministic search ranking, pagination, and output budgets.

**Gate:** An added Python function is discovered even when no breaking change exists; a behavior-only release note is surfaced with unchanged public API; config/toolchain-induced differences are separated from release changes.

### Phase 4 — Semantic and verification paths

Implement capsules, rust-analyzer and ty LSP clients/session reuse, supported-method probing, Cargo/ty checks, runtime inspection profiles, jobs/cancellation, and failure recovery.

**Gate:** A valid consumer and invalid consumer produce the correct opposite outcomes in both languages. The exact environment is recorded. Unsupported ty implementation navigation is not reported as “no implementations”. No working repo is modified.

### Phase 5 — Skill and client acceptance

Install the skill in a controlled test user directory and exercise both clients. Test a known API question, unfamiliar-capability discovery, an upgrade comparison, a docs/API disagreement, and a runtime-dependent Python question.

**Gate:** Both clients successfully use Context7 and the service, report version-qualified evidence, and produce a compact research brief. Record actual tool traces; do not invent client test results when clients or credentials are unavailable.

### Phase 6 — Operational hardening

Add single-flight deduplication, concurrency limits, restart recovery, dependency-update tests, provenance export, cache pruning, structured logs, and operations documentation.

**Gate:** Two clients share one extraction; a crash cannot publish half a snapshot; previously pinned evidence remains retrievable after newer evidence is published.

## 14. Test strategy and definition of done

### 14.1 Test classes

**Unit tests:** Identity normalization, archive/path handling, version/feature selectors, source locators, signature normalization, schema mapping, change classification, ranking, and cursor validation.

**Contract tests:** Actual FastMCP schemas equal the declared wire schemas; every status validates; resource and tool reads agree; invalid enums and unknown fields fail; limits never produce malformed JSON. Test FastMCP through its in-process client as well as a real stdio subprocess. FastMCP's documentation provides a Client/Pytest pattern for these tests. [S06]

**Fixture integration tests:** Check in small purpose-built Rust/Python fixture libraries with known edge cases. Store captured upstream metadata/rustdoc/inventory artifacts with source/digest attribution where redistribution permits. Core CI should not require a live internet response to remain stable.

**Opt-in live tests:** Resolve real current libraries, fetch docs.rs/PyPI/official docs, and validate current adapter assumptions. Select and record exact versions at run time. Keep live failures distinct from deterministic regressions.

**End-to-end clients:** Test real Codex and Claude integrations when installed/authorized. Otherwise deliver reproducible test instructions and explicitly mark these cases unexecuted.

### 14.2 Minimum acceptance cases

Include: exact version vs latest; Rust default vs optional-feature visibility; changed target; hosted rustdoc format mismatch; re-export alias; missing JSON; Python distribution/import mismatch; stub/runtime conflict; namespace package; native extension with missing runtime signature; static no-import safety; added nonbreaking API; behavior-only release note; unsupported LSP method; typecheck vs runtime distinction; producer timeout; duplicate requests; cancellation; restart recovery; stale registry; Context7 version uncertainty; output truncation and pagination; URI/path traversal; hostile downloaded instructions; project immutability; two-client stdio operation.

The companion `tests/ACCEPTANCE_PLAN.md` makes these cases concrete. Implement them rather than relying only on a few happy-path screenshots.

### 14.3 Operational metrics

Record cache hits/misses, producer duration, queued/running jobs, fetched bytes, response bytes, LSP startups/reuse, verification success/failure, and evidence gaps. Track tool output volume and number of required calls for a small fixed research-task set. These are engineering diagnostics, not a large benchmarking project.

Do not claim a universal research-quality percentage. Compare against concrete task outcomes: correct capability discovered, exact environment respected, useful integration requirement identified, and every material factual claim supported.

### 14.4 Final delivery from the programming agent

The finished repository must include working code, dependency locks, generated schemas, sample config, bootstrap/install scripts, CLI and stdio entry points, the skill, fixture/live tests, an operations guide, an architecture decision record, and a truthful test report. Document unsupported libraries/platforms/formats and recovery procedures.

Completion requires a demonstrated Rust and Python end-to-end path. Optional features may be explicitly deferred; required paths cannot be replaced by TODOs, canned evidence, or mock production responses.

## 15. Design review checklist

Before declaring a design or implementation complete, ask:

- Does the agent request semantic research operations, or must it orchestrate a collection of raw shell commands?
- Can every material returned fact be traced to exact source content and an extraction/verification environment?
- Can the system find an unfamiliar capability without dumping the entire API into the context window?
- Are declared availability, configured availability, type-check success, and runtime behavior separate states?
- Does each producer add typed evidence without becoming another publication authority?
- Can a caller get a useful partial answer without pretending the evidence is complete?
- Are project code, service code, service environments, and retained evidence physically separated?
- Does a new capability require a small producer/projection extension rather than a new bespoke pipeline?
- Does the implementation work without embeddings, a graph, an embedded LLM, or a new general-purpose server framework?
- Can today's evidence bundles become tomorrow's graph evidence without changing their source identities?

## 16. Governing instruction for implementation

> Implement the smallest complete, reproducible evidence service that lets a strong coding agent discover useful Rust/Python capabilities, retrieve precise support, verify consequential assumptions, and explain deployment choices. Preserve Rust ownership of the evidence model, use FastMCP 4 as a thin interface, use ty for Python semantics, and keep Context7 and design reasoning in the calling agent. Optimize for correct end-to-end operation and bounded evidence, not for building another universal knowledge platform.

See `SOURCES.md` for primary-source references and `AGENT_HANDOFF.md` for the copy-ready execution brief.
