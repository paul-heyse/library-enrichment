# Design review — typed Arrow target contracts

## 1. Decision and scope

**Decision: accept ADR-0022, ADR-0023 and ADR-0024 at proposed-contract scope.** The
same-context publication omission found during review was corrected in ADR-0023 and Plan 10;
TC1 records the resolution. This is not implementation, resource qualification or Phase 4–6
acceptance.

Reviewed 2026-09-14: [ADR-0022](../../adr/0022-typed-evidence-hard-pivot.md),
[ADR-0023](../../adr/0023-relational-catalog-publication.md),
[ADR-0024](../../adr/0024-bounded-datafusion-queries.md), and
[Plan 10](../../plans/10-arrow-datafusion-architecture.md) §3 plus its dependent work/oracle
definitions. Standard: [charter](../design_principles/DATA_MODEL_DESIGN_CHARTER.md),
directive and repository addendum; compact depth.

**Method and coverage:** read the proposed records and incorporated plan contracts, compared
them with the prior independent assessment and its four plan corrections. Relevant pinned
Arrow/DataFusion interfaces were inspected during that assessment; this review does not
promote those interface checks to an exercised integration. No implementation files were
edited and no tests, builds, containers, dependency changes or data deletion were performed.
The current concurrent implementation is not certified by this document. All acceptance here
is **Proposed**; the known execution route is at most **Interface-checked**.

The operator authorized a hard pivot. Existing development evidence is disposable, with no
legacy-reader, migration or compatibility obligation. Source provenance and unrelated state
remain outside the reset. Future exact-context evidence validity is a separate lifecycle rule.

## 2. Authority and lifecycle map

| Fact | Authority and identity | Derived or operational representation |
|---|---|---|
| Definition/public binding | Rust typed model; definition differs from exposed alias/binding | Snapshot-scoped Arrow relations, navigation and presentation views |
| Qualified observation | Semantic producer binding plus qualified content | Distinct source/stub/runtime alternatives; actual attempt attribution remains explicit |
| Semantic snapshot | Context, inputs, producer/environment/policy semantics, observations and coverage | Exact file hashes and manifest separately bind physical bytes |
| Acquisition attempt | Unique run/event identity, clocks/logs and truthful associations | Equivalent reacquisition appends associations without changing semantic observation/snapshot identity |
| Published selection | One committed catalog generation and its root | All lookup views and pinned request catalogs use that generation |
| Query validity/resources | ValidatedSnapshot plus daemon runtime/admission ownership | Bounded provider/footer caches, disposable plans and spill |

Core Rust owns meaning, validation, state and publication. The Python boundary remains a
separate extractor and thin adapter. Ordinary Rust scoring/canonicalization is deliberately
specialized behavior behind a versioned contract; its presence is not semantic duplication.

## 3. Semantic contracts and invariants

| Invariant | Enforcement boundary and refusal | Proposed oracle |
|---|---|---|
| No conflation of aliases, same-path variants or semantic membership with lexical ancestry | Typed keys/references and conditional admission validation | `typed_model`; O1/O4/O7 |
| Hash graph is acyclic; equivalent runs do not manufacture new semantic facts | Semantic binding/observation IDs precede snapshot ID; file/manifest hashes follow it | Literal vectors and repeated-acquisition association test |
| Required meaning survives physical representation | Explicit variants, Struct validity, ordered lists, enum validation and output metadata | `arrow_projection`; O2/O3 |
| Invalid rows never reach a provider asserting keys | Bounded staging validation without PK/Unique assertions; admitted providers only afterward | `snapshot_admission`; duplicate/FK/corruption attacks |
| Every eligible search result can compete; count and page share scope | SearchSpec native eligibility, pure scoring, folding and deterministic keyset ordering | `search_truth_table`; O7/O9 |
| Failure/unknown/absence stay distinct | Typed coverage, resource errors and observed alternatives | O1/O8/O9; no false-empty success |

No old-format roundtrip is required. The target's declared semantics, including deliberate
corrections to current bugs, are the oracle. Exact-version facts do not expire when a provider
cache is evicted or a clock advances.

## 4. Derivation and execution design

