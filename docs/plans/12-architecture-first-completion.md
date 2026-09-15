---
title: Finish the native architecture and remove legacy before completing Phase 4–6
status: done
date: 2026-09-14
adrs: [ADR-0022, ADR-0023, ADR-0024, ADR-0025, ADR-0026, ADR-0027, ADR-0028, ADR-0029, ADR-0030, ADR-0031, ADR-0032, ADR-0033, ADR-0034, ADR-0035]
phase: 4
---

# Finish the native architecture and remove legacy before completing Phase 4–6

## Scope and execution priorities

This is the remaining-work plan requested on 2026-09-14. It replaces the execution order in
[Plan 11](11-arrow-datafusion-completion-and-legacy-removal.md), using the target contracts in
[Plan 10](10-arrow-datafusion-architecture.md), the current
[execution ledger](11-arrow-datafusion-execution-ledger.md), and the still-required functional
scope in [Plan 09](09-phase-4-6-completion.md). It contains gaps and actions, not a completion
inventory. Implementation was authorized on 2026-09-14; the
[execution ledger](12-architecture-first-execution-ledger.md) records current work and checks.

**Order: native architecture → legacy decommission and development cutover → remaining
functional capabilities → integrated acceptance.** In particular, legacy removal no longer
waits for real-client acceptance or performance tuning, as Plan 11 W9's dependencies implied.
Publication, identity and cleanup repairs necessary to make the cutover sound belong before
that boundary; additional producer/client functionality belongs after it.

This is a design-stage hard pivot. Replace unsuitable internal contracts directly, regenerate
their consumers, and delete superseded implementations. Do not add migration readers,
compatibility adapters, dual writers, engine switches or preservation work for old development
library evidence. Preserve unrelated source changes, credentials and frozen design provenance.
Future validated evidence for an unchanged exact version and qualified context remains reusable
without age-based expiry; disposing of development state does not change that product contract.

The audit used current source and test assertions, not the plans' historical completion labels.
A missing implementation, an inadequate assertion and an unexecuted integration scenario are
different gaps. Existing code should be completed or removed, not rebuilt because its broader
work package is still open. No new runtime gate suite was run for this planning review.

### Validation approach

Implementation and useful final output take priority over exhaustive intermediate certification.
Use existing end-to-end journeys and inspect their resulting library information: API coverage,
signatures, documentation, alternatives, citations, environment qualifications and explicit gaps.
Where useful, compare those outputs with exact source/rustdoc/worker inputs or known fixture
expectations. New probes are optional tools for answering a concrete unresolved question, not
an additional workstream or prerequisite.

Add focused regressions when a change could silently lose information, alter identity, expose
an incomplete publication, or delete the wrong state. Do not require a Cartesian test matrix
of every Arrow encoding, operator, nullable field and execution profile before continuing.
Plan 10 O1–O12 and Plan 11's matrices remain coverage checklists; their exhaustive intermediate
test expansion is replaced by representative integrated checks and targeted defect tests.
The semantic guarantees remain required. Existing named product acceptance scenarios remain
in scope and must assert their intended outcomes.

Keep the current resource configuration while doing this work. Resource exhaustion must remain
an explicit failure, but further workstation sizing, parallelism tuning and benchmark campaigns
are deferred until the architecture and functional paths settle.
The user subsequently clarified on 2026-09-14 that the arbitrary timing target must not delay
correctness, robustness or functional completion. Record the existing measurement as a baseline;
defer performance optimization and threshold enforcement to subsequent work.

## Phase 1 — Finish the Rust Arrow/DataFusion architecture

**Dependency:** none beyond settling the relevant internal contracts. This is the first
implementation priority.

### AR1 — Remove collection-based relational assembly from ingestion

**Gap:** `normalize_into` still maintains full-input definition/binding/path maps and observation,
relationship, fragment and input-ID sets. `Repository::complete_files` accumulates semantic keys
for an entire relation, then `sorted_digest` sorts and deduplicates them in Rust. These remain
relational operations outside DataFusion even though later storage uses Arrow.

