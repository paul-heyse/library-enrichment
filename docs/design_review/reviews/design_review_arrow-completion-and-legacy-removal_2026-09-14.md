# Code review: Arrow/DataFusion completion and legacy removal

## 1. Decision and scope

**Decision: Revise; completion is not established.** The principal Rust Arrow/DataFusion paths
are connected. The next work is targeted completion and deletion, not another storage rewrite.
The requested implementation sequence is
[Plan 11](../../plans/11-arrow-datafusion-completion-and-legacy-removal.md).

**Reviewer:** Codex. **Reviewed:** 2026-09-14. **Baseline:**
[Plan 10](../../plans/10-arrow-datafusion-architecture.md), accepted ADRs 0022–0027, current
working tree over `cd6d9490a6433ae355b04a465538062e1320544c`. Extensive uncommitted and
untracked implementation means the commit alone does not reproduce this review.

**Observable outcome:** one native evidence authority, bounded planned retrieval, reliable
retained evidence reuse, and no remaining selectable or implicitly used legacy implementation.
The user explicitly paused implementation for this review and new plan.

### Method and coverage

Read the charter/directive/addendum, prior Arrow reviews, Plan 10's T0–T7/F1–F13/O1–O12 and
deletion requirements, relevant accepted decisions, STATUS, measurement protocol and current
core/store/daemon source. Traced normalization/publication, runtime/provider construction,
native search/browse/comparison, exact inspection, artifact/source reads, durable resolution,
job delivery, leases/export, maintenance and client assertion/installation paths. Searched
Rust source for old bulk APIs, raw utility consumers and session/view overrides. Inspected
existing execution logs and a bounded inventory of development and known test-created state.

This is **source analysis plus inspection of dated evidence**. No new application tests,
benchmarks, reset, installation or runtime recovery were executed. Reported defects have
specific source paths and counterexamples, but those new counterexamples were not dynamically
executed. Complete O1–O12, real client assertions, producer crash matrices, resource exhaustion
and host ownership recovery remain unverified on current source. This is not a fresh gate tally
or an exhaustive line-by-line review of every producer/parser. Existing tests are not treated
as proof merely because their names suggest coverage.

Current structural evidence is distinguished from legacy code already removed and ordinary
non-Arrow boundaries. A JSON value, Vec, BTreeMap, MemTable or specialized kernel is not by
itself a second architecture.

## 2. Authority and lifecycle map

| Meaning | Owner/identity | Publication/lifecycle | Derived representation |
|---|---|---|---|
| Definitions, public bindings, observations and coverage | Rust typed evidence relations, snapshot scope and qualified IDs | One snapshot contract 5.0; admitted before selection | Native views and bounded tool DTOs |
| Release/environment/context/current selection | Rust relational catalog, catalog/2 generation | One durable visibility root | Request pins and disposable lookup/view caches |
| Actual attempts and inputs | Producer attempt, qualified input role/content identity, actual log | Closure checked with snapshot/catalog association | Export projection and diagnostics |
| Resolve/Inspect/Verify progress | Concrete Rust job specifications and journal concrete-jobs/3 | Publication precedes terminal delivery; restart reconciles committed results | Inline/overflow responses |
| Raw source bytes | Content-addressed retained artifact | Immutable retained bytes; explicit cleanup | Extracted source cache; currently insufficiently checked on reuse, N2 |
| Query resources | One shared RuntimeEnv plus bounded scoped sessions | Permits/streams/leases govern active use | Operational metrics; full plan metrics still absent |
| Development cutover | Operator-owned validated state roots | Preview/apply and execution recovery | No imported historical store |

Opaque behavior remains appropriate for archive/HTTP I/O, Griffe, rust-analyzer/ty, isolated
runtime execution, hashing, the narrow scoring kernel and transport rendering. They require
bounded inputs/results, explicit effects and exact provenance rather than a new declarative DSL.
Semantic snapshot identity excludes incidental encoding/batching/timing; physical digests bind
actual bytes. W4/W5 still need the full metamorphic/interruption proof.

## 3. Semantic contracts and invariants

