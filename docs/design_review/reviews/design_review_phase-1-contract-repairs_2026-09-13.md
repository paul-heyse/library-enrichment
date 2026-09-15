# Phase 1 contract repairs

## 1. Decision and scope

Proposal-stage review by Codex, 2026-09-13. Scope: R08/P06 phase assignment, declared feature
knowledge, absent observed configuration, snapshot-bound replay and normalization provenance.
Method: inspected blueprint §§3, 6, 8, 13, the charter, current Rust identity/retrieval/publication
code, and the failing fixture assertions. No claim about Python extraction or execution readiness.
The existing three failures were reproduced before implementation.

## 2. Authority and lifecycle map

| Concept | Authority | Boundary |
|---|---|---|
| Gate phase | Mutable gate registry under ADR | IDs/assertions retained from frozen plan |
| Feature selection | Rust Environment features plus features_known | Knowledge participates in new identity; old serialized IDs retained |
| Observed build | Optional Rust ObservedConfiguration | None means no observed build, not a fabricated target |
| Resolution replay | Stored resolution snapshot reference | Never substitute the mutable current pointer |
| Producer run | Daemon acquisition | Completed normalization recorded before immutable publication |

## 3. Semantic contracts and invariants

Omitted and explicitly empty feature selections differ. Legacy empty records cannot establish
whether the caller declared emptiness. Metadata-only snapshots report missing public API;
unknown observed configurations stay null. Root envelope behavior is unchanged.

## 4. Derivation and execution design

Generate boundary schemas from Rust. Bump normalizer identity for changed publication semantics.
Keep legacy context IDs and manifests readable. Validation compares temporary generated output,
without editing tracked files. Gate reassignment changes scheduling, not acceptance assertions.

## 5. Representative journeys

An explicit empty list survives resolution and inspection. A target-only request keeps features
unknown. A legacy record reads without identity rewriting. A hosted-JSON failure still publishes
source evidence. A newer current snapshot does not change a replayed resolution's evidence.

## 6. Acceptance gates

| Gate | Verdict | Evidence or scope rationale | Required action |
|---|---|---|---|
| G1 | pass | Rust and registry authorities retained | Generate projections |
| G2 | pass | Knowledge flag and optional observation preserve required distinctions | Negative/legacy tests |
| G3 | pass | No acceptance assertion removed | Validate request/result corpus |
| G4 | pass | Changes add no execution path | Keep schema validation non-mutating |
| G5 | pass | Replay uses stored snapshot and publication receives completed provenance | Fixture replay tests |
| G6 | pass | New identities include new knowledge; old IDs read unchanged | Identity regression tests |
| G7 | pass | Missing API remains partial; comparison gates still required in Phase 3 | Registry oracle |

These are proposal-level decidability verdicts, not runtime certification.

## 7. Principle findings

| Finding | Principles | Evidence or gap | Consequence | Correction | Verification |
|---|---|---|---|---|---|
| Empty features lost | DM-08, DM-15 | inspect uses nonempty list as knowledge | Explicit empty becomes unknown | Persist knowledge in identity | omitted_and_explicit_empty_features_have_different_identities; R05 |
| Replay mixes snapshots | DM-14, DM-32 | resolve replay uses current pointer with old data | Evidence and root identity disagree | Stored snapshot reference | recorded_resolution_keeps_its_snapshot_after_current_changes |
| Comparison scheduled too early | DM-59, DM-60 | R08 phase 1, P06 phase 2 versus blueprint Phase 3 | Gate cannot reflect its promised slice | ADR changes phase only | test_comparison_gate_phases_follow_blueprint |

Applicable groups: authority, absence, identity, publication, reuse, migration and verification.
Performance, numerical precision, remote distribution and execution isolation are outside this change.

## 8. Alternatives and architectural leverage

| Alternative | Cost and risk | Decision |
|---|---|---|
| Retain current behavior | Loses feature knowledge and mixes evidence | Reject |
| Explicit knowledge and optional observations | Small additive model/read changes | Select |
| Infer empty from declared environment | Simpler, but target-only declarations become false feature claims | Reject |

## 9. Verification and measurement plan

Run focused Rust fixtures, identity and legacy decoding tests, schema generation/conformance, then
phase gates. Reporting gains source/log digests and failure precedence tests. No performance claims.

## 10. Exceptions and unresolved decisions

No scoped exceptions. Operational hardening beyond these repairs remains future required work.

## 11. Decision and implementation changes

| Priority | Decision | Evidence strength | Acceptance condition |
|---|---|---|---|
| 1 | Accept proposed contract repair and phase alignment | Proposed; source inspected | Implement named oracles before claiming tested |

Acceptance here permits the ADRs; it does not certify Phase 1 completion.