Source anchors: [ingest.rs](../../crates/enrichment-core/src/evidence/ingest.rs)
(`normalize_into`, `admit_identity`, `fragment_subject`),
[repository.rs](../../crates/enrichment-store/src/repository.rs) (`complete_files`, `validate_blobs`),
[dataset.rs](../../crates/enrichment-store/src/dataset.rs) (`semantic_plan`, `batch_keys`, `sorted_digest`).

Remaining work:

1. Emit bounded typed staging records with explicit producer-local references. Resolve bindings,
   fragment subjects and relationship endpoints with native joins against staged target identities.
   Use native distinct/grouping and conditional anti-joins for cross-batch deduplication,
   collisions and reference closure. Temporary staging is part of the one ingestion path,
   not a second persistent evidence store.
2. Keep raw-language traversal, cycle prevention, alias interpretation and canonical ID kernels
   in ordinary Rust where they supply domain semantics. Remove the relation-wide maps/sets
   that duplicate relational grouping or reference resolution after extraction.
3. Project canonical semantic keys, distinct/order them in DataFusion, and hash their ordered
   stream without collecting the entire relation's keys. Preserve the chosen canonical hash
   preimage and literal identity contracts; any deliberate identity change needs a recorded
   contract change and fresh state, not an old-reader branch.
4. Move repeated input/provenance matching and closure deduplication into native selection
   where it currently rescans or retains a whole input relation. Keep exact file hashing as
   bounded Rust I/O.

**Check:** representative Rust and Python acquisition through final overview/search/inspect/
comparison output; a cross-batch duplicate/conflict case; stable semantic identity under reordered
input and different batch boundaries. Inspect the resulting code to ensure removed collectors
have no production caller. Do not build a second reference engine.

### AR2 — Finish the bounded parsing, writing and result boundaries

**Gap:** Rustdoc deserialization disables recursion limits before item-count/value checks; the
raw crate and signature-rendering representation can allocate before those checks. Native assembly
casts batches and performs synchronous Parquet writes inside an async consumption callback.
Serialized-record and post-construction Arrow limits do not by themselves bound parser, cast,
compression and footer allocations.

Source anchors: [Rust normalizer](../../crates/enrichment-core/src/producer/normalize.rs)
(`deserialize_crate`, `prepare`, `render_signatures`),
[Python normalizer](../../crates/enrichment-core/src/producer/python/normalize.rs),
[record_writer.rs](../../crates/enrichment-store/src/record_writer.rs),
[dataset.rs](../../crates/enrichment-store/src/dataset.rs) (`write_transformed_plan`),
[resolve.rs](../../crates/enrichment-daemon/src/ops/resolve.rs),
[python.rs](../../crates/enrichment-daemon/src/ops/python.rs).

Remaining work:

1. Establish a practical allocation and recursion boundary for hostile raw producer input before
   it can exhaust the daemon. Reuse the bounded native-process pattern where the upstream parser
   cannot enforce the required limit. Do not write a second rustdoc/Parquet parser.
2. Bound actual retained producer state and batch construction, including nested values, casts,
   writer buffers and final serialization. Remove duplicate whole-input representations where
   the native library or extraction boundary permits it. Distinguish a bounded raw AST needed
   for language semantics from a duplicate normalized evidence graph.
3. Move blocking file/lock/archive/writer work off async executor workers. Use bounded handoff
   and cancellation between query streams and writing; releasing an async caller must not
   release ownership while its blocking task continues.
4. Audit remaining normalizer fallbacks. `render_signatures` currently returns an empty map on
   a public-api build error; `EvidenceFragment::new` turns a non-object locator into an empty
   map. Make affected failures or missing information explicit at the typed boundary instead
   of silently losing explanatory evidence.
