# Plan 13 implementation closing review — 2026-09-15

## 1. Decision and scope

**Proposal:** complete the DataFusion research-operation hard pivot prescribed by
[Plan 13](../../plans/13-datafusion-research-operations-hard-pivot.md), replacing the original
review's competing scope, delivery and recovery paths.
**Reviewer:** implementation agent; acceptance commands receive a separate auditor replay.
**Revision:** source `224dea96aca19e8482b6715f524ebe0b5948ba039e0c8fac00200aa933d72bf5`, research/2.0,
DataFusion 55.1.0, Arrow/Parquet 59.3.0 and FastMCP 4.0.3.
**Decision:** accepted for the implemented Plan 13 scope, after independent final-source
qualification and the verified fresh-generation production activation at 13:49:55 UTC.

### Method and coverage

Reviewed the actual shared scope relation, independent Arrow requirements, native class-scope
assessment, operation-index ownership, comparison reconciliation/publication, job failure recovery,
MCP output projection and installed launch selection. Read the corresponding negative, boundary
and lifecycle assertions. The independent final replay passed 432 native, 223 Python, 12 execution,
2 live and 15 installed-client tests. Inspected the retained live family/focused and equal-result
measurement receipts, deployed smoke and identity/preservation checks.
The exact upstream capability findings are documented in
[the source supplement](../../architecture/plan13-current-upstream-verification.md).

This review attacks missing scope, incompatible qualification, alternative loss, role/nullability
loss, exhausted managed-memory/output/spill quota, timeout ownership, interrupted publication and client error
presentation. It does not establish arbitrary filesystem fault tolerance, crash consistency under
faulty hardware, full build-script/source closure, process RSS caps, universal package semantics
or statistically reliable tail latency. Those are not claims of this implementation. Required
installed client and activation evidence passed and remains linked below.

## 2. Authority and lifecycle map

| Fact | Semantic identity / owner | Revision and update boundary | Derived representation |
|---|---|---|---|
| Evidence and source qualification | Rust immutable snapshot, FactSource and producer bindings | snapshot/projection 6.0; validated publication | Arrow/Parquet and native views |
| Requested sufficiency | Rust requested kind/subject/producer-binding relation | One selected snapshot/environment and requested scope | `ScopeAssessment`, research status |
| Request and resource policy | Rust operation descriptor and captured config digest | Foreground operation or independently owned durable job | Child query identities, budgets, retained leases |
| API selection and changes | Rust typed aspect/key/alternative cursors | Snapshot, selection and ordering identities | Native pages, tagged complete values |
| Durable result | Rust blob/index and publication catalog | Complete result closure before catalog/5 commit; jobs/5 reconciliation | Delivery descriptor and read-only indexed recovery |
| Client presentation | Authored FastMCP model over generated Rust domains | Validate original arguments and actual output | Structured envelope, bounded preview, resource link |
| Deployment | Explicit installed executable set and fresh state/6 root | One supervised generation, old root inactive | Absolute daemon/adapter/worker launch and hashes |

Scoring, archive decoding, producer invocation and byte encoding remain ordinary bounded code.
They do not establish an alternative schema, semantic planner or job authority. Re-serialization
can change presentation bytes; distinct normalized identities and qualified sources remain
explicit. Reordering physical input files does not authorize a new semantic observation.

## 3. Semantic contracts and invariants

| Invariant | Representation and boundary | Failure behavior / oracle |
|---|---|---|
| No evidence row is not a proof of absence | `coverage.rs::evaluate`: requested-domain LEFT JOIN, qualified state rank and witness | Unknown/partial/missing remain distinct; requested-coverage and cold/offline fixtures |
| Every declared source participates in class scope | `semantic_scope.rs`: qualified semi join, FILTER counts and UNNEST bases | Unknown/conflicting bases cannot establish nominal support; native scope fixture plus actual ty/runtime journey |
| An executable plan still must satisfy its consumer | `preparation.rs::QueryFamily`, Arrow name/type/nullability/role checks before and after preparation | Typed contract failure with bounded offending IDs; independent-family and invariant-witness runtime tests |
| Pages preserve distinct retained identities | Native total order, independently bound aspect/side cursors | Foreign cursor rejection; 97-alternative fixture and 117-relationship installed traversal |
| Delivery cannot change the research outcome | `DeliveryDescriptor`, compact `JobResult`, complete result closure | Genuine budget failure differs from artifact delivery; raw-stdio, Unicode and interrupted-publication oracles |
| Resource ownership outlives cancelled awaiting code | Operation context, shared permit, native reservations, captured leases | Blocking-reader/sink timeout tests; quota released only after last reader/worker |

Distinct comparison uses null-equal set semantics; it does not preserve bag multiplicity or infer
source compatibility from compiler-rendered signatures. Alias folding uses complete source and
definition qualification; equal prose from independent sources remains separate. A source window
states its actual extent. Revision omissions do not establish complete build inputs.

