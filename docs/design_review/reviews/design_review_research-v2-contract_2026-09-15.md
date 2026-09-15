# Research v2 contract review

## 1. Decision and scope

**Decision: Accept-scoped.** Accept the replacement contract and ownership decisions in
ADR-0036–0039 for continued implementation. This is **not** final Plan 13 acceptance or
permission to present the service as fully qualified. Evidence: Implemented/Tested for the
specific paths below; remaining implementation and campaign obligations stay open.

Baseline: HEAD `41ac67219ea39e6b085dde6273c9df08c9937bba` plus the Plan 13 working tree.
Review date: 2026-09-15. Reviewer: implementation agent, with a separate upstream verifier.

### Method and coverage

Read the charter, directive, addendum, original functional review, Plan 13, core research wire,
coverage/comparison types, store preparation/runtime/coverage/comparison/result paths, daemon
comparison/delivery, FastMCP presentation/middleware and focused tests. Exact dependency source
checks are in the upstream verification notes. Assessed public meanings, ownership, version
rejection, bounded continuation and declared effects. Did not certify all producer routes,
all archive formats, final installed clients, performance or production state. Those are W7,
W10–W12 and J01–J20 obligations, not silently excluded product requirements.

## 2. Authority and lifecycle map

| Concept | Owner and identity | Derived representation / update |
|---|---|---|
| Selection, outcome, coverage, diagnostic, delivery | Rust core; research/2.0 | Rust schema emitter → Python domain models; authored Pydantic output composition |
| Evidence and independent source observations | Rust normalized immutable snapshot/6.0 | Admitted Arrow/Parquet, native views; no adapter-side sufficiency policy |
| Query attempt and limits | Store runtime; operation ID plus early query ID | Bounded stage diagnostics, actual physical metrics; no replay to collect metrics |
| Job/publication | Rust daemon/store; job/5, catalog/4 | Complete admitted result before success; indexed result/2 recovery |
| Large comparison value | Content-addressed artifact plus alternative FactSource | Readable handle with byte length/digest; no loss of source identity |
| Historical specification | Original provenance manifest | Retained original bytes; new bundle selects current research/2.0 only |

Specialized scoring and archive decoding remain ordinary code behind typed inputs and limits.
They are not arbitrary user execution mechanisms.

## 3. Semantic contracts and invariants

| Contract | Boundary / rejection | Evidence |
|---|---|---|
| Requested scope distinguishes indexed/partial/missing/unknown | Native requested-domain left join | requested-coverage repository test; offline/cold retrieval test |
| Output fields retain native types, role tags and allowed nullability | Independent family requirements, analyzer/optimizer, physical stream checks | runtime tests, including malformed plan and missing role |
| Every alternative remains reachable | Changed-key and side cursors, individual inline/artifact variant | 97-alternative traversal; Unicode artifact roundtrip |
| Errors preserve cause and response status | Typed native errors, JSON-schema/Pydantic output validation, MCP is_error | retrieval corruption and catalog/socket/progress tests |
| Recovery does not regenerate results | Verified indexed sections from committed bytes | read-only no-scratch recovery regression |

Set comparison promises distinct value equality with null-equal anti joins. It does not promise
bag multiplicity or source compatibility from rendered signature equality. Empty lexical results
and incomplete sides never establish absence. String representation changes are permitted only
within native string families; semantic role metadata and physical stream shape remain checked.

## 4. Derivation and execution design

Selection lowers to a scoped native plan over admitted files. DataFusion 55.1.0's default analyzer
and optimizer run before physical planning; no replacement analyzer pipeline or raw SQL tool is
introduced. Shared runtime resources coexist with independent operation deadlines and retained
output charges. Counts and pages retain exact declared ordering; operation-local materialization
is a measurement decision, not a presumed property of a ViewTable.

FastMCP 4.0.3 provides transport, strict argument binding and authored output composition. Rust
remains the source of domain defaults and coverage. Optional progress has no authority to change
a completed result. Producing derived delivery artifacts is a declared operation effect; it does
not mutate evidence snapshots. Native managed-memory limits are not RSS guarantees.

## 5. Representative journeys

