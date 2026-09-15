---
title: Complete the Arrow and DataFusion architecture and remove superseded paths
status: in_progress
date: 2026-09-14
adrs: [ADR-0022, ADR-0023, ADR-0024, ADR-0025, ADR-0026, ADR-0027]
phase: 4
---

# Complete the Arrow and DataFusion architecture and remove superseded paths

## 1. Purpose and authority

This is the new completion plan requested after reviewing the implementation of
[Plan 10](10-arrow-datafusion-architecture.md). It has two independently verifiable outcomes:

1. Complete the Rust Arrow/DataFusion architecture, including its connected producer, query,
   publication, retention, resource and deployment behavior.
2. Delete every superseded production path, compatibility surface, obsolete test assumption
   and owned development evidence store. Replacements become the only executable paths.

**Review conclusion:** the central architecture has already changed substantially. Ten typed
relations, admitted providers, a shared bounded runtime, relational catalog publication,
native search/comparison and native snapshot contribution are connected. Starting another
storage rewrite would duplicate work. Completion requires integration repairs, bounded execution
at the remaining boundaries, proof of the advertised guarantees and removal of the residual
paths below. Neither overarching outcome is complete.

The user authorized implementation of the entire scope on 2026-09-14. Work is in progress;
package exits require the named executable evidence. It replaces Plan 10's stale **remaining-work sequence**, preserving
that plan's target contracts and O1–O12 obligations, including its integrated unfinished
Plan 09/blueprint Phase 4–6 scope. The supporting
[code review](../design_review/reviews/design_review_arrow-completion-and-legacy-removal_2026-09-14.md)
records findings N1–N10 and independent G1–G7 judgments.

Accepted ADRs and the living [design](../design/DESIGN.md) remain decision authority. Frozen
blueprints and accepted ADR arguments are not rewritten to accommodate implementation. A change
to a binding contract requires a new or superseding ADR with primary-source evidence.

### Lifecycle boundary

The hard pivot discards existing **development library evidence and historical snapshots**.
There is no migration, old-format reader, dual writer, optional legacy engine or backup/import
requirement. Preserve unrelated source edits, frozen design provenance, credentials and unrelated
host/container state.

For the target product, validated evidence for an unchanged exact version and qualified context
has no age-based expiry. Mutable registry selection may be refreshed without regenerating the
same documentation. A new version is a separate acquisition/selection event. Cache eviction
does not remove retained facts. Explicit evidence cleanup remains distinct from cache cleanup;
neither clock advancement nor repeated inspection authorizes it. Changes to source, environment,
producer or validator are assessed by actual dependency identity, not elapsed time.

## 2. Reviewed baseline and evidence limits

Reviewed 2026-09-14 in `/home/paul/library-enrichment`, HEAD
`cd6d9490a6433ae355b04a465538062e1320544c`, with extensive tracked and untracked changes.
HEAD alone does **not** identify the reviewed implementation. Recheck cited source locations
before editing. No application suite, producer, reset or installer was run for this review.
Existing logs are dated execution evidence, not current-source certification.

The lockfile selects DataFusion 55.1.0, Arrow 59.3.0 and object_store 0.13.2. The snapshot contract
is 5.0, catalog catalog/2, job journal concrete-jobs/3 and state marker library-enrichment-state/4.
No dependency change is proposed. Verify exact installed crate sources and the
[compatibility matrix](../architecture/compatibility-matrix.md) before using additional APIs;
Context7 is discovery support, not exact-version proof.

### What is already connected

Source paths without a repository prefix in the following tables are relative to `crates/`.

| Area | Current source evidence | Remaining qualification |
|---|---|---|
| One typed storage model | `enrichment-core/src/evidence/{relational,execution,metadata,ingest,snapshot,catalog}.rs`; `enrichment-store/src/projection/`; `dataset.rs:114` writes ten relations | O1–O4 complete representation and invalid-input matrix |
| Shared query resources | `enrichment-store/src/runtime.rs:74–110`: shared RuntimeEnv, FairSpillPool, spill/cache limits and scoped sessions | Actual concurrent resource/abort behavior and whole-process accounting |
| Native retrieval | Store `search_plan.rs`, `browse.rs`, `comparison.rs`, `views.rs`; `query.rs:245` filters exact paths before rendering | Residual early hydration in inspect/overview; optimizer/scan measurements |
| Coherent publication | Store `repository.rs:278–508`, `catalog_generation.rs`; native contribution/rebase | Full interruption matrix for Resolve/Inspect/Verify and concurrent publishers |
| Retention/export | Store `admission.rs`, `leases.rs`, `bundle.rs`; daemon `ops/resolve.rs` | Source-cache tamper handling, exact derived execution lookup, changed-file/restart lifecycle |
| Durable producers | Daemon `jobs.rs`, `ops/{resolve_job,inspect_execution,verify}.rs`; typed execution relation | Latest replay/delivery/helper edits need end-to-end rechecks; compare assumes synchronous resolve |