5. Finish the excerpt/staging caller audit so citations, semantic documents and runtime inputs
   use the same verified retained bytes. Audit producer/normalizer dependency identity,
   including the hardcoded `revision-source` versions in
   [revision.rs](../../crates/enrichment-daemon/src/ops/revision.rs).

**Check:** final output from representative larger inputs, plus focused rejection/cancellation
checks for the concrete unsafe allocation or malformed-value paths changed. Reuse existing source
integrity and artifact paging checks. Do not start a general memory benchmarking project here.

### AR3 — Close remaining semantic and projection gaps in the single query path

**Gap:** broad plan-level claims about ten-relation fidelity, metadata, qualified alternatives
and cross-operation equivalence exceed the currently demonstrated integrated coverage. Remaining
selection and hydration code needs a final consumer audit; a missing exhaustive matrix is not
itself evidence of a defect.

Source anchors: [query.rs](../../crates/enrichment-store/src/query.rs),
[projection/](../../crates/enrichment-store/src/projection),
[views.rs](../../crates/enrichment-store/src/views.rs),
[search_plan.rs](../../crates/enrichment-store/src/search_plan.rs),
[comparison.rs](../../crates/enrichment-store/src/comparison.rs),
[inspect.rs](../../crates/enrichment-daemon/src/ops/inspect.rs).

Remaining work:

1. Audit each live operation's selection boundary. Move any remaining corpus-wide eligibility,
   joins, grouping, ordering and paging into the store's native plans. Hydrate only selected
   results. Keep bounded DTO presentation and specialized scoring/canonicalization in Rust.
2. Check resulting library information across all ten relations for meaningful distinctions:
   absent versus empty, unknown versus failed/unattempted, source/stub/runtime alternatives,
   same-path kinds/qualifiers, local/external/unresolved references, and actual provenance.
   Correct losses at the canonical schema/projection boundary, not with consumer-specific defaults.
3. Complete actual consumer metadata rules where scans or derived expressions lose a required
   semantic role. Validate conflicting metadata on admission. Test the transformations used by
   the product rather than every theoretical Arrow/DataFusion composition.
4. Review final search, overview, inspect and comparison outputs against independent expectations
   for short/summary-only matches, Unicode/path separators, aliases, namespace membership,
   alternative observations, partial coverage and scope-specific differences. Preserve stable
   paging and access to alternatives when an inline answer cannot contain them.
5. Regenerate changed wire/tool/worker schemas and thin Python projections from Rust. Remove
   displaced representations and their callers in the same change.

**Check:** use and extend existing `typed_arrow`, `admission`, `scoring`, repository and real MCP
fixture journeys only where a missing output case or changed contract needs coverage. Expected
information and explicit limitations are the oracle, not parity with an old implementation's bugs.

### AR4 — Finish publication, durable delivery and reuse foundations

**Gap:** recovery coverage is concentrated on five diagnostic barriers and Resolve. The complete
Inspect/Verify delivery and same-context publication behavior remains unqualified. Comparison
also has a concrete failure path: `compare_job::run` delegates terminal delivery to `Jobs::finish`,
whose encoding/write error can leave the job running while the spawned caller only logs the error.

Source anchors: [repository.rs](../../crates/enrichment-store/src/repository.rs),
[catalog_generation.rs](../../crates/enrichment-store/src/catalog_generation.rs),
[publication_probe.rs](../../crates/enrichment-store/src/publication_probe.rs),
[leases.rs](../../crates/enrichment-store/src/leases.rs),
[delivery.rs](../../crates/enrichment-daemon/src/delivery.rs),
[jobs.rs](../../crates/enrichment-daemon/src/jobs.rs),
[compare_job.rs](../../crates/enrichment-daemon/src/ops/compare_job.rs).

Remaining work:

1. Finish the shared publication/terminal-result contract for every existing publishing job.
   Required delivery must be admitted before visibility; recovery reads committed bytes rather
   than rerunning a producer. Handle comparison and other terminal-delivery failures explicitly,
   including in-memory status and restart behavior when storage cannot persist a terminal record.
