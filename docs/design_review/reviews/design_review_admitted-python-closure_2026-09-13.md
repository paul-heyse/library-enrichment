# Python dependency admission proposal review

## 1. Scope and method

Codex,2026-09-13. ADR0017 replaces the unsafe networked Python resolver mechanism in ADR0016.
Reviewed current core requirement parsing, capsule closure, recorded upstream compatibility
constraints and focused MCP tests against the charter/addendum. This accepts the revised design;
implementation and Phase4 acceptance still require independent review and broader negative tests.

## 2. Authority

Rust admits metadata and owns traversal/fetching. uv installs a supplied complete offline closure.
There is no second resolver authority after publication and no adapter-owned dependency policy.

## 3. Semantics

Unknown markers and conflicting selections are unresolved environments, not empty dependencies.
The root artifact stays pinned. Extras, constraints, selected hashes and environment are retained.
A supported conservative closure is distinct from universal packaging support or project parity.

## 4. Effects

Admission precedes each outgoing dependency edge. HTTP policy, resource bounds and cancellation
apply to acquisition. Package installation and all target execution remain in separate capsules.

## 5. Journeys

A normal transitive pure wheel is installed offline and imported by an explicit runtime probe.
A dependency containing a URL is rejected before following that URL. Cycles terminate; incompatible
constraints, unknown environment markers, source-only and native wheels fail with an explicit gap.

## 6. Gates

| Gate | Proposal assessment | Required oracle |
|---|---|---|
| G1 | pass | Canonical lock and source digests |
| G2 | pass | Explicit unresolved scopes and marker evaluation |
| G3 | pass | Parse all metadata before traversal |
| G4 | pass | Policy-bound fetch plus isolated offline install |
| G5 | pass | No environment publication before complete admitted closure |
| G6 | pass | Pinned root and exact lock identity |
| G7 | pass | No universal resolver or project-compatibility claim |

These are design assessments, not acceptance-gate results.

## 7. Findings and applicability

DM07/20/28/43/46 govern admission, effects and reproducibility. The main residual implementation
risks are current marker semantics, greedy conflict handling, bounded cancellation and retained
metadata provenance. Numerical-model and generic graph/query machinery are outside this scope.

## 8. Alternatives

Networked uv exposes effects before admission. A dependency-policy exception or fork adds
maintenance and does not establish current marker correctness. Explicit bounded scope is preferred.

## 9. Verification

Core grammar/environment negative tests; real dependency/hostile-source fixtures; cancellation,
conflict and cycle cases; Phase4 gate and independent re-review. Existing focused checks are not
full acceptance and do not retire the earlier execution-review findings without their oracles.

## 10. Exceptions

No binding exception or suppression. Unsupported packaging cases remain unresolved with a reason.

## 11. Decision

Accept the revised proposed contract. Implementation remains subject to the named obligations.