## 4. Derivation and execution design

| Stage | Native implementation and prerequisites | Effects / invalidation |
|---|---|---|
| Admission | Exact files, schema identities, key/reference witnesses | Reject corruption before evidence visibility |
| Scope and selection | Native expressions, joins, windows, aggregates and explicit supported domains | Same semantic path for candidate and retained answers |
| Preparation | Configured default analyzer → optimizer → configured physical planner | No double optimizer pass or empty custom-only analyzer; actual bounded plan/function/config inventory |
| Reuse | Operation-local search score/key, overview namespace and changed-key indexes | DataFusion DiskManager/SpillFile plus Arrow IPC and StreamingTable; no persistent semantic cache |
| Execution and delivery | Bounded Arrow batches to selected values/indexed result sections | Native memory/disk quotas plus cumulative output/artifact charges and deadline |
| Publication/recovery | Complete artifact closure before comparison/snapshot publication | Catalog commit wins late cancellation; restart reads committed bytes without another producer/query/write |
| Adapter/client | Strict original JSON and generated domain validation, explicit effects | Optional progress/resources cannot change an outcome; error text carries native recovery |

DataFusion owns relational planning and resource-managed execution. The small preparation
boundary supplies application-specific requirements the native type checker intentionally does
not prove. Scalar UDF return fields preserve role/nullability. A custom analyzer rule, logical
node, UDTF or ConfigExtension has no unresolved current consumer requiring it; the recorded
adoption triggers in ADR-0038 remain applicable.

The operation index is a local physical reuse boundary. Its managed reader reservation includes
Arrow decode workspace and the exact release's 128 KiB spill reader buffer. Drop behavior is
proven independently of measured latency. The selected relationship endpoint semi join was
measured against four keyed joins plus deduplication; the hash-join alternative did more work on
all three sampled symbols. That is a bounded-page result, not a universal optimizer claim.

## 5. Representative journeys

**Extension:** adding a research aspect requires one Rust selection/sufficiency choice, a native
query family and output variant, then generated domain schemas and an authored presentation
composition. It does not add a Python evidence query or another coverage rule.

**Meaningful change:** comparing DataFusion 54.1.0→55.1.0 with documentation and release notes
reconciles flat values first. The live answer preserves 24 changed documentation keys while
reporting release-note missingness independently on both sides. Rendered API differences remain
qualified observations, not an automatic breaking-change conclusion.

**Alternate representation:** a large answer retains its original outcome in an immutable result
artifact. Direct section reads advance using encoded byte counts. The exact section content and
cursor chain can be independently re-read; a partial section does not purport to verify the full
artifact hash. MCP keeps canonical structured output and one small portable projection.

**Failure:** an unavailable exact release creates a durable failed job. Claude's native trace
originally discarded structured error content, leaving a generic status summary. The adapter now
projects the native diagnostic and full artifact read action into bounded text. An artifact-write
failure supplies its inline diagnostic without inventing an artifact. Real raw-wire and unit
boundary checks and actual installed recovery pass, with independently read native journal and
artifact witnesses for both clients.

## 6. Acceptance gates

These are independent charter verdicts, not machine acceptance-registry states.

| Gate | Final verdict | Evidence |
|---|---|---|
| G1 — Authority | pass | Shared Rust scope/query/publication ownership; thin authored adapter composition |
| G2 — Semantic fidelity | pass in reviewed operations | Qualified alternatives, exact scope/side distinctions and complete bounded live navigation |
| G3 — Validity | pass in reviewed native/stdio boundaries | Independent Arrow family contracts, invariant witnesses, strict inputs and actual output validation |
| G4 — Hidden behavior | pass | Explicit acquisition/execution/cancellation effects; bounded optional presentation; static profile remains usable |
| G5 — Consistency and recovery | pass in tested boundaries | Native ownership/precommit/restart, final Python fault/export, five publication SIGKILL barriers and installed failed-job recovery all passed |
| G6 — Transformation and reuse | pass | Native reconciliation, qualified alias folding, measured operation indexes and last-reader quota release |
| G7 — Truthful capability claims | pass for the service contract | Final source, actual clients and sole-generation activation verified; partial evidence and client prose limitations remain explicit |

## 7. Principle findings