2. Close meaningful failure windows around manifest/file durability, snapshot visibility,
   catalog replacement and terminal journals. Extend existing interruption tests where needed
   to cover an uncovered state transition; do not require a new probe at every internal step.
3. Strengthen C07 to create a distinct later snapshot for the **same context** and preserve the
   old pinned snapshot. The current e2e test resolves another package and revalidates unchanged
   inputs without requiring that same-context transition.
4. Complete same-context/disjoint-contribution, stale-base and conflicting-observation handling
   with coherent catalog selection. Verify cleanup cannot delete active reader, stream, catalog
   or export inputs through cancellation, eviction and restart.
5. Finish independent export closure for inputs, actual attempts/logs, derived ancestors and
   promised delivery artifacts. Unknown recovery children, altered files and unresolved external
   cleanup must fail explicitly rather than disappear through cleanup.

**Check:** selected real daemon interruption/restart journeys, actual delivery/storage failure,
concurrent same-context enrichment and independent bundle verification. These protect visibility
and deletion semantics that inspection of a successful final answer alone cannot establish.

**Phase 1 exit:** the supported production path has one Rust-owned evidence model, one native
relational assembly/query path and one publication/reuse contract. Remaining functional journeys
may still need completion; they must use this path or report a precise unavailable capability.
This milestone is not full Plan 10/11 or product acceptance.

## Phase 2 — Decommission legacy and perform the development cutover

**Dependency:** Phase 1. Complete this phase before prioritizing additional or unfinished product
functionality. It must not depend on authenticated clients or a performance-tuning campaign.

### DC1 — Delete residual production surfaces and prevent reintroduction

Remaining work:

1. Remove test-only collection helpers from production exports. Current callers of the Rust and
   Python `normalize` collectors and `paths_by_definition` are tests; move any necessary fixture
   collection into test support over the real streaming producer interface. Do not retain public
   alternate corpus-building entry points because tests find them convenient.
2. Remove obsolete adapter behavior such as `normalize_into` silently dropping synthetic Python
   namespace symbols from the old producer. The target producer must emit its declared contract;
   malformed input must be explicit. Audit legacy-only field defaults/comments and overloaded
   old identity names without deleting meaningful nullability or producer-local references.
3. Close Plan 11 D1–D13 by imports and actual callers across binaries, supported profiles, recipes
   and tests. Delete remaining old readers, writers, indices, broad query helpers, raw production
   Parquet bypasses and obsolete installer/acceptance logic. Do not keep an engine-selection flag.
4. Add a small semantic deletion gate and scoped Rust ast-grep rules with negative fixtures.
   The current rules corpus has no Rust architecture rules. Guard actual forbidden entry points
   and ownership boundaries; do not globally ban JSON, maps, vectors or Arrow casts.
5. Finish the scoped reviews and decisions for ADR-0028–ADR-0032; their named review documents
   are absent. Reconcile the living design, ADR index/register and stale plan/status/operations
   references. Fold the resource-contract documentation into this reconciliation without
   reopening workstation tuning. Accepted ADR arguments and frozen enforcement files stay intact.

**Boundary:** raw artifacts, small visibility manifests, operational journals, bounded producer/
MCP DTOs, specialized kernels and test-local corruption helpers are legitimate. Delete a parallel
semantic or storage authority, not every non-Arrow data structure.

**Check:** no forbidden production caller or alternate path in the supported build graph; affected
build/lint/schema/rule checks; fresh ordinary acquisition/retrieval still works using only target
state. Do not make this phase wait for the Phase 4–6 functionality below.

### DC2 — Remove owned old evidence, snapshots and indices

Remaining work:

1. Complete an ownership-aware inventory under the nested development roots
   `.dev-state/{rn,rm,w1b,rj,lc,h,w1c,w1,rk,rl,ri,phase456-historical-capture}`.
   Read-only inspection still finds nested snapshot directories. Identify their actual formats
   and service ownership; directory names alone do not authorize deletion. Do not follow
   `*current` symlink aliases.