| Contract | Current enforcement | Gap/failure behavior | Verification required |
|---|---|---|---|
| One storage/query authority | `repository.rs`, `admission.rs`, `catalog_generation.rs`, native plans | Public raw Parquet utilities remain exposed, although current callers are tests | D3 production reachability guard and negative fixtures |
| Evidence reuse preserves exact support | Catalog/snapshot/input identity and admission cache | Source-cache completion marker does not validate member content | N2 offline tamper and safe-member-read oracle |
| Retained coordinates identify original bytes | Typed positions/ranges and LSP adapter | LF-only splitting changes line interpretation; unsupported encoding can pass at column zero | N3 line/encoding truth table and real known-token navigation |
| Cold resolution is durable | `ops/resolve.rs` delegates cold work to resolve jobs | Compare consumes Pending as though a completed context must exist | N1 cold version comparison and subscription lifecycle |
| Bounded execution precedes DTO rendering | Runtime stream row/byte/deadline limits; native search/comparison pages | Some selection and producer/artifact bounds occur after allocation | N4 projection/peak-allocation cases |
| Committed evidence and terminal jobs agree | Native catalog job publication and startup reconciliation | Complete all-job crash/delivery matrix is not established | N10 process interruption/oversized-result recovery |

Unknown, empty, unsupported, failed and partial remain distinct. Missing evidence is not absence
of a capability. Physical nullable descendants do not relax logical required-value checks.
No compatibility reader is required for old development snapshots; fresh malformed/unsupported
format refusal remains part of validity.

## 4. Derivation and execution design

The source route is producer transport → Rust normalization → typed Arrow construction →
relational admission → immutable snapshot/catalog publication → admitted scoped providers →
native query plan → bounded batches → Rust DTO → thin MCP adapter.

`enrichment-core/src/evidence/ingest.rs:23–45` explicitly separates wide producer transport from
the normalized relations. `enrichment-store/src/dataset.rs:99–185` writes ten relations, but
accepts an already accumulated `NormalizedEvidence`. That establishes native storage, not
streaming ingestion from the beginning. W3 addresses the remaining allocation boundary.

`runtime.rs:89–110` constructs shared resources and scoped sessions. `search_plan.rs:224–232`
sorts and limits before hydration; `comparison.rs` uses native nested sets and scope joins.
`repository.rs:508` stages a native union rather than reopening all rows into the former wide
object graph. These are meaningful architectural gains: replacing them with collection loops
would lose planner visibility, bounded execution and coherent relational validation.

Publication/rebase, exact file providers and leases are implemented. Their complete operational
guarantees remain subject to W5/W7. Catalog roots/journals are control protocols, not competing
evidence databases. No evidence supports adding Delta or a custom optimizer framework now.

## 5. Representative journeys

**Ordinary extension:** another qualified execution observation should add a typed payload,
projection/admission rules and a producer mapping, then use the existing native contribution,
selection and export path. It must not introduce a JSON observation sidecar or Python query path.
The release/execution relations show this route exists; the ten-relation matrix still needs closure.

**Meaningful change:** a new version/context selects different evidence. An unchanged exact
context reuses validated facts indefinitely. Explicit execution may produce a qualified child;
W0 must settle exact discovery from a parent before ExecuteOnMiss can reuse it safely. Cache
eviction is not evidence invalidation; marker-only extracted-source reuse currently violates
the required byte fidelity.

**Alternate representation:** Parquet/view/dictionary encodings and native derived views must
preserve null/empty, qualified alternatives and metadata. Bounded DTOs are permitted output;
hydrating every fragment before choosing one is unnecessary representation expansion.

**Interruption:** a cold version comparison invokes resolve. The new resolver may return
Pending; compare currently reports an invalid input because no context is present. After actual
publication, recovery must use catalog job association rather than rerun the producer. Existing
crash tests cover part of this; complete Resolve/Inspect/Verify and large-delivery cases remain.

## 6. Acceptance gates

These are charter judgments, not R/P/C/A execution statuses.

