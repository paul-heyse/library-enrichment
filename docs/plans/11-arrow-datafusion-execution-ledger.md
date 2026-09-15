# Plan 11 execution ledger

Active implementation, 2026-09-14. Authority remains [Plan 11](11-arrow-datafusion-completion-and-legacy-removal.md), including **all W0–W10** and both exits. This ledger is a checkpoint, not a reduced scope or completion report.

The working tree was already extensively dirty. No reset, stash or commit was used. Current-source digest at workload selection: `f872a92b6c5409757e8fed6bb04e2d17670b528e1424f07b853eeee0e632cc50`; it includes tracked and untracked implementation inputs through `scripts/evidence_run.py`. Subsequent edits require fresh execution receipts. Incremental logs below are diagnostic evidence and do not certify unchanged source.

## Decisions and workload inputs

- ADR-0028 proposes a concrete durable comparison job and exact direct-child reuse for explicit ExecuteOnMiss. Retained reads stay in the supplied context. Its scoped review and acceptance remain pending.
- ADR-0029 proposes explicit API presentation projections with `docs_included`; the complete retained observation ID is referenced, never recomputed from an incomplete payload. Its scoped review and acceptance remain pending.
- ADR-0030 proposes mandatory precommit complete delivery artifacts and typed catalog references. Resolve/Inspect/Verify now prepare delivery before catalog visibility; focused failure/restart/export tests pass. Scoped review and the complete operation/crash matrix remain pending.
- ADR-0031 adds a separately limited native Parquet admission process for evidence and catalog files after exact-version inspection disproved the eight-row preallocation assumption. The Arrow parser and schema validators remain shared; no alternate query engine is introduced. Scoped review, distribution support and measurements remain pending.
- One original-byte line scanner recognizes CRLF, CR and LF. Strict retained admission rejects unrepresentable positions. LSP's oversized-character normalization is an upstream protocol behavior, not the service's retained-position admission rule. No claim that universal rejection is prescribed by LSP is permitted.
- The [measurement protocol](../architecture/arrow-measurement-protocol.md#plan-11-workload-selection-2026-09-14) now fixes DataFusion 55.0.0/55.1.0 and Pydantic 2.13.4/2.13.5 with exact archive digests, scopes and unchanged budgets. Actual acquisition/helper/environment identities and the full skew workload remain W6/W7 inputs.

## Obligation owners, sources and oracles

The implementation owner for every row is the integration maintainer in this session. Source paths below are relative to `crates/`; test names identify executable oracles, not assumed passes. Package exits remain `not_run` until the full named scope has matching-source evidence.

| Obligation | Sources / existing concrete oracle | Remaining code or qualification | Dependencies |
|---|---|---|---|
| O1 model | Core `evidence/{relational,execution,text}.rs`; `execution_identity_is_nonrecursive_and_malformed_payloads_are_refused` | Complete identity/absence table and real retained token locations | W1/W4/W6 |
| O2 Arrow fidelity | Store `projection/`, `record_writer.rs`; `nested_observations_round_trip_through_parquet_and_datafusion`, `streamed_records_cross_batches_without_retaining_the_corpus` | All ten relations; invalid nullable descendants; large real producer qualification | W3/W4 |
| O3 metadata | `projection/cells.rs`, `provider.rs`; `text_encodings_preserve_null_empty_and_dictionary_values`, `release_metadata_roundtrip` | Alias/cast/join/window/aggregate/view/export matrix, conflicting metadata | W4 |
| O4 integrity | `admission.rs`; `duplicate_keys_and_conditional_foreign_keys_are_rejected_before_constraints`, `missing_or_mutated_exact_files_fail_closed_on_warm_and_cold_opens` | Full conditional-reference/provenance attack matrix | W1/W4/W5 |
| O5 publication | `repository.rs`, `catalog_generation.rs`; `catalog_job_publication_is_atomic_recoverable_and_requires_result_closure` | Every flush barrier; SIGKILL for Resolve/Inspect/Verify; precommit delivery admission | W2/W5 |
| O6 reuse | `admission.rs`, `leases.rs`, daemon `ops/source_tree.rs`; `exact_derived_inspection_reuses_retained_observations` | Ambiguous/changed child selection; eviction/invalidation/drop matrix | W1/W2/W5 |
| O7 research | `views.rs`, `browse.rs`, `search_plan.rs`; `admitted_views_preserve_observations_and_derive_ancestry_without_cross_scans` | Cross-operation independent semantic cases and actual MCP breadth/depth | W3/W4/W8 |
| O8 comparison | `comparison.rs`, daemon `ops/compare_job.rs`; `cold_version_comparison_waits_for_exact_acquisitions`, `native_comparison_preserves_nested_alternatives_and_pages_complete_keys` | One-side failure, actual independent adapters, all scopes/alternatives; unit cancellation with another subscriber passes | W2/W4 |
| O9 resources | `runtime.rs`, `record_writer.rs`, daemon `delivery.rs`; `resource_exhaustion_is_an_error_and_never_a_successful_empty_result` | Input producer growth, writer memory, real spill/full disk/deadline/abort/concurrent producer-query | W3/W5/W7 |
| O10 physical behavior | `query_diagnostics.rs`, `runtime.rs`; `production_inspection_view_skips_large_documentation_leaves`, `diagnostics_observe_the_executed_scan_and_bound_failed_query_history` | Bounded operator artifacts/cache timings, full fixed benchmark series and justified tuning | W7 |
| O11 operations | Existing `tests/e2e`, `tests/client`, ignored Rust execution tiers | Real requalification/producers, R09 same consumer, Rust C20, installer/client rewrite, owned cutover | W6/W8/W9 |
| O12 evidence integrity | `scripts/evidence_run.py`, acceptance scripts | Correlated client events, source/log tampering, fresh complete gates/report and G1–G7 review | W0/W8/W10 |

## Removal ledger checkpoint

| IDs | Current disposition | Remaining oracle |
|---|---|---|
| D1/D2 | Old flat store and major corpus query paths remain deleted | Final semantic deletion gate and supported-build inventory |
| D3 | `parquet_io` production export deleted; corrupt-fixture helpers moved under tests/support | Rules/negative fixtures and import inventory |
| D4 | Dead `StatePaths::contexts()` deleted | Final accessor/layout search |
| D5 | Typed fragment/aspect limits and narrowed inspection provider connected | Full alternatives/coverage tests; conflicting feature definitions now fail explicitly |
| D6 | Shared counting/overflow encoder and precommit delivery connected; small and large committed replies recover from admitted bytes without writes | Complete Resolve/Verify large-result, actual disk-full and interruption matrix |
| D7 | Marker-only source cache removed; private verified archive captures used | Full excerpt/staging caller audit and offline scenarios |
| D8 | `NormalizedEvidence` corpus removed; static normalization emits to `RelationWriter`; bounded `EvidenceBatch` remains for execution transport/tests | Rust/Python API walkers now visit bounded raw inputs; source/Markdown/inventory records are emitted from descriptors and borrowed sections. Bound remaining raw-parser/writer growth, finish native cross-batch dedup and measure full producer memory |
| D9 | No new cleanup has been applied | Exact nested ownership inventory, 43 old creator records, XDG residue and broker absence checks |
| D10 | Destructive shell implementation deleted; the command delegates to one bounded, locked generation installer | Twelve ownership/alias/arbitrary-cwd/SIGKILL cases pass; deployment configuration and correlated real-client acceptance remain |
| D11 | Some stale assertions corrected | Correlated real clients, per-scenario prerequisites and remaining producer assertions |
| D12 | Shared byte-coordinate scanner connected | Complete coordinate matrix and real LSP locations |
| D13 | Old revision acquisition writer remains deleted | Native attempt-log revalidation tests and old-file cutover |

## Dated incremental evidence

| Status | Command / diagnostic log | Scope and limits |
|---|---|---|
| passed | Store dataset/repository/views; `.dev-state/plan11-w3-record-writer-tests.log` | 3 dataset, 11 repository, 2 view tests after routing publication through the writer; later builder changes require recheck |
| passed | `production_inspection_view_skips_large_documentation_leaves`; `.dev-state/plan11-w3-native-narrowed-provider.log` | Default two partitions, ordinary and leased cached views; selected scan 13,582 bytes versus 1,011,708 full; cleanup released after request drop |
| passed | `cold_version_comparison_waits_for_exact_acquisitions`; `.dev-state/plan11-w2-comparison-recheck3.log` | Cold/warm/offline and restart terminal delivery; not other-subscriber cancellation |
| passed | `exact_derived_inspection_reuses_retained_observations`; `.dev-state/plan11-w2-derived-declared-tests.log` | Declared Python parent to exact retained child after restart, no fresh execution policy receipt |
| passed | Shared delivery unit; `.dev-state/plan11-w2-delivery-tests.log` | Escaped serialization count and identical overflow artifact reuse |
| passed | Schema regeneration/conformance; `.dev-state/plan11-w3-schemas.log` | Generated projection DTO; four positive and three negative cases |
| passed | `.dev-state/plan11-w3-streaming-records-admission.log` | 3,000 large fragments through bounded writer, exact ID reassembly and complete admission; 8.83 s. Earlier builder/admission failures remain in diagnostic logs; thresholds unchanged |

Additional 2026-09-14 diagnostics:

- `.dev-state/plan11-w3-visited-core-store.log`: the complete core/store test command passed (129 core unit tests and all crate binaries/integrations; 13 store unit tests and all store integrations). This includes a 6,000-observation Python visitor, 12,000 fragments, first-error propagation and borrowed original-byte Markdown sections. It does not measure whole-process peak memory.
- `.dev-state/plan11-w3-visits-final-clippy.log`: daemon and dependencies, all targets, passed with `-D warnings` after removing the duplicate production semantic-components fixture helper and charging all retained identity sets.
- `.dev-state/plan11-w3-connected-visits.log`: daemon 120 unit tests passed; resolution integration failed six stale inline-only assumptions. The failed log is retained.
- `.dev-state/plan11-w3-connected-paged-visits.log`: eight resolution and 19 retrieval tests passed after resolution assertions followed public `artifact.read` pages, including exact per-page digests and unchanged scope IDs. The retrieval helper has subsequently been changed to the same public paging helper; its fresh recheck is separate.
- `.dev-state/plan11-w2-comparison-subscription-recheck.log`: cancellation preserves another subscriber; an overflowed successful resolution remains usable by comparison.
- `.dev-state/plan11-w3-feature-alternatives.log`: conflicting qualified feature definitions are refused instead of overwritten.

The current static producer path is `Prepared` visits → `normalize_into` → bounded typed `RelationWriter` → native relational admission/publication. `ProducerBatch` and `EvidenceBatch` are separately bounded transport/fixture conveniences; neither is a full static-production corpus. Public-api still builds its own bounded-input signature representation, and parsed raw producer documents remain owned until visits finish. These costs and all writer/footer allocations remain explicit W3/W7 obligations.

No fresh full CI, real-container sweep, A01–A06, cutover or final acceptance report has run for Plan 11. W3, W4–W10 and the explicit residual W1/W2 obligations remain active work.

### Precommit delivery and native decoder checkpoint, 2026-09-14

- `.dev-state/plan11-w2-complete-delivery-admission-tests.log`: 122 daemon unit, 14 store unit and seven execution integrations passed. New oracles cover large inspection restart without re-execution, read-only delivery recovery, actual artifact-write failure preventing job publication, and exported delivery closure. The earlier export verifier failure on a missing listed file was fixed to return a bounded explicit problem; its failed diagnostic log is preserved.
- Catalog is now `catalog/3`, service state `library-enrichment-state/5`, journals `concrete-jobs/4`. No old-format reader/default adapter was added. The bundle outer-version audit and owned old-state deletion remain open.
- `.dev-state/plan11-w3-native-decoder-test-recheck.log`: four admission, four dataset and seven execution/publication tests passed through the isolated native reader. The 3,000-fragment dataset test remains a diagnostic, not the fixed performance series.
- `.dev-state/plan11-w3-native-decoder-attack-tests.log`: hostile-footer allocation abort was confined to the real worker; a subsequent valid admission succeeded. Native per-row-group reading passed an actual Arrow allocation bound that a combined native batch exceeded.
- `.dev-state/plan11-w3-cancellable-decoder-clippy-recheck.log`: daemon and dependencies, all targets, Clippy with `-D warnings` passed before the additional supervision tests. Later changed source requires fresh checks.

The previous eight-row cap was **not** a hostile-input memory guarantee: native Parquet footer/page allocation precedes decoded-batch validation. That heuristic is deleted. A mandatory sibling native executable installs checked Linux address-space/CPU limits, validates exact evidence/catalog bytes with the same schemas, and returns only a bounded completion report. The parent enforces concurrency, bounded output, cancellation, kill/reap and wall time. The configured 1 GiB is the decoder's virtual-address limit, not whole-service RSS. No full-plan resource or performance exit is certified.

### Source, static-producer and performance checkpoint, 2026-09-14

- Source excerpts now use the shared original-byte delimiter rules, including CRLF across read
  chunks and a final empty line. Source/archive capture verifies exact retained identities;
  bounded inventory, metadata and source-file work runs outside the async executor. Remaining
  ordinary artifact writes and raw parser allocation still need the full caller/bounds audit.
- `.dev-state/plan11-w1-w3-static-producer-recheck.log`: 17 real adapter/worker static Python and
  revision fixture cases passed. This fixed a precommit template bug: the final ProducerRun must
  replace its provisional row by attempt ID, rather than append a conflicting duplicate.
- `.dev-state/plan11-w2-fixture-durable-wait-recheck.log`: eight Resolve and 19 retrieval cases
  passed. Test consumers now follow actual pending jobs and verified artifact pages; producer
  work was not forced inside the inline response deadline.
- `.dev-state/plan11-w1-inventory-clippy.log`: daemon/dependencies all-target Clippy passed with
  `-D warnings` before the latest query-diagnostic and view changes.
- `.dev-state/plan11-fixture-series-1/measurements.json`: all five correctness runs passed but
  performance **failed**, median 7.5085 seconds and every run above three seconds. Exact
  identities and diagnostic limits are recorded in the measurement protocol. Thresholds remain.
- Native session-template isolation and the six typed Arrow tests passed in
  `.dev-state/plan11-w7-template-runtime-tests.log`. Leaf-rebound logical-view inlining is still
  a candidate: the expanded sort/exhausted-stream ownership test failed and requires a fix at
  the returned physical-plan/stream boundary before acceptance.
- Bundle outer contract is now `typed-evidence-bundle/3`; old formats remain unsupported.
  Rustdoc/source/Python producer identities were bumped for changed normalization semantics.

### Retention, measurements and installer checkpoint, 2026-09-14

- `.dev-state/plan11-w7-root-retention-tests.log` and
  `.dev-state/plan11-w7-root-retention-pruning.log`: repository/typed Arrow/session-isolation
  and native projection-pruning tests passed. Exact scan leases survive logical inlining;
  the final returned plan encloses native coalescing in the existing ownership wrapper.
  Exhausted result streams keep their lease until dropped. Fresh physical plans preserve
  independent metrics; no physical-plan cache/reset or alternate optimizer was introduced.
- Debug fixture series 2 improved to median 6.5740 seconds but failed the unchanged budget.
  Separate optimized release series 2 passed, median 1.4409 seconds and every run below three
  seconds. Exact artifact/source/worker identities and measurement methodology are recorded
  in the protocol and `.dev-state/plan11-fixture-release-series-2/measurements.json`. The
  optimized result is not a comparison against the old debug baseline or full W7 closure.
- `.dev-state/plan11-w8-installer-entrypoint-tests.log`: twelve tests passed. Actual SIGKILL
  covered a partial staged file, committed pointer, old-generation deletion and uninstall;
  recovery preserved an old-or-new complete public skill. Additional cases refuse changed
  content/extra files/links/manifests, unrelated targets, unsafe transactions and concurrent
  locks. Destination aliases deduplicate; both update and uninstall default to preview.
  The shell entry point ran from an unrelated cwd in a temporary user home. No operator
  credential files or real user configuration were installed or modified.

W8 still requires complete launch configuration and actual correlated client transcripts.
W5's broader publication/abort/cleanup matrix and W7's real-library/resource measurements
remain open, along with the other residual packages listed above.

### Workstation capacity and launch checkpoint, 2026-09-14

- Generated personal launch configuration selects the checked-in workstation resource profile:
  32 GiB shared query pool, 64 GiB spill, 2 GiB metadata cache, 16 partitions, 16 concurrent
  queries, 256 cached snapshots, eight expensive jobs and eight warm LSP sessions.
- `.dev-state/plan11-w8-workstation-32g-launch-test.log`: the absolute locked adapter, real daemon
  and Python extraction worker passed from an unrelated cwd, with caller bytes unchanged.
  `.dev-state/plan11-w7-workstation-fixture-correctness.log` passed the full offline correctness
  fixture under the larger query configuration; its debug timing is not a benchmark series.
- Removed hardcoded producer CPU and memory clamps. `Execution::resources()` drives validation,
  broker flags, qualification and identity. Workstation producer capacity is 32 CPUs, 32 GiB
  memory, 16 GiB scratch and 2,048 PIDs/threads. Capsule protocol v2 permits 64 GiB bounded data;
  old protocol and qualification receipts are unsupported. ADR-0032 is proposed.
- `.dev-state/plan11-w7-resource-config-tests.log`: nine configuration tests passed.
  `.dev-state/plan11-w7-resource-admission-tests.log`: seven qualification contract tests passed,
  including absent, changed, inconsistent and unlimited resource observations.
  `.dev-state/plan11-w7-resource-clippy.log`: core/daemon all-target Clippy passed with warnings
  denied. These contract checks do not substitute for the actual run below.
- First real qualification failed on the owned empty sidecar's obsolete state/4 marker.
  `.dev-state/plan11-qualification-sidecar-cutover.log` records explicit removal after canonical
  ownership, unchanged empty directories, all four coordination locks and absence of matching
  owned container journals were checked. Locks were preserved; unrelated p4p records/images
  remain outside that narrow deletion. No old-format reader or migration was introduced.
- `.dev-state/plan11-w7-workstation-qualification-2.log`: both selected actual producer images
  and all eleven real containment/cleanup tests passed. Receipt
  `.dev-state/p4p/admitted-images.json` at 2026-09-14T19:31:13+00:00 binds exact image/helper/broker
  and resource identities. Both images observed CPU `3200000 100000`, memory 34,359,738,368,
  swap zero, pids 2,048 and tmpfs 17,179,869,184 bytes, with confirmed cleanup.
- The tracked measurement harness now refuses zero matched tests. The current native client
  event parser is a prototype; parser regression tests, scenario integration and authenticated
  A01–A06 remain open. No operator credential files have been adopted.

The new producer receipt establishes configured containment, not full W6 producer journeys,
real-client acceptance, measured speedup or Plan 11 completion. All remaining scope stays active.