2. Reconcile the 43 older `.dev-state/p4p/owned` creator records against their canonical owners,
   broker storage and actual container state. Recover/remove confirmed-owned leftovers before
   deleting their journals. Do not conceal uncertainty by erasing the record.
3. Reconcile the service residue still present under the real XDG cache/data paths. Establish
   that each candidate is test-created, has no live owner, and contains no unrelated user state.
4. Apply one scoped development cutover with exact preview/apply inventories and locks. Remove
   old library evidence, snapshots, acquisition/selection indices and incompatible caches without
   backup/import or a legacy reader. Preserve unrelated credentials, source, diagnostic logs,
   design provenance and container/image storage outside the selected cleanup inventory.
5. Bootstrap fresh target state and verify ordinary acquisition, offline retained reads, separate
   cache/evidence cleanup and no real-XDG mutation.

**Check:** before/after owned-path and resource inventories, a fresh target-only start, and existing
isolation/cleanup checks. A single narrow cutover action is sufficient; do not build a migration
subsystem to manage disposable development formats.

**Phase 2 exit:** architecture implementation and legacy decommission are ready for subsequent
functional work. Every removal obligation has a concrete deletion or a justified boundary; no
old evidence/reader is required to run the service. Unresolved owned cleanup remains explicit.

## Phase 3 — Complete the remaining Phase 4 functional behavior

**Dependency:** Phases 1–2. Preserve the full required Plan 09 capability scope; finish it directly
on the target architecture, without incremental compatibility work.

### FN1 — Finish durable research and execution journeys

Remaining work:

- Complete cold/warm version comparison composition for one-side failure, shared acquisition,
  cancellation and restart. Audit callers for the assumption that Resolve always returns an
  immediate context. Carry pending jobs and overflow artifacts through to a usable final answer.
- Finish exact derived-context reuse for explicit ExecuteOnMiss: dependency-complete keys,
  ambiguity handling, changed inputs/producer/environment, eviction and restart. Default retained
  reads remain in the supplied context; do not copy facts between unlike environments.
- Complete semantic inspection and typed runtime-object behavior through actual ty/rust-analyzer
  and runtime consumers: supported methods, document coordinates, diagnostics, stale/partial/
  unsupported outcomes, source/stub/runtime disagreement and usable limitations. Resolve concrete
  output gaps found in these journeys rather than adding a generic inspection framework.
- Finish warm-session and execution ownership through abort, failed cleanup, idle eviction,
  retained-capsule integrity and restart. Required leases and reservations last through confirmed
  cleanup, not merely until the requesting future disappears.
- Complete latest/revision replay and producer attribution so refreshed selection or a new attempt
  does not regenerate unchanged documentation or corrupt acquisition-specific citations.

Sources: daemon [ops/](../../crates/enrichment-daemon/src/ops),
[lsp/](../../crates/enrichment-daemon/src/lsp), [execution/](../../crates/enrichment-daemon/src/execution),
`tests/e2e/test_{semantics_fixture,runtime_objects,verification_scope,evidence_retention,revision_fixture}.py`.

**Check:** reuse existing real producer/MCP journeys and inspect retained/replayed outputs, including
known locations, qualified alternatives and actual producer identities. Extend checks for a concrete
missing behavior, not every permutation of each protocol field.

### FN2 — Finish the specific Rust and Phase 4 acceptance gaps

Remaining work:

1. Correct R09: compile the **identical consumer snippet and dependency closure** with stable and
   the configured dated nightly. The current test pairs a nightly documentation build with only
   a stable consumer compile; that does not establish the required comparison. Keep exact toolchain
   provenance and truthful limitations, without adding arbitrary caller-selected shell/toolchains.
2. Extend C20 beyond the current Python-focused canary to completed Rust acquisition/fallback,
   semantic and verification operations, including failure/cancellation where relevant. Await the
   actual intended outcome before accepting unchanged repository bytes.