### Existing checks that constrain the next run

These are diagnostic observations, not an acceptance gate tally:

- `.dev-state/plan10-real-producer-sweep.log`: **24 passed, 3 failed**, 440.85 seconds. Two failures
  assert former contracts: the local rustdoc manifest producer list and derived-context snapshot
  equality. Tests were edited afterward but not rerun. The cancellation/cleanup test uses a broker
  wrapper with mismatched qualification; its intended runtime scenario was not exercised. Keep
  all three unresolved until execution proves their replacement assertions.
- `.dev-state/plan10-isolated-qualification.log`: actual image probes and boundary/cleanup checks
  after qualification state was isolated. This supersedes STATUS's earlier no-qualification claim;
  it does not automatically qualify every wrapper/configuration or later runtime helper change.
- `.dev-state/plan10-resolution-e2e-recheck.log`: 30 passed, one deselected and a real-XDG teardown
  error. Passing publication-crash assertions do not erase the isolation failure.
- `.dev-state/plan10-durable-current-build.log`, `plan10-durable-delivery-clippy.log`,
  `plan10-staging-recovery-unit.log`, `plan10-durable-journal-tests.log` and
  `plan10-derived-retention-unit.log` provide build/focused evidence only. Later changed behavior
  includes latest-version replay, acquisition-log attribution and bounded runtime reports.
- The [measurement protocol](../architecture/arrow-measurement-protocol.md) fixes a five-run
  baseline median of 0.5626 seconds. The recorded target diagnostic was 5.75 seconds, exceeding
  the per-run budget. It is not a completed current-target benchmark series. A large real
  Rust/Python workload pair is still absent from the protocol. Performance is unaccepted.

STATUS and earlier plan progress paragraphs predate some of this work. Use this baseline for
the next assignment; reconcile all status/design evidence labels at final closure.

## 3. Alignment to Plan 10

“Connected” means source routes through the target design, not that a package exit passed.
No T0–T7 exit is certified by this review.

| Original package | Current alignment | Remaining work here |
|---|---|---|
| T0 contracts/baseline/reset | ADRs 0022–0027 and initial protocol exist; primary reset ran | Workload identities, assertion map, nested-state cutover, narrow contract clarifications; W0/W9 |
| T1 canonical model | Typed relations, IDs, SearchSpec, execution payloads, generated contracts | Coordinate fidelity, exact retained selection, complete identity/absence matrix; W1/W2/W4 |
| T2 ingestion/admission/publication | Ten relations, encoders, relational admission and one target writer | Bound producer-to-batch transition, remove bypass exports, qualify closure/crash matrix; W3–W5 |
| T3 runtime/admission reuse | Shared runtime, admitted/provider caches, leases, scoped catalogs | Actual invalidation/abort/resource evidence, diagnostics and early payload bounds; W1/W3/W5/W7 |
| T4 browse/search/inspect | Main set operations native | Move residual aspect/kind/limit work before hydration; prove alternatives/coverage; W3/W4/W7 |
| T5 comparison/export | Native scope plans, pinned pairs, independently checked bundles | Cold version-form integration, expanded scope/metadata/export tests, delivery closure; W2/W4/W5 |
| T6 physical/product integration | Substantial Resolve/Inspect/Verify code and partial real evidence | Requalification, same-consumer stable/nightly, Rust canary, clients/installer, measured layouts; W2/W5–W8 |
| T7 deletion/certification | Old flat store and major collection queries removed | Removal ledger, fresh full gates, assertion review and docs; W9/W10 |

| Original review findings | Current disposition and follow-through |
|---|---|
| F1/F10 namespace authority | Typed paths/ancestry/views exist. Keep lexical containment distinct from MemberOf; cross-operation equivalence in W4. |
| F2 evidence claim | Real typed roundtrip tests exist; complete matrix/source-bound references in W4/W10. |
| F3 candidate completeness | SearchSpec/native eligibility/batch scoring exist; independent expected cases and actual MCP coverage in W4/W7. |
| F4 materialization | Main whole-corpus retrieval and enrichment loops removed; residual payload/producer stages in W3/W7. |
| F5 physical encodings | View/dictionary accessors and metadata restoration exist; no production forced non-view setting found. Complete W4. |
| F6 Delta | Continue accepted non-adoption. No parallel Delta catalog. Keep compatible-dependency plus demonstrated-consumer revisit trigger. |
| F7 freshness | Old compile diagnosis is stale. Full matching-source validation and truthful status remain W10. |
| F8 catalog/views/joins | Connected; pinned discovery, scope projections and observed plans in W4/W5/W7. |
| F9 keys/FKs/publication | Typed references, admission and root publication exist; negative/crash proofs in W4/W5. |
| F11 nested evidence | Connected, including release/execution payloads; nullable descendants must not admit invalid logical values; W4. |
| F12 metadata | Explicit schemas/restoration exist; casts/windows/mixed encodings/export matrix remains W4. |
| F13 diagnostics/UDF/session | Bounded sessions and narrow scoring kernel exist; executed-plan artifacts/operator metrics remain W7. |

