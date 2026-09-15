# Bounded comparison proposal review

## 1. Decision and scope

Codex, 2026-09-13. Proposed contract review of ADR-0014 against blueprint §§3,6,7,13,
DESIGN §§5.2,7.1,7.3, current identity/evidence/query/request models and MCP comparison stub.
Read the charter authority, absence, publication, inspection, reuse and capability requirements.
Runtime probes, isolation and client acceptance are later required work, not evaluated here.

## 2. Authority and lifecycle

Rust request/data types govern the generated schemas. Immutable input snapshots govern the
comparison; a deterministic typed projection carries deltas, interpretation and provenance.
No comparison result mutates the meaning of either context or stored snapshot.

## 3. Semantic contracts

Context-pair and version-convenience requests are exclusive forms. Environment differences and
unknowns remain explicit; no identical-unknown inference. Path additions/removals, annotation
changes and behavior notes are independent axes. Missing API cannot produce a clean API verdict.
The envelope retains the six evidence classes; comparison interpretation is derived, not executed.

## 4. Derivation and execution

DataFusion reads canonical records through existing typed queries; Rust calculates changes.
Complete serialized responses determine budget fit. Overflow uses whole-entry pagination or
artifact references. Cursors bind snapshots, scopes, ordering and budget. No new dependency.

## 5. Representative journeys

An added Python function appears without a break. An unchanged Rust API still surfaces a changed
behavior note. Different default features/toolchains yield confounded comparisons. A pinned old
snapshot remains comparable after newer publication. Oversized signatures remain retrievable.

## 6. Gates

| Gate | Proposal verdict | Evidence / acceptance obligation |
|---|---|---|
| G1 | pass | Rust-owned contract, generated projections; schema conformance |
| G2 | pass | Separate absence/confounders/change axes; missing API and unknown environment tests |
| G3 | pass | Exclusive request forms and scope/cursor validation |
| G4 | pass | Pinned path never fetches or executes; offline fixture |
| G5 | pass | Snapshot pair immutable; newer publication regression |
| G6 | pass | Cursor binds all comparison inputs; changed-scope rejection |
| G7 | pass | Potential-break labels, no compatibility promise; additive/behavior fixtures |

Proposal verdicts do not establish implemented behavior.

## 7. Findings and applicability

| Finding | Principles | Required correction | Oracle |
|---|---|---|---|
| A signature diff alone misses capabilities and behavior | DM-08, DM-43 | Independent API-set and release-note deltas | P06, R08 |
| Different configurations can masquerade as release changes | DM-08, DM-32 | Explicit comparability/confounders | C15 |
| Payload-only counting exceeds wire budget | DM-14, DM-43 | Measure complete serialized envelope | C02/C03 adversarial payloads |

Authority, absence, immutable reads, reuse and truthful interpretation apply. Numerical kernels
and runtime isolation are outside this contract change; they remain relevant to Phase 4.

## 8. Alternatives

A breaking-only external diff loses additions and behavior notes. A new universal graph adds
unnecessary authority and migration work. Existing typed snapshots plus a Rust comparison are
sufficient and preserve the product's local evidence scope.

## 9. Verification plan

R08/P06 and C01-C03/C08-C09/C15 fixtures, schema conformance, Rust/Python checks, Phase 3 gate,
independent acceptance reruns. Actual client outcomes remain Phase 5.

## 10. Exceptions

No binding-boundary exception. Compatibility interpretation is explicitly limited to normalized
observations; semantic and runtime compatibility require later exact-environment probes.

## 11. Decision

| Priority | Verdict | Strength | Condition |
|---|---|---|---|
| 1 | Accept proposed comparison contract | Proposed | Negative/missing/configuration/budget oracles must pass before acceptance |