3. Finish requested Rust target/default/features/lock behavior and hosted-first qualified fallback
   on real inputs. Run the known-location Rust navigation assertions and remaining Python/Rust
   runtime and policy scenarios after repairing target-contract assertion mismatches.
4. Make execution fixtures consume the actual selected helper/broker/resource qualification
   contract. Several fixture config writers still use defaults while the selected receipt is
   configuration-bound. A cancellation-wrapper broker needs its own qualification; no copied
   receipt or softened readiness check.
5. Re-run unresolved real-producer scenarios on the final target source, including large Unicode
   documentation/oversized results and retained execution replay. A contained image alone does
   not establish these product behaviors.

Sources: [test_rustdoc_fallback.py](../../tests/e2e/test_rustdoc_fallback.py),
[test_canary_repository.py](../../tests/e2e/test_canary_repository.py),
[test_execution_readiness.py](../../tests/e2e/test_execution_readiness.py),
[test_operations.py](../../tests/e2e/test_operations.py), and the existing ignored execution tier.

**Check:** the original R09/R10, P08a/P08b/P09/P10 and applicable C10–C12/C16–C18/C20 scenarios,
with asserted final outcomes and cleanup. Reuse suitable existing tests; missing prerequisites
remain scenario-specific rather than suppressing the entire tier.

## Phase 4 — Complete client use and remaining Phase 5–6 operations

**Dependency:** Phase 3 for execution-dependent journeys; all work uses the Phase 2 target-only
architecture. Installation and client work do not delay the earlier decommission milestone.

### FN3 — Finish actual client integration

**Gaps:** the old client harness still uses tool-name substring matching and global prerequisites;
`scripts/client_events.py` is not integrated or covered by parser regression tests. The harness
does not establish the full daemon/Context7 setup, current CLI invocation contract or service-absent
scenario. The sandbox rejects every descendant of the operator's home, including isolated owned
service state. Current fixture prompts also reference unavailable/mismatched fixture APIs.

Sources: [client-acceptance.py](../../scripts/client-acceptance.py),
[client_sandbox.py](../../scripts/client_sandbox.py), [client_events.py](../../scripts/client_events.py),
[test_client_acceptance.py](../../tests/client/test_client_acceptance.py),
[skill_install.py](../../scripts/skill_install.py), [launch_configuration.py](../../scripts/launch_configuration.py).

Remaining work:

1. Connect native invocation/result correlation to generated envelope/tool-data validation and
   independent daemon context/snapshot/artifact checks. Require completed usable results; a
   pending call or a model's tool-name mention is not research completion. Delete substring logic.
2. Separate Codex, Claude and Context7 prerequisites by scenario. Treat a harness crash or missing
   summary as failure, not an authentication block. Finish supported CLI arguments, absolute
   launch configuration, daemon lifecycle and separate Context7 registration in isolated homes.
3. Make credential adoption explicit, minimal and per selected client, using the existing-login
   reuse scope in Plan 09. Preserve real user configuration. Fix isolated-root validation without
   allowing real client directories as destinations.
4. Repair prompts to target actual fresh fixtures; enforce breadth-before-depth where required,
   direct known-symbol lookup, exact versus unknown version support, polling/artifact retrieval,
   and an actually unregistered/unavailable enrichment service for A06.
5. Finish remaining installer recovery for initialization/intent temporaries outside the committed
   journal and integration with the complete native executable/adapter deployment. Keep one
   implementation for preview/apply install/update/uninstall.
6. Run real A01–A06 and Plan 09's additional upgrade/runtime journeys in both clients. Review
   the resulting briefs for useful supported conclusions, evidence IDs and explicit uncertainty.

**Check:** focused parser rejection cases followed by actual authenticated client transcripts and
real-user configuration checks. Parser fixtures and skill discovery cannot substitute for those
journeys. Do not create a second synthetic client acceptance path.

### FN4 — Finish operational product behavior

Remaining work:

- Complete actual two-adapter surviving-subscriber journeys for the remaining operations and
  same-context distinct enrichment. Reuse the existing durable acquisition test rather than
  implementing another single-flight system.
- Finish operational recovery, explicit cache/evidence cleanup and portable exports through the
  installed command surface. Cover closure and ownership failures that remain after Phase 1/2.
- Connect useful bounded query/admission/cache diagnostics to operational inspection, with scope
  distinct from library Coverage. Fill missing stage information only when it supports diagnosis;
  do not grow a monitoring platform or instrument every internal function.
- Finish operator instructions for setup, all required native binaries, qualification, retained
  evidence, upgrades, cache versus evidence removal, interrupted work and recovery. Reconcile
  service status and product-skill guidance with the final behavior and current configuration.

**Check:** representative fresh-install → acquire → research/verify → restart/offline reuse →
export → explicit cleanup journeys, with usable status and error messages. These are the remaining
Plan 09 W6 consumers of the architecture, not another storage implementation.

## Phase 5 — Integrated output assessment and final acceptance

**Dependency:** Phases 1–4. This is the final validation pass, not a prerequisite for each edit.

1. Assess completeness and usefulness of the final library outputs against exact selected inputs
   and known expected information. Check omissions, qualified alternatives, docs/examples/configuration,
   citations and evidence limits across both languages and two-release comparisons. Comparison probes
   may help diagnose an observed gap; implementing a new probe suite is not mandatory.
2. Run the relevant existing correctness and boundary checks, then current-tree `just ci`, schema
   conformance, dependency policy, rules, pinned-toolchain/provenance and state-leak checks. Run
   the ignored containment/cleanup tier, live producer and actual client scenarios explicitly.
   Do not repeat full suites between routine edits.
3. Assess the retained performance budgets on representative final workflows using the existing
   measurement tooling. Measure more deeply and tune only if the finished behavior exposes a
   material latency/resource problem. Keep resource bounds and truthful failures; do not make
   workstation optimization the critical path or change a budget silently after a failure.
4. Regenerate and check source/command/log-bound acceptance evidence. Audit weak assertions,
   stale test references and report promotion; compilation and output inventories do not establish
   a real producer/client scenario. Missing external prerequisites remain named blocks.
5. Finish scoped design/decision reconciliation and review the remaining G1–G7/O1–O12/D1–D13
   obligations at the strength of the actual evidence. Update STATUS and the execution ledger to
   the final remaining state, and stop with no task-owned processes or hidden cleanup obligations.

Architecture/deletion and full functional acceptance are separate milestones. Do not make the
former depend on client credentials or tuning; do not describe the latter as complete while a
required capability or actual acceptance journey remains missing.

## Coverage of the predecessor plans

This table maps remaining obligations only; it is not a completed-work report.

| Remaining source obligation | Address here |
|---|---|
| Plan 10 T1/T2 and O1–O4: canonical relations, ingestion, semantic fidelity and provenance | AR1–AR3; integrated output assessment in Phase 5 |
| Plan 10 T3–T5 and O6–O8: native selection/assembly, reuse, research and comparison | AR1/AR3/AR4; remaining durable product journeys in FN1 |
| Plan 10/11 O5/O9: coherent publication, ownership and bounded failure | AR2/AR4; remaining execution/operational consumers in FN1/FN4 |
| Plan 11 W1: source and coordinate trust | AR2/AR3; actual semantic journeys in FN1/FN2 |
| Plan 11 W2: pending composition, retained selection and delivery | AR4 first; remaining functional behavior in FN1 |
| Plan 11 W3/W4: residual relational work, representation and information preservation | AR1–AR3; targeted/integrated validation rather than exhaustive intermediate matrices |
| Plan 11 W5: publication, leases, recovery and export | AR4/DC2 first; installed operational journeys in FN4 |
| Plan 11 W6 and Plan 09 W2–W4: execution ownership, inspection and Rust/Python acceptance | FN1/FN2, after architecture and deletion |
| Plan 11 W8 and Plan 09 W5: installation and actual clients | FN3 |
| Plan 11 W9, Plan 10 T7 and D1–D13: deletion and owned state cutover | DC1/DC2, before functional expansion |
| Plan 09 W1's still-valid provenance/identity requirements | AR1–AR3/FN1; historical-reader/migration requirements remain excluded |
| Plan 09 W6: single-flight, same-context enrichment, export, pruning, metrics and guide | AR4/DC2 foundations; FN4 product completion |
| Plan 11 W7/O10: physical behavior, resource measurements and tuning | Correctness boundaries in AR2/AR3; performance assessment deferred to Phase 5 |
| Plan 10/11 O11/O12, Plan 11 W0/W10 and Plan 09 W7 | DC1 decision/removal inventory; FN2–FN4 actual journeys; Phase 5 final evidence |

