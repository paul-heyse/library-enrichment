# Exact inspection selection review — 2026-09-14

## 1. Decision and scope

Review ADR-0035's inspection discriminator and typed ambiguity candidates. Verdict is **Accept-scoped**, supported by
the real static extraction/restart regression on 2026-09-14. This is a local request/response correction;
full Plan 12 acceptance remains separate. Reviewed the actual Serde A04 brief and native result,
Rust request, native lookup, inspection and execution request normalization, and MCP forwarding.
No new library semantics or external macro-resolution completeness is claimed.

## 2. Authority and lifecycle

Definition IDs already belong to admitted Rust evidence. The request selects one within an
existing path and snapshot; candidates are bounded projections, not another identity authority.

## 3. Semantic contract

Distinct same-path kinds remain distinct. Selecting a definition retains every independent
observation of it. An ID from another path does not widen the selection. Qualified Python paths
are no longer treated as unqualified suffix requests.

## 4. Execution

The path and definition predicates run in DataFusion before hydration. Explicit execution uses
the selected definition in its normalized durable request; retained reads retain no execution
side effect. Ordinary domain validation and bounded candidate formatting remain in Rust.

## 5. Representative journey

Source declares a class and stubs declare a function at one Python path. The client receives
two IDs, selects either, and repeats its selection after daemon restart. The rejected path/ID
pair must not silently select an unrelated suffix. The original real Serde collision motivated
this journey; unknown external re-export targets remain explicit evidence gaps.

## 6. Gates

| Gate | Verdict | Evidence and scope |
|---|---|---|
| G1 authority | pass | Existing admitted definition IDs and snapshot ownership |
| G2 fidelity | pass | Typed candidates preserve kind/qualifier; selected observations stay distinct |
| G3 validity | pass | Real selection preserves source/stub origin, kind and source excerpts; mismatched qualified path rejected |
| G4 effects | pass | No change to explicit execution policy or retained-read effects |
| G5 recovery | pass | Both selections remain usable after daemon restart |
| G6 reuse | pass | Normalized durable requests include exact selected definition |
| G7 claims | pass | Ambiguity/unknown targets remain explicit; no external resolution promise |

## 7. Findings

Applies to identity, distinctions, actionable interfaces and native selection. Numeric models,
optimizer extensions and new storage infrastructure are outside this scope.

| Finding | Principles | Evidence | Consequence | Correction | Oracle |
|---|---|---|---|---|---|
| Corrected implementation: repeating one path cannot select two definitions | DM-06, DM-11, DM-16 | Actual Serde A04 trace; former string-only candidates | Client is stuck after a valid ambiguity result | Optional ID predicate and typed candidates | Real same-path wheel/MCP/restart regression and schema conformance |

## 8. Alternatives

Keep ambiguity as-is (incomplete interface), encode IDs into path strings (unnecessary syntax),
or use the already-established ID as an explicit optional selector (chosen). No new index or
framework is needed.

## 9. Verification

All-target/all-feature Clippy and generated schema conformance passed during implementation.
The named real regression, parser/receipt checks and MCP catalog checks passed together: 23 tests
in `.dev-state/plan12-selection-parser-current.log`. Full integrated Plan 12 acceptance is separate.

## 10. Exceptions

No new exception. The design-stage wire correction deliberately has no old candidate decoder.

## 11. Decision

| Decision | Priority | Required action |
|---|---|---|
| Accept-scoped | correctness | Selection, mismatch and restart are demonstrated; retain the regression and schema check |