An added inspection aspect changes the Rust selection and sufficiency mapping, its native query
and focused tests; schemas regenerate mechanically. A changed scope invalidates the bound cursor.
An outer join may legitimately make a field nullable, but cannot erase a required output role.
A value that exceeds inline size retains its own artifact and source. A rejected plan retains
its own diagnostic identity; a failed progress notification leaves the pending receipt intact.
A restart recovers the committed outcome from verified indexed sections without a replacement
write. These journeys have focused regressions; full real-client/publication campaigns remain open.

## 6. Acceptance gates

| Gate | Scoped contract verdict | Evidence / remaining action |
|---|---|---|
| G1 Authority | pass | Rust wire/core and normalized evidence own meaning; generated domains and authored presentation have distinct roles |
| G2 Semantic fidelity | pass | Explicit scope states, source-bearing alternatives, tagged complete value delivery, per-facet pages |
| G3 Validity | pass | Admission and native field checks; strict inputs and actual output validation; expand all-family coverage in W2 |
| G4 Hidden behavior | pass | Derived artifact writes declared; no producer execution on ordinary signature lookup; explicit execution profiles |
| G5 Consistency/recovery | pass for contract | Precommit publication and immutable indexed recovery; full current-source fault/export journeys still required |
| G6 Transformation/reuse | pass for contract | Qualified source folding and distinct null-equal reconciliation; no new persistent cache; W11 measurements required |
| G7 Truthful claims | pass for scoped acceptance | No declaration of overall completion, source closure or measured speedup; capability failures remain visible |

These are design-contract verdicts, not the acceptance registry's runtime gate states.

## 7. Principle findings

| Finding | Principles / verdict | Evidence and consequence | Correction / oracle |
|---|---|---|---|
| One explicit research result model | DM-01/02/08/41/42: aligned | Domain state no longer inferred from delivery success or empty payload | schema-conformance, catalog tests, retrieval_fixture |
| Complete qualified alternative delivery | DM-18/24/46/53: aligned in tested bounds | Alias folding preserves independent sources; large JSON stays readable | views::documentation_folds_only_the_same_definition_and_source; comparison value test |
| Full operation preparation coverage remains incomplete | DM-07/22/31/50: not yet aligned for all W2 families | Current independent requirements cover three families; full selected-input/inventory diagnostics remain open | Complete W2; runtime negative family tests |
| Final deployed closure is unverified | DM-14/30/39/54/59: not yet aligned for complete product claim | Focused tests cannot certify clients, all publication failures or workload costs | J16–J20, W11, closing review |

Applicability: all groups bear on these cross-boundary contracts. Group 8's comparative cost
claims are deliberately unmade and remain W11 work; no unrelated numerical-precision guarantee
is introduced. No aggregate score is used.

## 8. Alternatives and leverage

| Alternative | Disposition |
|---|---|
| Patch old handler thresholds and retain old readers | Rejected: preserves competing scope rules and indivisible responses |
| New general query type system or native MCP composition engine | Rejected: duplicates Arrow/DataFusion/Pydantic mechanisms |
| Small Rust contracts, native plans, generated domain models, authored presentation | Selected: semantic choices have explicit owners and focused oracles |

## 9. Verification and measurement

Current receipts are enumerated in the execution ledger's 2026-09-15 implementation checkpoint.
Runtime/repository/views, 23 retrieval tests, delivery recovery, value roundtrip, schema
conformance, adapter tests and affected Clippy support only their named cases. W11's cold/warm,
one/eight-client measurements and complete final-source campaign remain required. CI currently
stops at the historical product-skill provenance mapping; refreeze must preserve original bytes.

## 10. Exceptions and unresolved decisions

No MUST exception is accepted. The scope of this verdict is the contract decision, not complete
implementation alignment. Full operation inventory, revision disposition lowering, artifact
closure fault/export qualification, deletion evidence, measurements and deployment remain in
Plan 13. Owner: implementation/integration maintainer. Trigger: closing any corresponding W/J
row requires its executed oracle and the final review.

## 11. Decision and implementation changes

| Priority | Decision/change | Required evidence |
|---|---|---|
| 1 | Accept ADR-0036–0039's replacement contracts; retain original provenance and refreeze current contract | ADR index/lint, original/new manifest verification, schema-conformance |
| 2 | Complete remaining native, revision and closure work | W2/W5/W7/W8 exit tests |
| 3 | Qualify final installed generation and cut over once | W10–W12, D01–D12, J01–J20 and closing review |

**Accept-scoped** allows continued implementation and contract refreeze. It does not close
Plan 13, authorize weakened checks, or substitute for real end-to-end acceptance.