No additional platform work is required by this plan. Delta, persistent text indices, custom
optimizer machinery, new secondary storage and Python Arrow IPC remain outside the required
scope unless a concrete subsequent need justifies them. Aggressive implementation here means
completing the chosen native design and useful product behavior directly, not preserving old
paths or adding speculative infrastructure.

## Outcome (recorded after implementation)

### What was built

**Implemented, with focused and actual-producer/client evidence recorded in the
[execution ledger](12-architecture-first-execution-ledger.md):** one Rust-owned typed
Arrow/Parquet evidence/catalog path; native relational assembly, semantic identity, selection and
comparison; bounded native/Python producers; coherent delivery/publication/recovery; exact retained
reuse; complete execution and client journeys; generational installer and operational commands.
D1–D13 legacy obligations and the owned development-state cutover are complete. ADR-0035's
exact-definition selection was added after actual output assessment found an unusable ambiguity.

**Functionally complete, 2026-09-14.** Full CI passed 390 Rust and 189 Python tests; the explicit
contained tier passed twelve, live registries passed two, and ten actual client journeys passed
(eleven pytest checks). The final overview correction passed its deterministic native regression,
real Python/MCP journey and full CI. Only two stale test-registry references changed afterward.
The user explicitly stopped redundant full-suite replay for that recording-only correction.
Completed logs remain evidence for their recorded implementation; they are not rewritten as
current-registry receipts. Final machine-report certification is intentionally not claimed. The
[integrated review](../design_review/reviews/design_review_plan12-integrated-completion_2026-09-14.md)
records scoped G1–G7 and O1–O12/D1–D13 judgments.

### A mistake made and corrected

Actual client runs exposed a stale pre-publication resolution summary, unselectable same-path
Rust definitions, a false rejection of restarted paged reads, and a teardown path which could
retain a duplicate successful scenario. The implementation now reconstructs the final summary,
selects typed definition candidates through DataFusion, retains documented overview summaries
when an undocumented stub sorts first, validates complete native cursor chains,
and records exactly one outcome after cleanup with persistent trace hashes. Earlier failures
remain in diagnostic logs; they are not claimed as passes.

### Deviations from the plan, deliberate

The user explicitly deferred attention to arbitrary timing targets and workstation tuning until
correctness, robustness and target functionality are complete. The existing measured median
(2.20897 seconds) remains above the historical 1.5-second target; no threshold was changed or
performance pass invented. Expanded performance campaigns and exhaustive intermediate matrices
were not required for this implementation. Targeted negative cases and actual final journeys
supply the validation described above.

The independent auditor reviewed assertions and architecture/removal closure, found no remaining
functional defect, and independently replayed all 390 Rust tests. The user stopped its redundant
Python replay after the registry-only correction; that audit is recorded as interrupted, not passed.
This is an explicit validation-scope decision, with no remaining implementation work inferred from
metadata-invalidated report entries.

No compatibility reader, historical import, additional storage engine or speculative optimizer
was added. Old development payloads were removed only after exact ownership and container
reconciliation; future validated evidence retains the indefinite version/context reuse contract.