## 4. Removal ledger

Each row needs caller/import evidence and a behavioral replacement oracle. Text search alone
cannot prove equivalent renamed code was removed. Retire each obsolete path in the change that
makes its replacement usable; leave no runtime legacy selector.

| ID / classification | Exact target | Required disposition and proof | Work |
|---|---|---|---|
| D1 already removed | `enrichment-store/src/{catalog,snapshot,tables}.rs` | Keep deleted. No old-schema writer/reader, spelling/default adapter or historical fixture migration. Fresh unsupported-format refusal remains. | W9 |
| D2 already removed | Production `all_symbols`, `all_fragments`, old collection comparison/search, forced `schema_force_view_types=false` | Scoped Rust search found no matches. Guard native plans semantically; no renamed corpus materializer. | W3/W9 |
| D3 public bypass; test callers only | `lib.rs:20` exports `parquet_io`; raw helpers at `parquet_io.rs:15,50` | Move required corrupt-fixture construction to test support or explicit non-production test feature; remove production export. Update current callers in daemon retrieval and store admission/typed-arrow tests. | W3/W9 |
| D4 dead layout API | `StatePaths::contexts()` in `paths.rs:124` | Remove unused accessor/stale flat-store comments; keep typed catalog contexts relation. | W9 |
| D5 residual execution pattern | Daemon `ops/overview.rs:86–113`, `ops/inspect.rs:216–276`; broad store fragment helpers | Replace hydration-then-kind/aspect/take/truncate with bounded typed query selections; delete unused helpers afterward. Preserve qualified alternatives. | W3 |
| D6 competing response policy | Daemon `ops/common.rs:249` and `jobs.rs:495` | One bounded result-artifact encoder/storage policy with explicit inline/journal parameters. Delete duplicate canonicalization/overflow builders, keeping distinct lifecycle semantics. | W2/W3 |
| D7 cache-derived authority | `ops/source_cache.rs:37` marker-only hit and `source::source_excerpt_checked` mutable member reads | Remove marker-only trust as validation. Bind safe archive-member reads to verified retained content; cache remains disposable acceleration. | W1 |
| D8 scheduled retirement | Whole `ProducerEvidence`/`NormalizedEvidence` accumulation before Arrow construction | Replace corpus-sized staging with bounded relation ingestion; remove duplicate owned graphs/post-hoc-only bounds. Keep bounded worker/output DTOs. | W3 |
| D9 old development payloads | Nested fixture state, old contexts/releases/acquisitions indices, snapshots and incompatible query caches | Validate physical root/marker/ownership, recover execution, preview exact paths and delete without import. Current reset handles only top-level cache/data. | W9 |
| D10 obsolete installation mechanism | `scripts/install-skill.sh:60–77` destructive in-place replacement | Single locked staged install/update/uninstall; delete former mutation logic and outdated entry points. A wrapper can only delegate. | W8 |
| D11 obsolete acceptance assumptions | Former manifest producers, derived snapshot equality, tool-name substring assertions, shared client skips | Replace with target scenarios, retaining acceptance IDs and original user journeys. No compatibility to satisfy old assertions. | W6/W8/W10 |
| D12 duplicate coordinate rules | Core retained-position validation plus three LF-only LSP document paths | One original-byte scanner; delete local splitting/counting conversions. Separate strict admission from protocol normalization. | W1 |
| D13 removed acquisition writer | Revision `AcquisitionEvent` / `record_acquisition` sidecar route | Source removed; rerun native attempt-log tests. Revalidation creates no data/acquisitions; remove old files under D9. | W2/W9 |

### Legitimate boundaries that remain

- Content-addressed raw archives, rustdoc JSON, text, logs and result blobs; typed relations
  describe identity/use, while raw bytes remain artifacts.
- Small visibility manifests, active job/creator/cleanup journals, locks, policy and capsule
  configuration. They govern effects/recovery, not a second evidence query store.