| Finding | Principles / verdict | Concrete evidence and consequence | Correction / executable oracle |
|---|---|---|---|
| Competing sufficiency and delivery interpretation replaced | DM-01/02/08/11/41/42: aligned | `coverage.rs::evaluate` is shared by all research renderers; `JobResult` separates outcome from storage location | `requested_coverage` fixtures; comparison partial-scope and raw-stdio assertions |
| Native executability alone no longer certifies application validity | DM-07/22/43/53: aligned in used families | `QueryFamily::require` rejects role/type/shape loss even when a plan can execute | `independent_family_requirement_fails_before_execution_with_its_own_trace`, native field and witness tests |
| Client error presentation had hidden a valid recovery path | DM-19/29/30/59: corrected and verified | Real Claude failed-release trace had retained generic error text and dropped structured output; the new bounded native projection enables recovery | `test_research_stdio`, `test_error_preview`, actual installed `recovery-claude` and native journal witness |
| Work ownership now includes blocking result work | DM-14/28/35/54: aligned in tested boundaries | A cancelled await cannot release query admission while its worker still owns output or index bytes | timed-out blocking reader/sink, shared materialization budget and index last-reader tests |
| Deployment closure verified | DM-14/30/39/54/59: aligned | One new supervised generation serves research/2.0; old state/config remain inactive and unchanged | Old adapters/daemon stopped; fresh state, matching installed launch, J20 smoke and process/config hashes passed |

Applicability: authority, fidelity, validity, derivation, lifecycle, provider boundaries and
truthfulness groups all bear on this change. No numeric solver, optimization objective or
approximate numerical equality is introduced. Comparative cost claims are limited to the measured
workloads. No aggregate score is used to offset a failed invariant.

## 8. Alternatives and architectural reuse

| Alternative | Duplication / locality | Cost and risk | Disposition |
|---|---|---|---|
| Keep old handlers and raise presentation caps | Independent coverage and indivisible result interpretation persist | High-fanout failures move to larger inputs; recovery remains host-dependent | Rejected by hard-pivot mandate |
| Shared typed operations with native indexed reuse and immutable delivery | Domain decisions remain Rust-owned; Python composes presentation | Small physical index/lifetime boundary, measured and tested | Selected |
| Shared typed operations with only lazy views and no operation index | Slightly less index code | Repeats expensive score/key/namespace work across count/page consumers; cannot assume a view caches results | Simpler viable alternative, rejected where measured repeated work justifies local materialization |

Persistent caches, another job engine, a schema DSL or a custom query node would add authority and
invalidation burdens without a current consumer. Ordinary trusted SQL, typed expressions,
source decoding and byte encoding stay ordinary code. The chosen abstractions each replace an
observed repeated semantic or physical boundary.

## 9. Verification and measurement

See [live outcomes](../../reports/plan13-live-functional-outcomes-2026-09-15.md),
[measurements](../../reports/plan13-native-operation-measurements-2026-09-15.md),
[removal evidence](../../reports/plan13-removal-evidence-2026-09-15.md) and the
[execution ledger](../../plans/13-research-operations-execution-ledger.md).

The independent final native replay records 432 passed tests. Installed raw-MCP
campaigns record exact 55.1.0 versions and useful payloads, not just envelope validation. The
40-crate campaign belongs to the preceding candidate with identical native binaries; the current
adapter revision has its own focused installed receipt. The 135-task one/eight-client experiment
records equal data digests, sample latency, native aggregate deltas and sampled process memory.
It does not manufacture per-stage latency from aggregate counters or call a small sample a
reliable p95. Wider throughput work has an observed latency/memory regression trigger.

A real native sort under a 128-byte managed pool also passed: capacity failure retains operation
correlation, reservations/admission return to zero, and a subsequent query succeeds. Oracle:
`native_memory_pressure_preserves_owner_and_releases_admission`. This adds no runtime mechanism.

Final stable-source Python, live registry, installed clients and containment receipts were
independently replayed and audited. The final state-leak check and actual deployed smoke passed.
The generated acceptance report has 47 passed active gates, no failed/blocked gates and retired
P08 as not_run. Historical failed receipts remain intact and are not promoted by later checks.
See [final qualification](../../reports/plan13-final-qualification-2026-09-15.md) for exact
selection hashes, deployment identities and the independent per-gate audit path.

Manual client review found overbroad wording about a missing registry release and a stale
process-readiness observation taken during acquisition. Native evidence preserves the narrower
facts. This review accepts the service contract; it does not certify every generated client sentence.

## 10. Exceptions and unresolved decisions

No MUST exception or narrowed product scope is accepted. W12 and J17/J19/J20 are closed by the
final receipts. Full build/source closure, custom planner extensions and broader throughput
machinery retain their explicit ADR adoption triggers. They do not waive a present functional
or boundedness exit. Production preserves its existing static-only execution policy; qualified
execution is independently proven in the recorded isolated roots.

## 11. Decision and implementation changes

| Priority | Completed action | Closing evidence |
|---|---|---|
| 1 | Completed real installed error recovery and stable-source fault/export checks | Independent final command receipts and native client witnesses |
| 2 | Performed one supervised fresh-state activation with matching installed components | Seven deployed smoke cases; process/config/input digests; old evidence unchanged |
| 3 | Reconciled D01–D12/J01–J20 and independent acceptance with final state | Final qualification report, ledger, operations guide, plan outcome and STATUS |

**Accepted.** The architecture replaces the reviewed defects and all required runtime/deployment
exits passed within their declared scopes. Remaining limits and adoption triggers are explicit.