| Gate | Judgment | Evidence and required action |
|---|---|---|
| G1 — Authority | Fail in source-cache boundary; native catalog route aligned | N2 allows mutable extracted bytes to stand in for retained artifact authority. W1 removes that authority gap; D3 is preventive removal, not proof of a live second store. |
| G2 — Semantic fidelity | Fail | N3 source coordinates use the wrong line model; unsupported encoding at zero can pass. Correct and test all boundaries. |
| G3 — Validity | Fail | N1 consumes a nonterminal resolve envelope under a completed-context assumption; N3 has inconsistent encoding validation. |
| G4 — Hidden behavior | Unresolved | Explicit retained/execution intent exists, but complete parent/child reuse, cancellation and source-cache lifecycle need W1/W2/W6. No blanket claim that inspection executes implicitly. |
| G5 — Consistency and recovery | Unresolved | Native publication/leases exist; all durable barriers, delivery overflows, combined resource failures and owned reset remain W5/W9. |
| G6 — Transformation and reuse | Fail for source reuse; broader matrix unresolved | N2 can change cited text on cache hit. O2/O3/O6 complete transformations and reuse require W1/W4/W5. |
| G7 — Truthful capability claims | Unresolved | Real production/client/performance acceptance is incomplete; current client substring oracle cannot establish actual successful calls. W6–W10 must supply matching-source evidence. |

## 7. Principle findings

Paths beginning with crate names are relative to `crates/`. Evidence is source analysis unless
explicitly identified as a dated executed log. Verification entries are required future oracles,
not claims that they passed during this review.

| Finding | Principle IDs | Concrete evidence or gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| **N1 — High: version comparison retains the synchronous resolver contract** | DM-07, DM-27, DM-30 | `enrichment-daemon/src/ops/compare.rs:57–71` handles only Error then requires context; `ops/resolve.rs` schedules cold resolution via `resolve_job` | Comparing two uncached releases returns `Resolution returned no context` while acquisition is pending | Compose existing durable jobs and audit every direct resolver consumer; delete any superseded synchronous route | Native/daemon cold+warm version-comparison test, one-side failure, polling and two-subscriber cancellation; W2 |
| **N2 — High: extracted cache contents can replace retained source authority** | DM-02, DM-33, DM-46 | `ops/source_cache.rs:37–38` accepts `.complete`; `ops/inspect.rs:537–543` passes cached member to `enrichment-core/src/producer/source.rs:228` ordinary File::open | Modifying a cached file, or substituting a nested symlink, can change an excerpt while its evidence still cites the original archive | Validate exact safe member bytes against retained authority; disposable reconstruction must not fetch/regenerate unchanged docs | Native same-size/content/symlink tamper tests and offline source inspection; W1 |
| **N3 — High: retained and protocol positions repeat an incorrect line model** | DM-06, DM-07, DM-24, DM-56 | `enrichment-daemon/src/lsp/document.rs:88–90,111–114,133–154`; core `Utf8Position::validate` also splits LF. `byte_position` returns at units==character before validating encoding | Standalone CR/repeated CR yield wrong retained locations; an unsupported encoding may succeed at column zero | Shared original-byte line scanner; explicit strict-admission versus protocol-normalization policy | Core/LSP table tests plus real known-location result; W1. Primary sources below |
| **N4 — Medium: some bounds are imposed after expansion** | DM-18, DM-26, DM-36, DM-39 | `ops/inspect.rs:216–276` hydrates fragments before aspect filtering; `ops/overview.rs:86–113` loads before take; `query.rs:328–356` returns full matching fragments; `ops/artifact.rs:49–83` loads blob/sections; `ingest.rs:23–45` precedes `dataset::write` | Small requests may decode unrelated large text and hit global limits; batch output bounds do not constrain earlier producer graphs | Typed native selections before hydration; bounded ingestion and artifact range reads; delete superseded broad helpers | Executed projection/limit assertions, independent outputs and peak-allocation workloads; W3/W7 |
| **N5 — Medium: raw fixture helpers remain a public production bypass** | DM-07, DM-57, DM-60 | `enrichment-store/src/lib.rs:20`; `parquet_io.rs:15,50` raw write and unbounded collect; current call search finds only tests | A future production caller can bypass admission/write bounds using the normal public crate API | Remove production export; test support retains explicit corruption construction | Scoped ast-grep rule/fixtures plus production API/import gate; W3/W9. Not classified as an existing live legacy query engine |
| **N6 — Medium: result overflow semantics have two implementations** | DM-25, DM-30, DM-56 | `ops/common.rs:249–339` and `jobs.rs:495–545` separately canonicalize, create blobs and build overflow envelopes with different bounds | Transport and durable responses can diverge in retained identifiers/limits; postcommit serialization failure may leave delivery unresolved | One bounded encoder/store policy with lifecycle-specific parameters and prepublication delivery admission | Oversized escaped/Unicode results, full paging, disk failure, restart and terminal-state assertions; W2/W3 |
| **N7 — Medium: physical leverage is configured but not demonstrated end to end** | DM-36, DM-39, DM-50, DM-59 | `runtime.rs:57–62,119–148` lacks operator/plan metrics; daemon `metrics.rs` reports request/fetch/probe counters; protocol's real workload pair and full measurements remain open | Cannot establish selective scan/pruning, bounded RSS or required latency; tuning risks increasing cost without benefit | Executed-plan diagnostics, full-stage workloads and measured physical layout decisions | W7 fixed budgets, O9/O10 resource/concurrency tests and source-bound measurements |
| **N8 — Medium: hard-cutover inventory exceeds the implemented reset scope** | DM-29, DM-35, DM-51 | `maintenance.rs:99–125` accepts only checkout `.dev-state` and selects top-level cache/data; nested fixture roots persist; p4p has 43 older ownership records; known real-XDG residue exists | Old development evidence remains, and indiscriminate deletion could hide uncertain execution ownership | Exact nested inventory, actual ownership recovery, scoped preview/apply and D1–D13 closure | W9 before/after inventory, active/uncertain-owner refusal, fresh bootstrap/state-leak checks |
| **N9 — High acceptance gap: install/client paths still cannot prove target delivery** | DM-29, DM-35, DM-59, DM-60 | `scripts/install-skill.sh:42,60–77` source-text ownership and in-place replacement; `tests/client/test_client_acceptance.py:59–79,111–112` global skip and stdout substring matching | Interrupted update removes installation; repeated tool names can look like calls; one missing client skips unrelated runnable scenarios | One locked atomic installer and correlated real event/results; per-scenario prerequisites | Installer failure/ownership oracles plus actual A01–A06; W8 |
| **N10 — High completion gap: final connected-producer/recovery proof is missing** | DM-30, DM-48, DM-59 | Real-sweep log records 24 passed/3 failed; later corrected tests/helper/replay edits unrerun; full all-job crash and same-consumer R09/Rust C20 proof absent from reviewed evidence | Focused green source tests could be misreported as complete Phase 4–6 or target acceptance | Finish producer qualification, remaining scenarios, matching-source receipts and independent assertion review | W2/W5/W6/W10; actual runtime/client gates, no fabricated receipts or inherited passes |