- Bounded producer wire types and final MCP DTOs, never persisted/query-authoritative relation
  copies. The `Symbol` presentation type is not itself proof of old storage.
- Pure specialized score/canonicalization/range kernels behind one contract. `score_symbol`
  is a test-only wrapper; `score_symbol_fields` is the live borrowed batch kernel.
- Test-local MemTables/Parquet corruption helpers and empty catalog schema tables, inaccessible
  as selectable production providers.
- Separate Griffe extraction/runtime helpers and the thin FastMCP adapter. No Python core,
  storage engine or parallel query implementation is introduced.

## 5. Dependency-ordered work packages

All package exits are initially **not_run** for this plan; existing code/checks are inputs.
Owners below are roles assigned when implementation resumes. Dependencies do not authorize
starting parallel agents or competing full-suite runs.

Main order: **W0 → W1/W2 → W3/W4 → W5/W6 → W7/W8 → W9 → W10**. W4 may start after W0;
W6 after W1/W2; W7 instrumentation alongside W3. Final evidence follows the dependencies stated
in each package.

### W0 — Fix the remaining contract and executable inventory

**Owner:** architecture/integration maintainer. **Depends on:** review. **Size:** small.

1. Map every O1–O12 and D1–D13 obligation to an existing or planned test/recipe. Distinguish
   missing code, existing unexecuted tests, stale assertions and blocked prerequisites. Capture
   current source identity including untracked code without resetting/stashing shared work.
2. Select exact real Rust/Python releases, source digests, import roots, target/features,
   environment/producer identities before measuring. Include two-release comparison and
   deterministic long-text/alias/observation skew; fixtures have independent expected results.
3. Settle cold version-comparison composition through existing concrete durable jobs. Also
   decide whether explicit ExecuteOnMiss may discover an exactly matching derived environment
   from a parent context. Default retained reads stay within the supplied context; never merge
   environment alternatives silently. Record binding changes through the ADR workflow.
4. Distinguish strict retained UTF-8 validity from LSP response normalization. The protocol
   recognizes CRLF/LF/CR and specifies oversized-character behavior; do not present universal
   rejection as upstream semantics. Use the dated primary-source matrix and a new ADR if the
   accepted contract changes, without rewriting accepted ADR-0026's argument.

**Exit:** every obligation has an owner, source target, oracle and dependency; open decisions
are explicit. No generic query DSL, schema compiler, workflow engine or new storage backend.

### W1 — Make reused source and semantic coordinates trustworthy

**Owner:** evidence/producer maintainer. **Depends on:** W0. **Findings:** N2/N3. **Size:** medium.

1. Replace source-cache marker-only admission with verified archive-member identity and safe
   reads. Validate regular-file/path ownership at open; do not follow substituted links. Prefer
   verified member reads or bounded member digests over rehashing the entire tree per excerpt.
   Reconstruct corrupt disposable cache from retained artifacts without upstream acquisition or
   documentation regeneration.
2. Make excerpts, runtime/semantic staging and citations refer to the exact verified bytes.
   Audit every unpack caller. Keep blocking locks/archive I/O off async executor workers.
3. Share a scanner across `Utf8Position::validate`, automatic anchors, `protocol_position` and
   `byte_position`. Preserve original bytes; recognize CRLF before CR/LF; retain final empty
   lines. Validate encoding before zero-column returns. Unrepresentable ranges are limitations.

**Oracle / exit:** mutate a completed cached member, replace it with a symlink, and change it
without changing length: no excerpt may cite the original archive while showing substituted
bytes. Good reuse works offline. Coordinate tables cover empty/trailing/repeated delimiters,
non-BMP/combining text, both supported encodings, invalid encodings, reversed/outside ranges
and surrogate interiors. For `"a\r\r\n🌎é\r\n"`, line starts are `[0,2,4,12]`; line 2 UTF-8
columns `[0,4,6]` map to UTF-16 `[0,2,3]`. Real navigation points to known retained tokens.

### W2 — Finish durable resolution, retained execution and delivery

**Owner:** daemon integration maintainer. **Depends on:** W0; coordinate cases W1.
**Findings:** N1/N10; D6/D13. **Size:** large.

1. Fix `ops/compare.rs:57–76`: cold resolve returns Pending, but compare only handles Error
   before requiring a context ID. Compose exact resolution jobs with retained fast paths,
   bounded waiting and explicit intermediate responses. No private synchronous resolver or
   generic workflow engine. Pin the common catalog generation after both sides resolve.
2. Audit all resolver callers and client workflows for the same assumption. Preserve request/job
   IDs and cancellation subscriptions; one client cannot cancel another client's needed work.