The planned route is coherent: bounded typed producer results → Rust normalization → bounded
Arrow batches → local/relational admission → immutable snapshot → catalog generation commit
→ pinned validated providers → native plans and narrow score kernel → bounded hydration.

The process lock excludes another daemon; the new in-process commit coordinator reads and
merges against the latest generation while locked. Staging validation has a bounded runtime
in T2; T3 expands that resource owner rather than retroactively supplying its first limits.
Provider cache identity includes manifest/files, schema, validator and semantic dependencies.
Paths/mtimes are invalidation witnesses, not authority. The stated threat boundary excludes
an adversary controlling the daemon's OS account.

Search counts derive from the same folded scope as hits. A narrow UDF adapter does not hide
native eligibility predicates. Metadata receives explicit output roles when expressions do
not preserve source roles. Recorded diagnostics come from the executed plan and do not cause
a second hidden execution. Quotas cover allocations outside the managed DataFusion pool.

## 5. Representative journeys

- **Repeat acquisition:** two runs with different clocks/logs but the same semantic producer,
  inputs and observations select one semantic snapshot, with both attempts truthfully linked.
- **New observation:** source/stub/runtime disagreement remains separate qualified evidence;
  a display policy can choose a compact view without overwriting alternatives.
- **Crash/reset:** an unpublished complete artifact is not ordinary discoverable evidence;
  uncertain container creation/cleanup prevents deleting recovery authority during reset.
- **Concurrent same-context enrichment:** two jobs start at S0 and add different facts. The
  second detects a stale base under the commit lock, leaves its candidate unselected and
  rebases/revalidates against the winner. Conflicts remain alternatives; bounded retry failure
  is explicit. Current cannot silently lose the first successful job's additions.

## 6. Acceptance gates

These are contract-review verdicts, not results from `tests/gates.toml`.

| Gate | Verdict | Contract evidence and limit |
|---|---|---|
| G1 — Authority | pass, proposed scope | Typed Rust ownership, one semantic binding/hash graph, one catalog visibility root; views and caches explicitly derived. |
| G2 — Semantic fidelity | pass, proposed scope | Alternative observations, reference domains, null/list/enum/locator distinctions, metadata and coverage survive through declared transformations. |
| G3 — Validity | pass, proposed scope | Physical and relational admission precedes asserted constraints; bounded untrusted staging route and negative oracles are specified. |
| G4 — Hidden behavior | pass, proposed scope | Query side effects/resources are explicit; no arbitrary SQL/path provider, hidden reacquisition or duplicate ANALYZE execution. |
| G5 — Consistency and recovery | pass, proposed scope | Commit coordination, durable root, leases and reset recovery are specified; TC1 adds expected-base selection, validated rebase and explicit bounded conflict. |
| G6 — Transformation and reuse | pass, proposed scope | Semantic versus physical identity, dependency-bound admission, non-expiring facts and target-semantic query oracles are explicit. |
| G7 — Truthful claims | pass, proposed scope | ADRs claim Proposed behavior with a plausible inspected interface route; no test/performance/phase result is claimed. |

## 7. Principle findings

| Finding | Principles / verdict | Evidence and consequence | Correction / executable oracle |
|---|---|---|---|
| **TC1 — Same-context publication precondition; resolved during review** | DM-14/30/35 **Satisfied at corrected contract scope** | The initial proposal preserved catalog additions but did not settle current selection when two jobs enriched the same old snapshot. ADR-0023 Decision and Plan 10 §3.3 now require expected-base comparison under the commit lock; a stale candidate stays unselected, additions rebase onto the winner, then validate/retry. Conflicting observations remain alternatives; retry exhaustion is explicit. | Mandatory same-context disjoint-addition oracle proves current contains both successful additions; add conflicting alternatives and retry/failure cases. This correction is Proposed, not an executed concurrency result. |
| Semantic identity and attempt attribution separated | DM-02/11/12/15/46 **Satisfied at contract scope** | ADR-0022 and Plan 10 §3.2 explicitly separate semantic producer binding, observation, snapshot and physical/attempt identities; repeats append associations. | Literal nonrecursive identity vectors; two real attempt records, one semantic snapshot; changed material source/configuration changes the relevant identity. |
| Validated keys and bounded staging | DM-07/09/22/43 **Satisfied at contract scope** | ADR-0023 and T2 distinguish unvalidated staging from admitted providers and avoid false symbol-definition/fragment-subject keys. | O4/O9: duplicates, typed FK attacks, value/batch overflow and validation spill exhaustion never publish. |
| Resource/recovery lifecycle declared | DM-28/29/32/40 **Satisfied at contract scope** | ADR-0024 and Plan 10 §3.4 account for out-of-pool allocations; reset blocks on uncertain owned execution and retains its journals. | O6/O9/O11: cancellation/cleanup failures, cache invalidation, forced disk exhaustion and reset after owner loss. |
| Physical mechanisms do not define semantics | DM-05/23/24/36/58 **Satisfied at contract scope** | Native Expr lowering, typed Arrow boundaries, qualified ordinary score kernel, metadata handling and measured physical tuning have named consumers. | O2/O3/O7/O8/O10; no mandatory Delta/custom optimizer or claim that a dictionary enforces enums. |