LSP recognizes LF, CRLF and standalone CR; retained ranges must use that line model. Its Position
prose also specifies how oversized character offsets are normalized. These are separate from
the service's stricter retained-byte admission policy. Verified 2026-09-14 against
[text documents](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/textDocuments.md)
and [Position](https://raw.githubusercontent.com/microsoft/language-server-protocol/gh-pages/_specifications/lsp/3.17/types/position.md).
The [compatibility matrix](../../architecture/compatibility-matrix.md#lsp-line-endings-and-exact-retained-source-coordinates-2026-09-14)
records the 3.17/3.18/reference-implementation differences; no universal line-clamping claim is made.

**Applicability:** authority, validity, provenance, execution, reuse, representation, performance
and verification bear directly on this review. No numerical/scientific approximation or future
graph-platform requirements were introduced. No dimension score offsets a failed gate.

## 8. Alternatives and architectural leverage

| Alternative | Duplication and locality | Correctness/operational risk | Cost and evidence | Decision |
|---|---|---|---|---|
| Current connected implementation | Core relations/native plans unified; residual expansion/response/coordinate rules remain | Findings N1–N10 and unclosed oracles | Large amount implemented; focused checks, incomplete performance/live acceptance | Continue targeted repair |
| Complete Plan 11 | One admission/publication/query authority with bounded boundary mechanisms; explicit deletion ledger | Requires complete crash/resource/real-client proof | Reuses target code; measurable increments and removal in the replacement changes | Selected |
| Simpler: retain native store, keep small bounded producer DTOs and ordinary byte I/O, remove optional physical tuning | Less representation machinery; avoids forcing journals/blobs/scoring into tables | Viable only if hard latency/resource/semantic requirements still pass | Baseline configuration is preferable wherever measurements show extra tuning adds no value | Adopt this simplification within Plan 11; not a waiver for failed budgets or unbounded graphs |
| Add Delta/second engine or compatibility bridge | Adds storage/version authority and graph maintenance | No demonstrated current consumer; conflicts with requested hard cutover | No current compatible/beneficial deployment evidence in this review | Do not add |

DataFusion already supplies the needed selection, joins, set operations, aggregates and windows.
The useful expansion is applying those operators before payload hydration and instrumenting
their actual execution. A generic schema compiler, custom optimizer suite or graph layer would
not solve the identified defects more directly.

## 9. Verification and measurement plan

| Claim | Evidence strength now | Required conditions/oracle | Next work |
|---|---|---|---|
| Native storage/retrieval connected | Implemented by inspected source; dated focused logs | Current source semantic roundtrip plus actual MCP requests | W3/W4/W10 |
| Reuse preserves exact support | Source counterexample; not dynamically attacked here | Cache mutation/link cases, exact derived scope, restart/eviction without extraction | W1/W2/W5 |
| Metadata and variants preserved | Partial focused evidence | O1–O4 full relation/operation/representation matrix and malformed inputs | W4 |
| Publication/recovery coherent | Implemented; some dated crash tests | Every durable boundary, three job kinds, concurrent publishers, large delivery | W2/W5 |
| Resource/performance contract satisfied | Unestablished; earlier target diagnostic exceeds budget | Fixed five-run protocol; real/shape/concurrent workloads, scan metrics, pool/RSS/spill | W7 |
| Fully removed legacy architecture | Major old modules absent; residual ledger open | Caller/build/profile audit plus independent behavioral guards and owned state reset | W9 |
| Full client/producer acceptance | Incomplete; known failed/unrerun scenarios | Actual correlated clients, current qualified images/helpers, stable/nightly and canaries | W6/W8/W10 |

Cost accounting includes producer graphs, Arrow buffers/writers, admission, physical-plan creation,
scan buffers, payload hydration, serialization, spill, publication/fsync, catalog growth and export.
The managed query pool is not an RSS ceiling. No migration testing of historical snapshots is
required; malformed/unsupported old-state refusal and safe reset replace it.

## 10. Exceptions and unresolved decisions

No new SHOULD exception or scope waiver is accepted here. W0 must settle durable version-form
comparison and exact derived-context discovery through existing concrete contracts. LSP response
normalization versus retained strict validation must be accurately documented. The architecture
maintainer owns these decisions before W1/W2 changes, with a new ADR if binding semantics change.

Delta, custom optimizer nodes and persistent secondary indices retain Plan 10's compatible-graph
and measured-consumer triggers. This review does not activate them. G4/G5/G7 and complete O1–O12
remain unresolved; supported claims must stay at their actual evidence strength.

## 11. Decision and implementation changes

**Decision: Revise.** Keep the connected native architecture and finish it. Removing all JSON
or all collections would delete valid boundary mechanisms while leaving the actual semantic
defects untouched. Remove the specific superseded authorities/paths and prove both exits.

| Priority | Change | Principles | Acceptance evidence / regression control |
|---|---|---|---|
| First: correctness | N1/N2/N3/N6 and exact reuse decisions | DM-02/07/24/30/33/46 | W1/W2 adversarial source, coordinate, durable comparison and delivery tests |
| Next: native execution and deletion | N4/N5, D1–D13 | DM-18/26/36/56/57/60 | W3/W4 native plan/semantic tests; W9 import/profile and owned-reset guards |
| Next: recovery/product proof | N8/N9/N10 | DM-29/30/35/48/59 | W5/W6/W8 real interruption, ownership, producer and correlated-client oracles |
| Before completion: measured behavior | N7 and current evidence reconciliation | DM-39/50/59/60 | W7 fixed budgets, W10 matching-source full gates and independent review |

The review and plan are documentation deliverables. No finding is marked fixed by writing it
down, and no architecture or deletion exit is certified here.