3. Recheck latest-version replay through native job publication. A registry observation may
   contribute new selection provenance; unchanged documentation is not extracted again. Prove
   subsequent identical selection stabilizes and offline replay uses admitted facts. Finish
   revision revalidation via actual attempt logs, with no acquisition JSON index.
4. Implement W0's exact derived-context selection using catalog predicates over environment,
   document, symbol, method, position, runtime selection and producer identity. Exact child
   replay survives restart/eviction; ambiguity stays explicit; no cross-environment fact copying.
5. Consolidate D6. Bound serialization before a second huge JSON allocation; enforce immutable
   result size separately from inline/journal limits. Admit required result artifacts before
   successful job publication. Large completed results remain terminal and retrievable after
   restart without rerun. Disk errors/over-limit output cannot strand committed active journals.

**Oracle / exit:** cold/warm version comparisons, one-side failure, latest selection, revision
logs, two actual adapters/one cancellation, repeated parent/child inspection and restart preserve
IDs/coverage. Test oversized Unicode/escaped JSON, full paging, journal reconstruction and export
closure. Resolve/Inspect/Verify use one publication/terminal contract, with no second evidence store.

### W3 — Complete bounded native selection and ingestion

**Owner:** store/query maintainer. **Depends on:** W0/W2. **Findings:** N4/N5. **Size:** large.

1. Give inspect/overview typed aspect, subject/binding, fragment-kind, column, page and budget
   selections. Lower them before decoding. Signature-only inspect must not hydrate all docs;
   overview needs bounded crate-doc selection and native feature summaries. Push relationship
   limits and coverage/confounder aggregation into plans where they reduce data before rendering.
2. Preserve all required qualified alternatives via stable paging or explicit resource failure;
   do not silently truncate facts. Keep same-path kind/definition/source distinctions. Delete
   broad fragment helpers and handler post-filters once their callers migrate.
3. Retain native search eligibility/folding/order and comparison joins. Hydrate after bounded
   key selection. Every remaining Rust reduction has a bounded-output or specialized-kernel
   purpose, rather than another eligibility/comparison implementation.
4. Replace corpus-sized producer/normalized staging with a simple bounded relation sink/iterator.
   Release owned records as batches are written. Bound single values, total producer input,
   relation rows, identity state, writer buffers and concurrency before growth. Cross-batch
   dedup/FK checks remain relational. A bounded worker message can remain a DTO; remove duplicate
   full owned graphs. Preserve semantic IDs independent of batching and row order.
5. `ops/artifact.rs:49` reads the whole blob before paging and builds all Markdown sections for
   section requests. Replace with bounded range reads and verified streaming section locations,
   preserving CRLF/UTF-8 byte offsets. Raw slicing remains ordinary Rust I/O, not an Arrow table.
6. Move raw `parquet_io` fixture helpers out of production exports. Keep one admitted provider
   route and one bounded evidence writer; test-only raw construction remains available for
   corrupt-input oracles, never as a runtime option.

**Oracle / exit:** executed projections/limits prove small requests avoid unrelated large text.
Expected cases include same-path alternatives and tight budgets. Large producer input respects
declared limits without corpus-sized duplication; batch/order variants preserve IDs. Paged
artifacts reassemble exact bytes, including CRLF sections/malformed text. D3/D5/D8 callers retire.

### W4 — Close Arrow fidelity and native semantic equivalence

**Owner:** schema/query maintainer. **Depends on:** W0; finish after W1–W3. **Size:** medium.

1. Complete O1–O4 for all ten relations: absent/present-empty Struct/List, unknown/failed/partial,
   nullable physical descendants with required logical values, tags, duplicate keys, conditional
   local/external/unresolved references and exact provenance closure.
2. Exercise write/read/scan/project/alias/cast/join/aggregate/window/view/export with selected
   string/view/dictionary encodings. Preserve required metadata or explicitly derive/restore
   its role at the one projection boundary. Conflicting metadata fails; extension tags are not
   validation. Do not add legacy coercion/default branches to make fixtures pass.
3. Prove SearchSpec and lexical namespace semantics across overview/search/inspect/compare:
   short/name/summary-only queries, Unicode/case, separators, aliases, missing parents and
   lexical versus semantic membership. Keep independent expected results.
4. Expand comparison to every scope, same-path qualifiers, alternative sets, partial coverage,
   qualified relationship endpoints, CRLF/indentation and NULL/empty values. Stable change IDs
   and expected changes are the oracle; no production reference to the old implementation.

**Oracle / exit:** extend and run `typed_arrow`, `admission`, `scoring`, `repository`, daemon
`retrieval_fixture` and Python fixture tests for missing cells. Independently close the earlier
connected-slice review findings. Regenerate changed wire schemas from Rust; conformance passes.