**Applicability:** groups 1–12 apply through the scoped schema, identity, query and catalog
contracts. Container isolation internals, actual LSP behavior, installer/client qualification
and implementation assertion adequacy are outside this review. Those open execution gates
cannot be inherited from contract acceptance. No maturity score is assigned.

## 8. Alternatives and architectural leverage

| Alternative | Assessment |
|---|---|
| Preserve three old tables and historical readers | Contrary to the explicit development pivot; rejected. |
| Add Delta or custom query engines | No demonstrated need; target has native interfaces and one visibility protocol. |
| New tables but all assembly in owned Rust objects | Retains the identified repeated materialization and query costs. |
| Target native plans plus specialized Rust kernels | Selected proportionate route; one domain contract, explicit adapter boundaries and measured optimization. |

No additional schema platform, generic expression language or future graph infrastructure is
needed for acceptance. Further columns and exact budgets belong to implementation under the
declared invariants; their not-yet-written code is not itself a contract rejection.

## 9. Verification and measurement plan

| Claim | Required future evidence | Current strength |
|---|---|---|
| Typed/physical fidelity | O1–O4 negative tests and Arrow→Parquet→DataFusion→DTO roundtrips | Proposed |
| Repeated acquisition and publication | Acyclic vectors, attempt associations, fault barriers, distinct-context and TC1 same-context races | Proposed |
| Bounded lifecycle/reuse | O6/O9/O11 with small quotas, oversized values, replacement, cancellation and uncertain reset | Proposed |
| Search/comparison meaning | Independent target truth tables, real MCP cases, stable IDs/order/cursors and partial-coverage cases | Proposed |
| Performance claims | O10 workload measurements separating publication, validation, cold/warm query, hydration, RSS/pool/spill and scan costs | Proposed |

Tests do not have to run before acceptance of a proposed contract. They must run, with adequate
assertions and matching source/log/tool receipts, before an implementation claim or phase closure.

## 10. Exceptions and unresolved decisions

TC1 was the only identified blocking contract omission and is resolved in the revised proposal.
No blocking contract omission remains in this bounded review. API/field naming and numeric
resource settings can be completed in T0/T1 with the committed oracles and before tuning.

The lack of historical reader compatibility is authorized scope, not an exception needing
operator reconfirmation. New exact-context retention does not require preserving old development
evidence. Unmeasured optimization benefits remain hypotheses. Source work and frozen design
provenance are preserved during the reset.

## 11. Decision and implementation changes

| Scope | Decision | Remaining obligation |
|---|---|---|
| ADR-0022 typed evidence/reset | **Accept scoped proposed contract** | Implement and exercise the typed identities, attribution, reset and future retention oracles. |
| ADR-0023 catalog/publication | **Accept scoped proposed contract** | Implement and exercise TC1 expected-base/rebase behavior, durability, closure and read/export/cleanup leases. |
| ADR-0024 bounded queries | **Accept scoped proposed contract** | Implement and qualify semantics, quotas, cancellation, metadata and executed diagnostics. |
| Combined target / Phase 4–6 | **No completion claim** | T0–T7 implementation and required real acceptance evidence remain. |
