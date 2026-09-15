# Independent assessment — Arrow/DataFusion review corrections

## 1. Decision and scope

**Decision: Revise the proposed corrections before implementation.** The source review identifies
real correctness and efficiency work, but several recommendations assume invariants the evidence
model does not have. This assessment is input to the completion plan, not that plan itself.

Target: [the operator's F1–F13 review](design_review_arrow-datafusion-delta-leverage_2026-09-14.md).
Standard: [charter](../design_principles/DATA_MODEL_DESIGN_CHARTER.md), directive and repository
addendum; compact depth. Evidence below is **Implemented** source inspection or **Proposed**
correction, never a new test or performance claim. Checked 2026-09-14 against the shared tree.

**Method:** read the review and relevant current store `query`, `tables`, `catalog`, `snapshot`;
core search, comparison, evidence identities, Rust/Python normalization and source fragments;
daemon search/inspect/compare; receipt validator; relevant design/ADR statements. Checked pinned
DataFusion 55.1.0 expression and constraint source, and Parquet 59.3.0 writer properties. No code
edits, builds, tests, containers, dependency experiments or acceptance regeneration were performed.
The original review is unchanged. Delta's recorded dependency experiments were not independently
repeated. Execution/capsule completion remains outside this assessment.

**Operator correction incorporated:** this is a hard design pivot. Existing development evidence
and historical snapshots need not be preserved; the target does not require legacy readers,
migration machinery or compatibility with accidental current behavior. Plan to supersede the
historical-read obligations in ADR-0020. No data is deleted by this planning assessment. Indefinite
validity of exact-version evidence describes the future target lifecycle, not an obligation to
carry existing development state into it.

## 2. Authority and lifecycle map

| Fact | Current authority and distinction | Planning consequence |
|---|---|---|
| Search matching/ranking | Core scorer, with a separately implemented store candidate predicate | Candidate retrieval must be a superset of every positive score under the same scope. |
| Public symbol and definition | `model.rs:125–203`: a public path ID and a definition ID shared by aliases | `definition_id` is not a unique symbol-table key; same-path variants also matter. |
| Containment | Public lexical parent, transitive lexical area, and typed member edge have different domains | Name the domains before deriving views or consistency constraints. |
| Fragment subject | `model.rs:378–380`: symbol path, feature name or heading | No universal foreign key from subject to symbols. |
| Immutable evidence | Snapshot manifest and published tables; mutable current/lookup pointers select it | Query views and indexes are derived; they must not become competing publication authorities. |
| Execution evidence | Source/log/tool receipts in `scripts/evidence_run.py` | Dirty or untracked source can be named by a content digest; commit ID alone is insufficient. |

## 3. Required semantic contracts

Specify candidate completeness, ranking factors, alias folding, stable tie order, exact totals,
byte/item pagination and cursor binding before query rewrites. Specify symbol/definition identity
domains, relationship source/target roles and fragment subject variants before foreign-key checks.
Specify semantic comparison projections independently of their Rust or DataFusion execution.

Malformed required data must fail explicitly. The fresh target must distinguish declared absence
from malformed data. In its future lifecycle, exact validated evidence does not expire with age;
a new version does not invalidate old exact-version evidence. Existing development snapshots are
disposable, with no backward-reader or migration requirement for this pivot.

## 4. Derivation and execution

Keep backend-neutral search/comparison contracts in core and lower them to native DataFusion
expressions in store. Core currently has no DataFusion dependency. A small typed predicate
description with a Rust evaluator and store lowering is sufficient; putting `Expr` in core or
using a universal UDF is not required for one authority.

Cross-snapshot comparison requires one query session registering the two pinned inputs under
unambiguous names. A context/snapshot catalog hierarchy is also justified by concrete global
metadata discovery consumers; it is not a prerequisite for a join. Manifest-derived record
batches or views can make provenance queryable while retaining the manifest as authority. A
fresh relational catalog with one manifest commit is a valid target beyond patching JSON indexes.

## 5. Representative failure and extension cases

An inferred Python namespace has only `doc_summary`; a summary search must retrieve it. A
one-character fragment subject can earn `subject_exact` despite tokenization returning no terms.
A case-insensitive one-character symbol match can be lost by case-sensitive exact/suffix probes.
Adding another score factor must preserve the candidate-superset invariant in all these cases.

A reader or optimizer must not infer uniqueness from an ID constructor. Two observations can
carry the same constructor inputs, and aliases intentionally share a definition. Invalid duplicate
identities need rejection or an explicit observation identity, not accidental deduplication.

A release index update interrupted after one write must yield a defined committed selection or a
recoverable missing index. Merely reversing write order can temporarily hide an already valid
older selection when a secondary pointer is replaced before the new primary record exists.

## 6. Acceptance gates for the proposed correction set

These verdicts concern whether the recommendations can safely be implemented as written, not
new phase-gate results or a complete reassessment of the running service.

| Gate | Verdict | Reason / required correction |
|---|---|---|
| G1 — Authority | fail | F9/F10 infer false keys and equivalent relationship domains; define the actual contracts first. |
| G2 — Semantic fidelity | fail | Premature search limits, a string-only locator map and unvalidated dictionary domains can change meaning. |
| G3 — Validity | unresolved | Correct target relational/cross-field admission rules still need specification. |
| G4 — Hidden behavior | unresolved | Bound query resources/cancellation; distinguish planned explain from executed analysis and avoid exposing arbitrary SQL. |
| G5 — Consistency and recovery | fail | Secondary-first release writes do not create a coherent transaction or preserve old selection through failures. |
| G6 — Transformation and reuse | unresolved | Canonical comparison semantics, target-format round trips and physical-layout truth need executable oracles. JSON alone is not proof of loss. |
| G7 — Truthful claims | fail | F2's named test is absent; F7 and receipt-absence claims are stale. Unused API counts prove no defect or benefit. |

## 7. Finding dispositions and concrete corrections

| Finding | Assessment against current code and principles | Corrected action and oracle |
|---|---|---|
| **F1** | **Valid duplication; consequence overstated.** Three lexical implementations remain (`query.rs:250,445,595`, `ops/search.rs:157`). DM-02/23 unresolved parity. The final Rust filter executes before `total_matches`; the claimed unreachable-total scenario does not follow. Pinned DataFusion treats backslash as the implicit LIKE escape, so different syntax is not itself different semantics. | Define one lexical-area contract, lower to native expressions, apply it to exact/suffix probes before deleting the final filter. Test equal, descendant, unrelated prefix, Rust/Python separators, empty area policy, null, case, Unicode, `%`, `_`, and backslash. Prefer semantic parity tests over bans on ordinary `starts_with` code. |
| **F2** | **Valid missing evidence; failure location overstated.** The test name occurs only in `docs/design/DESIGN.md:119` and `query.rs:116`. View arrays would currently fail typed decoding of query results; open validates raw Parquet before registration, so this is not necessarily an open-time failure. DM-59/60 violated for the named Tested claim. | Add target-format published-snapshot DataFusion round trips for symbols, fragments and relationships under supported physical string forms. Correct the comment and claim independently. A discoverable test name is necessary but not sufficient evidence: require the relevant successful source-bound receipt, not just `cargo test -- --list`. |
| **F3** | **Valid current correctness defect, with broader contract coverage needed.** Candidate expressions omit `doc_summary` (`query.rs:235–241`), while scoring includes it. Python inferred namespaces have summary-only content. DM-02/08/42 violated. `FACTORS` currently contains labels and points, not machine-readable field predicates. | Immediately plan summary coverage, then a typed matching/scoring specification. Require `score > 0 => candidate` differential tests across every factor, NULLs, qualified names, case and short queries. Cover one-character fragment exact matches and case-insensitive symbol exact/suffix matches; clause-count or AST-name checks cannot prove completeness. |
| **F4** | **Valid cost shape; proposed rewrites need semantic guards.** Overview collects all symbols and loops per namespace; inspect loads all rows for one definition; search materializes full candidates. DM-36–39 improvement opportunity, benefit unmeasured. | Push exact inspect projections and overview aggregates with declared stable child selection. Define target scoring, alias folding, totals and byte pagination before pushing limits; current incidental behavior is not binding. Scoring can execute on docs while returning only hit IDs/factors, followed by bounded payload retrieval; dropping a declared score input is invalid. Benchmark cold validation and warm query separately. Physically sort rows before declaring sort metadata; measure row-group sizing, selected equality-column blooms and filter pushdown rather than assuming substring pruning. |
| **F5** | **Valid physical-type coupling; not process-wide.** `SessionConfig` belongs to each reader session. `str_col` is Utf8-only and repeated inside row loops (`tables.rs:161–227`). Unknown tags now error rather than silently dropping rows. DM-41/42 adaptation unresolved. | Resolve typed accessors once per batch and support explicitly declared Utf8/view forms, with contextual errors for unsupported/missing required columns. Define target optionality directly. Test direct Arrow and Parquet/DataFusion round trips under supported representations. Do not ban every `StringArray` downcast or enable views until those paths work. |
| **F6** | **Retain no-adoption decision; do not re-open Delta implementation.** The original measured dependency report remains its own dated evidence. DM-58 satisfied by proportionate non-adoption; no new measurement here. Two identity systems are not inherently G1 violations if a mapping has one authority. | Preserve the dated compatibility/policy record. A future dependency-compatible release is only a necessary technical condition: require a demonstrated capability need too. The target design may select suitable verified pins; the current Arrow/DataFusion set presents no demonstrated reason for an upgrade or policy exception. |
| **F7** | **Stale compile defect and stale missing-control claim.** `Operation::id` now passes a JSON value (`capsule_protocol/mod.rs:48`). `evidence_run.py` hashes all relevant source, including untracked files, before/after execution and binds the log; `acceptance-check.py:46,93` checks receipts. Current compilation was not run in this assessment. | Do not stash shared work or require a clean git tree. Preserve the source receipt mechanism, rerun relevant gates on the final source, and reconcile stale design/register claims. Oracle: changed/untracked source or changed log invalidates the receipt; unchanged dirty source can remain reproducible. |
| **F8** | **Valid repeated materialization and nested attachment; architectural premise too strong.** `ops/compare.rs:175–205` loads both snapshots and repeatedly scans fragments. A fresh context can register both inputs; catalogs are not necessary. `core/compare.rs` already states semantic projections and has same-path/whitespace tests; ordinary Rust set algorithms do not violate the charter. DM-03/10/38 opportunity. | Establish target API/fragment/relationship projections first. Use pair-scoped registrations, projections/grouping/joins and keyed attachment where useful; keep specialized canonicalization in core. Test intended change IDs, before/after variants, provenance, ordering, totals and cursors, including reexports, missing sides, NULLs and CRLF-versus-indentation distinctions. Existing-code differential tests apply only where the target intentionally retains its semantics. No blanket ban on `BTreeMap` diffs. |
| **F9** | **Valid missing relational admission and release-index protocol; several asserted invariants false.** `definition_id` is shared by aliases. Fragment subjects include features/headings/examples (`model.rs:379`, `source.rs:79–129`). Targets may be symbol **or definition** IDs (`model.rs:265`). Constructors do not enforce row uniqueness. DataFusion `Constraints` represents PK/Unique and explicitly does not validate its input. DM-07/09/14 unresolved. | Define target identity/observation domains, then enforce duplicates, typed/conditional FKs, source/path consistency and definition dependencies at publication. Declare optimizer constraints only after validation; never mark alias-bearing `definition_id` unique in the symbols table. For releases choose one immutable authority and recoverable derived indexes with a coherent commit/selection protocol. Fault-inject every write, including replacement, restart and concurrent lookup; secondary-first alone is insufficient. |
| **F10** | **Correction rejected; domain clarification valid.** Rust `walk_member` emits MemberOf for item members (`normalize.rs:484–526`); ordinary modules/items also have lexical parents. Python normalization populates parents but emits reexports/inheritance, no MemberOf. Direct parent, transitive area and typed member edge are not three equal facts. DM-09/34 would be violated by collapsing them. | Specify lexical containment separately from semantic membership; derive lexical parents/areas from one path contract. Apply conditional consistency only where a producer declares the edge expresses that parent. Test nested modules, reexports, methods/fields/variants, trait qualifiers and Python namespace contributions. Do not navigate all namespaces from the currently incomplete MemberOf relation. |
| **F11** | **One concrete loss, several useful type improvements, several flawed corrections.** `deprecated=false` with since/note survives validation then loses values. JSON-encoded cfg/Python/locator values preserve structure when validly round-tripped; reduced queryability is DM-10, not proof of G6 loss. Differing enum spellings are awkward but invertible. Dictionary encoding does not enforce a closed vocabulary. A `Map<String,String>` cannot preserve arbitrary JSON scalar/object/array values. | Design a fresh typed target projection: nested `Deprecated`, cfg list and selected Python structure; preserve heterogeneous locator meaning using typed variants plus a declared JSON boundary where needed. Validate one target enum convention explicitly. Supersede ADR-0020 historical compatibility; no legacy format readers or old-state migration are required. Round-trip None versus empty, malformed states and nested values. Manifest-derived tables can expose provenance without making a second authority. |
| **F12** | **Useful portability enhancement; absence is not automatically lost meaning.** Current Arrow fields lack semantic metadata; file metadata and manifest retain some context. A standalone column does not carry the vocabulary contract. DM-06/46/55 opportunity. Multiple producers occur per table, so one field-level producer ID would be wrong. | Add versioned semantic roles/vocabulary/locator encoding metadata and schema-level revision bindings; retain row provenance for row-specific producers. An extension tag is descriptive, not a validator. Test preservation through actual Parquet, DataFusion projection/view and export paths. Use standard JSON annotation only after checking the required dependency features; a custom extension needs a consumer benefit. |
| **F13** | **Accept bounded diagnostics; reject mandatory UDFs and API-count reasoning.** No custom planner hooks is not a defect. A namespace/match UDF can conceal native predicate structure needed by optimizers and Parquet; registration does not prove candidate/scorer equivalence. EXPLAIN describes a plan; ANALYZE executes it. DM-04/27/49/58 require scoped claims. | Prefer native Expr lowering. Add optional developer plan/metrics artifacts identifying predicate version and pinned inputs; do not put raw physical plans in routine product Coverage or expose arbitrary SQL. Enable schema discovery only through a deliberate bounded interface. Verify actual scan/projection metrics and parity, not a brittle EXPLAIN string alone. Use a UDF only for necessary semantics not suitably expressible natively, with measured batch behavior. |

**Applicability:** groups 1–5, 7–12 apply to semantic contracts, identities, storage/query
transformations and evidence. Group 6 applies to query resources and catalog publication only;
container execution, durable jobs and transport are not re-reviewed. No maturity score is warranted.

## 8. Simpler viable alternatives

| Decision | Proportionate starting point | Additional mechanism needs |
|---|---|---|
| Shared matching | Typed predicate contract plus native Expr lowering | UDF only for a proven unsupported semantic operation. |
| Comparison | Two pinned table sets in one session; typed projection contract | Global catalog only for a concrete cross-context discovery need. |
| Provenance queries | Batches/views derived from validated manifests | Additional persisted tables only with ownership/rebuild justification. |
| Integrity | Admission validation first | Provider constraint wrapper only for validated facts useful to the optimizer. |
| Physical tuning | Existing Parquet writer/read options plus measurements | Delta/custom optimizer rules have no established need. |

## 9. Expanded Arrow/DataFusion work and missing oracles

The most consequential omitted dependency is **validation lifetime**. Every
`SnapshotReader::open` currently calls `validate_snapshot_table`, which decodes every row of all
three tables (`query.rs:127`, `tables.rs:450`). A projected inspect still performs full-snapshot
validation first. Plan a reusable validated snapshot binding with bounded lifecycle and exact
artifact/manifest identity before claiming answer-proportional warm retrieval. Do not replace
validation with an unbound path/mtime cache. Test tampering, replacement, restart, concurrent
readers, explicit cleanup and stale/current-pointer changes; separate cold validation from warm
execution measurements.

Also include these bounded enhancements:

| Enhancement | Contract and oracle |
|---|---|
| Query resources | Explicit per-daemon/per-query memory, concurrency, time and spill policies in service-owned state. Streaming/bounded collection where applicable. Test cancellation, spill exhaustion and cleanup; a 12-hit envelope is not a memory bound on a million-row collect. |
| Projection-specific batch APIs | Return definition paths, overview rows, candidate scores and comparison projections without rebuilding full `Symbol` values. Test required-column errors and parity, benchmark allocations. |
| Deterministic overview | Define which alias represents a definition and a stable child order before aggregate/window rewrites. `SELECT *` has no ordering guarantee despite the current stored-order comment. Test reordered input batches and multiple aliases. |
| Typed provenance views | Expose inputs, producer runs and gaps as derived relations with snapshot identity and actual cardinality. Test multi-producer rows and offline export reconstruction without mutating retained evidence. |
| Physical layout | Choose actual sort order, row-group sizes, top-level equality blooms and decoder pushdown from workload measurements. Verify emitted order/statistics and compare result parity plus bytes/rows scanned, peak memory and end-to-end latency. |
| Comparison scope pushdown | Read only selected scopes/projections and join evidence for emitted change subjects, while preserving all relevant provenance and stable IDs. Test each scope alone and combined, including empty/incomplete coverage. |

## 10. Dependencies and unresolved decisions

Order the work as: source/evidence reconciliation → target search/identity/containment contracts
and fresh typed schema → admission and catalog consistency → physical-type-safe batch access and
reusable validated readers → narrow retrieval/overview → comparison and search ranking lowering
→ provenance portability → workload-driven physical tuning and final regression gates. Schema
design and measurement preparation can proceed independently; optimizer key metadata depends on
admission enforcement, and LIMIT/top-K work depends on the declared target rank/fold contract.

The plan must decide observation identity and duplicate policy, relationship target typing,
conditional fragment subject references, release-index selection/recovery, query resource ownership,
and the fresh target format. Existing development evidence need not survive this pivot. Reconcile the old R-13
policy-key statement against ADR-0020 and actual producer paths rather than copying it as a fresh
finding; this assessment did not audit every producer key. No unmeasured speedup is an acceptance
criterion, and source-level reasoning is not a phase-completion receipt.

## 11. Decision and requested planning changes

| Priority | Decision | Required evidence before closure |
|---|---|---|
| First | Include F3 completeness, F11 deprecation validity, F2 evidence repair and corrected F9 integrity/publication work. | Negative/adversarial and end-to-end semantic oracles bound to final source. |
| Next | Include F1/F5 and contract-first F4/F8 enhancements; clarify F10 without conflating relationships. | Target-semantic tests for retrieval, aliases and comparisons; differential parity only for intentionally retained behavior. |
| Then | Include typed Arrow/provenance/metadata, bounded query runtime and measured physical layout improvements. | Round trips, resource/failure tests and representative cold/warm measurements. |
| Reject or narrow | F7 dirty-tree prohibition; definition/subject false constraints; index-first atomicity claim; universal UDFs; dictionary-as-enum validation; JSON-to-string-map loss; technology usage counts. | Replace each with the explicit corrected contract above. |
| Scope | **Revise** the original recommendation set; use this assessment as bounded input to the implementation plan. | No implementation authorization inferred beyond the user's current planning-only steering. |