### W5 — Prove publication, ownership and recovery

**Owner:** storage/operations maintainer. **Depends on:** W2–W4. **Size:** large.

1. Extend the current five debug probes to complete O5: before/after file and manifest flush,
   snapshot rename, catalog-file persistence/root replacement and terminal journal persistence.
   Use actual process interruption as well as injected errors.
2. Exercise all three job kinds, same-context distinct contributions, distinct-context writers,
   same-identity retries and rebase conflicts. ID/name/current readers see one generation.
   Committed results recover without replay; precommit interruption cannot appear successful.
3. Prove provider/physical-plan/stream/catalog/export leases through timeout, dropped futures,
   cancellation, cache eviction, pointer changes, compaction and cleanup. Idle cache entries
   cannot indefinitely block explicit cleanup.
4. Verify export independently with source daemon/store absent: qualified attempts, logs, inputs,
   derived ancestors and promised delivery artifacts form complete closure. Reject missing or
   same-size replaced bytes without publishing a destination.
5. Attack startup staging recovery with unknown children, oversized counts/files and links.
   Uncertain external resources are not deleted. Keep crash probes test-only.

**Oracle / exit:** native fault/concurrency tests and real daemon SIGKILL/restart scenarios
assert old-or-new generations and before/after resource inventories. No selectable partial
snapshot, no producer rerun merely to restore delivery, and no hidden uncertain cleanup.

### W6 — Qualify every producer on the target evidence path

**Owner:** execution maintainer. **Depends on:** W1/W2; final run after W5. **Size:** large.

1. Requalify actual image/helper/broker/configurations after changes. The cancellation wrapper
   needs its own real qualification, not a copied/forged receipt. Reuse the repaired user-bus
   environment; preserve isolated qualification state and actual configured resource checks.
2. Rerun all three unresolved real-sweep scenarios against corrected target assertions. Execute
   current latest-selection, revision-log, large Unicode docs and oversized-signature tests.
   Keep valid typed partial reports rather than clipped invalid payloads.
3. Complete actual ty/rust-analyzer known-location/navigation/diagnostic cases, runtime objects,
   unsupported/empty/partial outcomes, cancellation and retained replay/export.
4. Prove Rust target/default/features/locks, hosted-first acquisition and qualified fallback.
   R09 must compile the **same consumer snippet and dependency closure** on stable and the
   configured dated nightly. Nightly rustdoc plus a different stable example is insufficient.
   Add no arbitrary-shell or caller-selected unrestricted toolchain interface.
5. Extend C20 to actual Rust producer/semantic/verification paths and every enabled execution
   profile, alongside Python canaries; digest before/after failure and cancellation. A repository
   under study is never a subprocess cwd, install target or extraction/output root.
6. Prove two-client single-flight with one surviving subscriber and same-context enrichment of
   distinct facts. Repeated derivation preserves child observations while excluding unrelated
   environment execution facts.

**Oracle / exit:** appropriate R/P/C scenarios execute through daemon/MCP and native store;
the ignored real-container tier runs explicitly. Missing prerequisites stay named blocks;
readiness alone is not producer acceptance. No owned process or unresolved cleanup remains.

### W7 — Measure and improve the Arrow/DataFusion pipeline

**Owner:** query/performance maintainer. **Depends on:** W3/W4; final workloads W6. **Size:** large.

**Operator clarification, 2026-09-14:** prompt-serving latency for coding agents is the primary
optimization objective. Initial intermittent personal use is on a 16-core/32-thread, 192 GiB
workstation. The initial small-pool settings are reference measurements, not target resource
ceilings. Use substantially larger configurable memory and parallelism where measurements
support faster responses; assess serial planning/admission, producer CPU limits and cache/plan
reuse alongside native parallel execution. Keep malformed-input/runaway protection and actual
resource observations, without treating minimal memory consumption as a success criterion.

1. Capture bounded logical/physical plans and metrics from the **executed** physical plan.
   `QueryOutput` currently exposes batches, rows, Arrow bytes and elapsed time; daemon metrics
   cover requests/fetch/probes. Add scan/pruning/spill/output information, catalog/admission
   cache behavior and stage timings. Keep diagnostics outside Coverage and public SQL; give
   diagnostic artifacts bounds separate from indefinitely retained library facts.
2. Measure acquisition, normalization, Arrow construction, validation, publication, cold/warm
   open, planning, execution, hydration and serialization. Record process RSS, managed pool,
   input/output/batch/writer allocations, spill, row groups/pages scanned, caches and write
   amplification. Shared pool size is not a whole-process memory ceiling.
