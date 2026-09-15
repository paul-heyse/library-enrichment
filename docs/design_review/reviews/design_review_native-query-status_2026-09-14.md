# Native query status — scoped design review

## 1. Decision and method

**Accept-scoped**, 2026-09-14. Reviewed ADR-0034, Rust `NativeQueryCounters`,
`QueryRuntime::operational_counters`, daemon status, generated contracts and the real
`test_the_counters_track_what_they_claim_to` consumer. `.dev-state/plan12-ops-final.log`
records 23 passing focused operational/installer/parser tests. This review establishes the
bounded status projection, not full Plan 12 acceptance or whole-process memory performance.

## 2. Authority and lifecycle

The existing store runtime owns physical execution totals and admission permits. Rust core owns
the status DTO; Python uses generated schemas. Counters reset at process restart. No new retained
library facts or alternate query path exist.

## 3. Semantic contracts

Executions are recorded when their physical-plan trace drops. Completed and incomplete remain
separate; failures before plan construction are excluded explicitly. Admission is an instantaneous
permit count. Summed overlapping durations are not wall time, and configured byte limits are not
measured consumption. Null means no open runtime, not zero work.

## 4. Derivation and execution

Read one bounded counter snapshot and the runtime's configured limits; hydrate a fixed scalar DTO.
Status performs no evidence scan or additional query. Concurrent scalar readings are diagnostic
observations, not an atomic scheduling or accounting decision.

## 5. Failure and ownership

Runtime counters remain process-local. This projection acquires no producer, reader or publication
ownership. Missing runtime state is explicit; existing response budgeting still applies.

## 6. Gates

| Gate | Result | Evidence |
|---|---|---|
| G1 Meaning and authority | pass | Core DTO and store counter ownership; explicit operational scope |
| G2 Identity and provenance | pass | Process lifetime and timing meanings stated; no evidence IDs invented |
| G3 Contracts and boundaries | pass | Rust schemas regenerated; schema-conformance passed 2026-09-14 |
| G4 Lifecycle and failure | pass | Null unopened runtime; incomplete execution retained separately |
| G5 Derivation and reuse | pass | One projection of existing counters; no new persistent state |
| G6 Verification and observability | pass | Actual acquisition moves counters and releases permits |
| G7 Simplicity and evolution | pass | Fixed optional object, no new MCP tool or plan export |

## 7. Findings and applicability

| Severity | Finding / principle verdict | Oracle |
|---|---|---|
| resolved | Operational counters were hidden from the operator; bounded status now exposes them | Real operational metrics test |
| limitation | Counters exclude pre-plan failures and are not an atomic accounting ledger | DTO documentation and ADR-0034 |

Meaning, authority, representation, failure and measurement principles apply to this projection.
Scientific modeling, new evidence identity, multi-authority conflict resolution and production
migration mechanisms are outside its scope; no assurance about them is inferred here.

## 8. Alternatives

Keeping counters test-only fails FN4. Returning recent full plans in routine status adds payload
and potential source details without helping this immediate operational consumer.

## 9. Required work

No blocking defect remains in this projection. Full-tree/client/performance acceptance remains
separately tracked by Plan 12 and must not be promoted from this scoped check.

## 10. Verification strength

**Tested** for the named real daemon consumer and regenerated schemas. No peak-RSS claim or
performance-budget pass follows from these counters.

## 11. Decision

| Scope | Verdict | Follow-up |
|---|---|---|
| ADR-0034 bounded native query status | Accept-scoped | Include in final Plan 12 integrated checks |
| Full product acceptance | unresolved here | Plan 12 final evidence and actual clients |