3. Run the fixed five-run fixture and preselected real/shape workloads. Preserve thresholds:
   median ≤1.5 seconds and every fixture run ≤3 seconds; warm selective end-to-end ≤1 second;
   broad query execution ≤5 seconds; cold admission ≤30 seconds. Do not move budgets after
   failure. Changes require workload justification and explicit review. Do not drop host caches.
4. Tune measured costs only: reusable derived plans/providers/footer admission, actual key sorting,
   row-group/page sizing, projected scans, decoder predicates, useful bloom/statistics and catalog
   compaction. Verify physical sort before asserting ordering. Keep only measured beneficial,
   semantically equivalent changes; no unsupported nesting-pruning or zero-copy claims.
5. Exercise concurrent broad search/comparison under small memory/spill limits, forced spill,
   full spill disk, large values, queued deadlines and abort. Require typed failure, bounded
   allocations and released permits/tasks/files/leases. Include simultaneous producer/query load.

**Oracle / exit:** source/workload/configuration-bound measurements satisfy the protocol and
O9/O10. Selective requests avoid full payload reconstruction. Remove ineffective optimization.
Persistent text indices, secondary projections, custom optimizer nodes and Delta remain behind
Plan 10's existing measured triggers.

### W8 — Replace installation and prove actual client use

**Owner:** product integration maintainer. **Depends on:** W2/W6; contracts W4. **Size:** medium.

1. Replace in-place installation with one locked staged install/update/uninstall. Default to
   preview; validate exact ownership/content digests; reject modified/unrelated destinations;
   handle symlink aliases; atomically replace and recover interrupted updates. Provide absolute,
   locked adapter/daemon launch configuration from an unrelated cwd. Never implicitly install
   the product skill during an agent session.
2. Parse actual client event records and correlate invocation IDs with successful typed results.
   Check returned context/snapshot/artifact identities against the daemon. Remove `tools_called`
   substring matching; a model repeating a tool name cannot pass. Verify breadth/depth order
   and full original gate scenarios.
3. Separate Codex, Claude and Context7 prerequisites by scenario. Missing credentials for one
   cannot skip another runnable gate. Harness crash/missing summary is a harness failure, not
   automatically a blocked prerequisite. Credential adoption remains explicit and minimal.
4. Run A01–A06 in isolated authenticated homes with newly generated target evidence and separate
   Context7. Cover arbitrary-cwd startup, unavailable service, exact/unknown version support
   and job polling after durable resolution.

**Oracle / exit:** installer ownership/atomicity tests and real correlated client transcripts
pass; real-user configuration digests are unchanged. External prerequisites remain specific
blocks with reproduction commands and keep this package incomplete.

### W9 — Execute owned cutover and close the removal ledger

**Owner:** storage/operations maintainer. **Depends on:** W1–W8, especially W5/W6 recovery.
**Size:** medium. This is one development cutover, not a migration subsystem.

1. Re-inventory D1–D13 through imports/callers, feature flags, recipes and tests. Delete obsolete
   implementations, accessors, comments and bypass exports. Guards include negative fixtures
   and explicit exceptions for the legitimate boundaries in §4.
2. Inventory nested physical state, beyond top-level reset paths. Review confirmed
   `.dev-state/{rn,rm,w1b,rj,lc,h,w1c,w1,rk,rl,ri,phase456-historical-capture}` exist with pytest
   subdirectories/aliases. Enumerate actual service payloads/formats; short names do not prove
   ownership. Do not follow `*current` aliases or import the historical capture.
3. `.dev-state/p4p/owned` contained 43 older creator records at review time with fields
   `capsule,created_at,image,name,owner`. Resolve each against canonical service ownership and
   actual broker state. Deleted fixture directories do not prove container absence. Preserve
   unresolved journals until absence is established; remove confirmed-owned resources only.
4. Reconcile the known test-created XDG residue. Review found `/home/paul/.cache/library-enrichment`
   with ownership/root markers and operational directories, and
   `/home/paul/.local/share/library-enrichment` with a state marker. Revalidate ownership, contents
   and live users before removing these test-created paths; do not reset unrelated production
   evidence. This closes the earlier isolation incident.
5. Extend preview/apply reset only enough for validated nested roots, or use a narrow one-time
   operator action with the same ownership oracle. No permanent legacy reader. Remove old
   evidence/snapshots/indices/incompatible caches without backup/import; preserve unrelated
   source/logs/credentials/container storage outside the selected inventory.
6. Add scoped rules/fixtures and a whole-repo deletion gate for raw production Parquet utilities,
   unadmitted providers, private legacy resolver, broad handler corpus materializers, old index
   writers and engine switches. Do not edit frozen enforcement files. Delete stale documented
   entry points; any retained wrapper delegates with no independent mutation logic.

**Oracle / exit:** preview/applied inventories agree; ownership/recovery, fresh bootstrap,
offline reuse and separate cache/evidence cleanup pass. Every D row has removal or justified
boundary evidence. No old snapshot/compatibility fixture is needed, and no uncertain process
is concealed by deleting its journal.

### W10 — Certify both outcomes and reconcile documentation

**Owner:** integration/review maintainer. **Depends on:** W0–W9. **Size:** medium plus live gate time.

1. During implementation run affected crate Clippy/tests. After source stabilizes run full
   `just ci`, explicit real-container tests, relevant phase gates and actual clients. Use the
   pinned toolchain, `CARGO_INCREMENTAL=0` where required here, and `uv run --frozen` for Python.
   Start with `just --list`; ordinary workspace tests do not include all ignored/client tiers.
2. Run schema generation/conformance, dependency policy, rules, toolchain, frozen provenance
   and state-leak checks. Regenerate/check the acceptance report; audit assertion adequacy and
   source/log receipts, including changed tracked/untracked source and tampered/missing logs.
3. Independently reassess G1–G7, Blueprint §15, N1–N10, O1–O12 and D1–D13. Compilation, EXPLAIN,
   source inventory and registry wiring are not substitutes for their actual scenarios.
4. Reconcile STATUS, index, Plan 10/09 dependency notes, living design labels, operations and
   product skill. Correct the measurement protocol's stale row-materialized enrichment wording
   to the actual native union behavior/cost. Accepted ADRs remain immutable.
5. Append the actual outcome: what was built/verified, a mistake corrected and deliberate
   deviations. Do not mark done while either exit below is incomplete. Blocked external clients
   keep the full scope incomplete rather than becoming an implicit waiver.

**Architecture exit:** all supported operations use target relations/admission/runtime;
semantics, retention, publication, resources and full Phase 4–6 journeys have matching-source
execution evidence; required performance budgets pass.

**Deletion exit:** no superseded reader/writer/query/selection/install behavior is reachable in
any supported build/profile; no old development evidence or compatibility fixture is required;
owned reset is complete; only the justified boundary representations remain.

## 6. Oracle and completion ledger

During execution record actual test names, commands, source identities and logs beside each
row. Every initial exit is `not_run`; use only `passed`, `failed`, `blocked`, `not_run` for
execution status. W/O/D IDs are not new acceptance gate IDs.

| Plan 10 oracle | Work | Required evidence beyond source inspection |
|---|---|---|
| O1 semantic model | W1/W4 | Literal IDs, variants/cardinalities, qualified observations, independent expected meaning |
| O2 Arrow fidelity | W3/W4 | Ten-relation encoding/decoding and negative representation matrix |
| O3 metadata | W4 | Scan/project/alias/cast/join/window/view/export preservation or explicit derivation |
| O4 relational integrity | W1/W4/W5 | Duplicate/FK/provenance attacks, same-size replacement, exact closure |
| O5 coherent publication | W2/W5 | Every durable barrier, real restart, concurrent writers, terminal reconstruction |
| O6 reusable admission | W1/W2/W5 | Warm opens, file/validator changes, eviction, pinned reads, leases |
| O7 search/browse | W3/W4/W8 | Expected candidates/namespaces/cursors plus actual MCP traversal |
| O8 comparison | W2/W4 | Cold convenience and pinned forms, all scopes, alternatives, coverage |
| O9 resource lifecycle | W3/W5/W7 | Memory/spill/disk/deadline/abort and owned-resource inventories |
| O10 physical behavior | W7 | Executed plans/metrics, actual sort/pruning, encoding/filter variants |
| O11 deployment/operations | W6/W8/W9 | Real producers/clients, reset/export/canaries, indefinite evidence reuse |
| O12 evidence integrity | W0/W8/W10 | Correlated client results, source/log tampering, real test mappings, fresh reports |

## 7. Decisions and deliberate exclusions

No additional platform is needed. Keep Delta, custom optimizer nodes, persistent lexical indices
and secondary layouts behind existing triggers. A measured need for another Arrow projection
must identify its consumer, source/derivation identity, invalidation, cleanup and benefit first.

The W0 choices are bounded: durable version-comparison composition, exact derived-environment
discovery for explicit execution and accurate protocol/admission coordinate handling. Resolve
them before dependent behavior; ambiguity is not a reason for multiple selectable implementations.

The original review was read-only. Implementation is now authorized and active. The
[execution ledger](11-arrow-datafusion-execution-ledger.md) records current changes, residual
obligations and diagnostic logs; no package is complete merely because its first regression passes.
