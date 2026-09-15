# Design review — retained execution observations

## 1. Decision and scope

**Final decision: Accept ADR-0026 at Proposed contract scope following the amended-contract
recheck in §11.** E1–E6 are resolved by the amended design. The initial findings and verdicts
below remain as review history; §11's final verdicts supersede them. Acceptance does not claim
that the new relation, durable job variants or producer integrations are implemented or qualified.

Date: 2026-09-14. Target: [ADR-0026](../../adr/0026-retained-execution-observations.md),
[Plan 10 T6](../../plans/10-arrow-datafusion-architecture.md), and their interaction with
ADR-0022/0023. Standard: [design charter](../design_principles/DATA_MODEL_DESIGN_CHARTER.md),
directive, template and repository addendum. Claim strength is **Proposed**, or
**Interface-checked** for the local types and call paths actually inspected.

**Method and coverage:** read the proposal, `ApiPayload`, `ApiObservation`, `ApiOrigin`,
`SubjectRef`, `FactSource`, environment/snapshot contracts, current `InspectRequest`, semantic
inspection and verification execution/journal paths, native environment admission and
`derive_environment`. Checked frozen blueprint §9.2 and the accepted target publication rules.
No implementation files were changed; no builds, tests, containers or upstream probes ran.
Cleanup fixes and the concurrently changing bounded job-cache implementation are outside this
review. The proposal explicitly rejects old development-data/journal migration; this review
does not reintroduce it.

## 2. Authority and lifecycle map

| Concept | Proposed authority | Boundary that needs to remain distinct |
|---|---|---|
| Static API | Existing `api_observations` plus definition/public-binding relations | A source/stub declaration is not a consumer diagnostic or runtime-object observation. |
| Executed result | New closed `execution_observations` relation | One authoritative fact representation; response fields and convenience views derive from it. |
| Semantic identity | Exact inputs/environment/provider/result, excluding attempt fields | Stable canonical result artifact precedes producer binding and observation identity. |
| Actual attempt | Durable job plus producer/log associations | Process clocks, document versions, physical capsule paths and cleanup witnesses remain attributable. |
| Execution scope | Input snapshot, query/snippet/object selector, actual resolved environment | Source context is not silently replaced or asserted to have been executed. |
| Mutable session | Warm LSP lease and protocol/document state | Completing a query is distinct from destroying its reusable server container. |
| Publication | ADR-0023 catalog commit and subsequent job completion | A crash between these durable authorities must have a defined reconciliation rule. |

## 3. Semantic contracts and invariants

| Invariant | Proposed mechanism | Assessment |
|---|---|---|
| Consumer probes do not masquerade as public API declarations | Separate relation and three closed payload variants | Sound direction; E1 must finish authority/subject/outcome rules. |
| Equivalent attempts preserve semantic identity | Stable result artifacts, semantic producer dependencies, attempt-only logs | Sound direction; E2 must define canonical positions/locations and the artifact hash order. |
| Scoped uncertainty survives normalization | Empty/unsupported/unresolved/incomplete/cancelled distinctions; typed coverage | Proposed; require per-variant conditional validation and explicit truncation/partial-result behavior. |
| No effect is caused merely by replay/cache eviction | Retained results and durable `job_control` | Terminal replay is specified; new inspection versus explicit execution remains E6. |
| Result publication follows safe execution settlement | Required cleanup and leases | Ambiguous for warm LSP sessions: E3. |
| Every job has immutable inputs and coherent recovery | Versioned concrete jobs, pinned inputs, no runtime rerun after restart | First resolution and committed-result recovery remain E4. |
| Actual environment is qualified | Resolved environment FK and inherited static sources | Output-context and inheritance semantics remain E5. |

## 4. Derivation and execution design

The additional relation is proportionate: a diagnostic about an agent snippet cannot be safely
encoded as an API declaration, and a runtime object need not have a matching static declaration.
Closed native Arrow payloads support filtering and validation without teaching consumers to
reinterpret arbitrary JSON logs. The existing nine public tools can carry this behavior.

Current local types expose the concrete integration pressure. `ApiOrigin` still permits
`Semantic` and `Runtime` (`relational.rs:382–383`), while `ApiPayload` represents declaration
fields (`:413`). `ApiObservation::new` admits only symbol/definition subjects (`:443`).
`SubjectRef` also has a document-artifact domain (`:19`), which can represent exact snippet
scope if its meaning and conditional FKs are selected explicitly. Existing admission requires
API observation environment IDs to match the snapshot (`admission.rs:437–442`).

The proposal can resolve these choices without a generic workflow engine, universal event
store, arbitrary LSP method surface, or compatibility reader.

## 5. Representative journeys

1. A snippet diagnostic concerns no single static symbol. Its exact document remains the
   subject, and an unresolved environment prevents an executed fact rather than inventing a
   placeholder environment or library-wide coverage.
2. The same source text is queried through servers negotiating UTF-8 and UTF-16. A character
   after an astral Unicode code point must identify the same bytes and produce equivalent
   canonical locations, while original protocol coordinates remain attributable.
3. A successful query returns from a warm server. The job publishes its answer while the
   server remains leased; later session eviction must not invalidate or regenerate that fact.
4. Evidence commits, then the daemon dies before writing the terminal job record. Restart
   retrieves that committed result without rerunning a runtime snippet or reporting it as an
   unqualified interrupted failure.
5. A runtime object exists dynamically but has no static symbol binding. The service reports
   its exact import/attribute selector and result; it neither invents a static definition nor
   drops the observed object because a local symbol FK cannot be satisfied.

These are required design journeys, not executed tests.

## 6. Acceptance gates

Initial verdicts, superseded by the amended-contract recheck in §11:

| Gate | Verdict | Contract-level evidence / remaining decision |
|---|---|---|
| G1 — Authority | unresolved | E1 must settle executed results versus the existing semantic/runtime API-origin variants. |
| G2 — Semantic fidelity | unresolved | E1/E2 need subject, outcome, coordinate and location preservation rules. |
| G3 — Validity | unresolved | Conditional variant/subject/environment admission is promised but materially underspecified in E1/E5. |
| G4 — Hidden behavior | unresolved | E6 must distinguish retained inspection from explicit new execution and policy checks. |
| G5 — Consistency and recovery | unresolved | E3/E4 need warm-query settlement and catalog/job crash reconciliation. |
| G6 — Transformation and reuse | unresolved | E2/E5/E6 need exact canonicalization, environment inheritance and reuse selection. |
| G7 — Truthful capability claims | pass, proposed scope | The ADR labels itself Proposed and names target oracles without claiming executed qualification. Scope still needs the decisions above before acceptance. |

## 7. Principle findings

Initial findings are preserved below. Their final dispositions appear in §11.

| Finding | Principles / verdict | Concrete gap and consequence | Bounded correction | Required oracle |
|---|---|---|---|---|
| **E1 — Close observation subjects, outcomes and storage authority** | DM-02/06/07/12/42; Unresolved, high | ADR Decision at 51–68 names a typed subject and outcomes without allowed combinations. A snippet diagnostic and a dynamic runtime object can lack a symbol; `ApiObservation` requires symbol/definition subjects, while existing `ApiOrigin::{Semantic,Runtime}` creates a second plausible authoritative storage path. Absence, unsupported signature, failed import/attribute lookup and truncation must not collapse to optional empty fields or complete coverage. | State allowed subjects/FKs for each variant: document-backed snippet/query scope; admitted symbol when available for runtime; an explicit exact selector/document scope when no static binding exists. Specify finite outcomes and payload applicability, including pre-execution refusal outside execution facts when no resolved environment exists. Remove unused semantic/runtime API origins, or make any API-shaped convenience representation explicitly derived with observation lineage and no independent writes. | Native roundtrip and negative admission matrix: wrong subject domain, absent required source/environment, conflicting variant fields, non-exited process with exit code, partial/truncated result, missing signature versus failed introspection, and dynamic object without static binding. Runtime/stub disagreement remains independently retrievable. |
| **E2 — Specify canonical query coordinates, locations and result identity order** | DM-07/15/24/31/32/46; Unresolved, high | ADR at 54–56/73–79/89–101 binds query position and normalized results but does not define input coordinate units/base, output range mapping, external-document locators, or what is excluded from the normalized artifact itself. Current wire locations carry capsule URIs and negotiated columns (`wire/data.rs:283–327`). Generic serialization can retain generation paths, conflate UTF-8/UTF-16 positions, or recursively include the `FactSource` whose artifact/binding is being computed. | Define optional advanced coordinates in one canonical domain, validate code-point boundaries, and convert at the negotiated protocol boundary. Use artifact-qualified canonical ranges, with explicit qualified external/unresolved locations when bytes are not retained. Normalize adapter-controlled URI/coordinate fields only; target stdout/stderr remains literal result content. Hash the stable result payload before inserting its own artifact/observation ID or `FactSource`, then compute producer binding and observation ID. Preserve raw protocol coordinates/versions in attempt receipts. | Unicode/CRLF/range truth table across UTF-8 and UTF-16; missing external source remains explicit; same query across capsule UUID/document-version/batch changes has one semantic result; changed literal target output changes it; literal nonrecursive identity vectors. |
| **E3 — Define successful warm-query settlement separately from container cleanup** | DM-29/30/35; Unresolved, high | ADR at 103 requires producer/required cleanup settlement before publication, while Plan 09/blueprint §9.2 retains warm servers. Applying verification's container-absence rule to semantic jobs either prevents completion until eviction or destroys warm reuse. Treating any received response as settled can instead leave cancellation/diagnostics for another document in flight. | Define a per-query protocol/document settlement point after the matching response and required diagnostics are bound. Successful queries may publish while the warm session's execution lease remains active. Cancelled/unsettled sessions must drain within a bound or be discarded; container cleanup is required on discard, and uncertainty remains explicit. Never release warm execution ownership merely because one query completes. | Two durable queries complete and publish through one warm server; cancel an outstanding request and verify no late response/diagnostic reaches the next document; discard with removal failure remains quarantined and cannot manufacture an unqualified successful job result. |
| **E4 — Specify cold-resolution input identity and catalog-to-job crash reconciliation** | DM-07/14/29/30/35; Unresolved, high | ADR at 81–87 says every variant pins an input context/snapshot before enqueue, but a first expensive resolution has neither. At 103–106 it promises terminal replay of published results without specifying the crash between catalog commit and terminal journal update. Current `Jobs::open` turns every nonterminal entry into interrupted failure, even if evidence has already committed. Cancellation racing publication is similarly undecided. | Use an explicit unresolved-resolution JobSpec containing the normalized selector/policy/configuration, then record resolved identities; inspection/verification require existing pins. Persist a recoverable result/publication reference or reconcile the actual attempt's committed catalog association before classifying a restart. A committed result wins over later cancellation; precommit interrupted execution remains an explicit nonreplayed interruption. Name failure behavior for bounded publication conflicts. | Cold resolution returns a durable pending receipt without an invented snapshot. Crash immediately before/after catalog commit and before/after terminal journal write; committed results replay once, precommit runtime never reruns automatically, and cancel-after-commit preserves the committed outcome. |
| **E5 — Make resolved-environment output context and inheritance explicit** | DM-06/14/24/31/46; Unresolved, high | ADR at 51/74–75/104–105 requires resolved environment facts and original static sources but does not select the output context when input context is declared/unspecified. Existing snapshot metadata and API admission require one context environment; `repository.rs::derive_environment` rewrites static observation scope and removes executed origins. Blind union or generic environment rewriting could reattribute earlier execution results to a new environment. | State that execution under a different actual resolved environment creates/reuses a derived context and contributes there; return both original and result context/snapshot references. Inherit only reusable static declarations with unchanged qualified source/observed-documentation provenance, explicitly describing any contextual rebinding. Never copy prior execution observations or their coverage across an environment boundary. Same-environment later observations use ADR-0023 rebase/alternative semantics. | Declared parent to resolved child; parent remains byte-identical, child's actual environment is validated, original static producer qualification remains intact, and prior runtime/semantic facts and coverage do not migrate into another environment. Concurrent same-child additions both survive. |
| **E6 — Declare retained-read versus explicit re-execution semantics** | DM-20/28/30/32/43; Unresolved, medium | ADR specifies terminal-job replay and runtime intent, but not whether a repeated `inspect_symbol` reads an exact retained observation or reruns target code, nor how conflicting retained observations are selected. Requiring runtime policy for a retained read conflates permission to read evidence with permission to execute. Conversely, implicit re-execution after cache eviction violates the intended evidence lifecycle. | Define the request distinction and defaults. A retained inspection reads bounded exact-query/environment alternatives without executing; new runtime/semantic work requires explicit execution intent and locally enabled policy. Define verification resubmission semantics separately from `job_control` replay. Cache eviction/age must never be the trigger for another attempt. Do not silently choose one conflicting result by insertion order. | Disable execution after retaining a result: retained inspection and job replay still work without process starts. Evict session/provider caches and advance time: result remains readable. Explicit new execution records another attempt, and contradictory outcomes remain distinguishable with bounded deterministic retrieval. |

**Applicability:** semantic authority/types, validity, effects, identity, concurrency,
provenance, boundaries and truthful claims are directly relevant. Physical tuning, backend
selection and broad performance qualification remain outside this proposed-contract review.

**Additional scope correction:** retain `references` in the finite semantic method vocabulary,
or explicitly decide and justify its removal; blueprint §9.2 names the scoped capability and
the existing producer implements it. Keep coordinates optional: blueprint §9.2 says callers
should not have to manufacture line/column coordinates. Rust should generate/map anchors or
return explicit ambiguity; document-wide diagnostics need no position.

## 8. Alternatives and proportionality

| Alternative | Semantic / maintenance tradeoff | Disposition |
|---|---|---|
| Logs plus transient wire DTOs | Reinterprets producer documents at every consumer; no native scope/FK queries | Correctly rejected. |
| Extend declaration payload with arbitrary execution fields | Conflates declaration and consumer/runtime facts | Correctly rejected. |
| One closed execution relation and concrete jobs | Localizes normalization, validation and native retrieval | Retain; resolve the six bounded contracts above. |
| Generic event model or workflow framework | More machinery without an additional required consumer | Unnecessary. Ordinary typed Rust variants and the existing publisher/journal suffice. |

## 9. Verification plan

| Claim | Evidence level now | Required proof after implementation |
|---|---|---|
| Tenth relation preserves variant meaning | Proposed | Independent Arrow roundtrip/negative variant and FK fixtures, including nested null versus empty and truncation. |
| Stable semantic result, complete attempt attribution | Proposed | Canonical coordinate/identity vectors, attempt-only changes, raw receipt and offline export closure. |
| Warm durable semantics and correct diagnostics | Proposed; local session interface checked | Actual ty/rust-analyzer methods, Unicode, push replacement, full/unchanged pull reuse, cancellation and next-document isolation. |
| Runtime objects remain distinct from declarations | Proposed | Actual native/dynamic object and stub disagreement; bounded attributes/signature failure; explicit runtime policy. |
| Durable jobs and publication survive interruption | Proposed | Cold resolve, surviving subscriber, queue/interest/journal bounds, commit/journal crash barriers, exact terminal replay. |
| Full product qualification | Not established by this review | Matching-source producer, containment, client, wire and state-boundary gates after implementation stabilizes. |

## 10. Exceptions and unresolved decisions

No broad architectural exception is required. The initial unresolved choices above are now
settled by the amended ADR as recorded in §11; implementation and actual qualification remain
later obligations.
Source provenance and future exact-context retention remain binding, while existing development
snapshots/journals remain disposable through the authorized scoped reset.

## 11. Decision and implementation changes

| Priority | Decision/change | Acceptance at this review's scope |
|---|---|---|
| 1 | E1/E2: close subjects, variant validity, authoritative storage and canonical identities | Explicit normative contract plus independent target oracles. |
| 1 | E3/E4: warm query settlement, cold resolve identity, commit/journal recovery | One decidable lifecycle for success, cancellation, failure and restart. |
| 1 | E5: derived-context environment boundary and inherited fact rules | No prior execution result can be rebound to another environment. |
| 2 | E6 and scope correction: retained read, explicit execution, optional anchors and references | Effects, replay and supported method scope are explicit. |

**Initial decision, superseded: Revise at Proposed contract scope.** The extra relation and concrete job approach are
appropriate. Acceptance can follow these contract decisions without requiring the implementation
to exist; it must not be mistaken for executed producer or phase acceptance.

### Amended-contract recheck — 2026-09-14

Read the amended ADR-0026 Decision from disk, including the final clarification of static API
contextual rebinding, retained-read policy and negotiated-encoding attribution. This recheck
changes only the design review. No implementation, build, test or container qualification was
performed, and the separate cleanup review was not reopened.

| Finding | Final disposition | Amended contract that settles it |
|---|---|---|
| E1 | Satisfied at Proposed scope | Runtime observations bind an admitted symbol or exact document/selector scope; semantic queries and usage probes bind exact documents. Referenced subjects, environments and sources must be in the admitted closure. The tenth relation is the sole execution authority and `ApiOrigin::Semantic/Runtime` are removed. Closed native variants, conditional admission and malformed-variant oracles remain required. Pre-execution refusal must not fabricate an executed fact or a resolved environment. |
| E2 | Satisfied at Proposed scope | Canonical payloads use fixed UTF-8 coordinates; optional input coordinates require character boundaries and are converted at the LSP boundary. Returned locations are artifact-qualified or explicitly external/unresolved. Negotiated encoding, document versions and physical capsule paths are attempt attribution. Stable payload hashing precedes source/binding/observation identity; target output remains literal. |
| E3 | Satisfied at Proposed scope | A successful query settles at its matching response/document boundary while the warm session keeps its lease. Cancellation must drain or discard/clean up before terminal cancellation. One-shot execution requires confirmed container cleanup. Query completion therefore does not release warm ownership or require destroying a reusable server. |
| E4 | Satisfied at Proposed scope | Cold resolution has a distinct normalized unresolved JobSpec and durably records exact identities after resolution. Verification/inspection require existing pins. The catalog commit includes a job publication association with normalized result artifact, outcome and published scope; restart reconciles it before marking interruption, and cancellation after commit cannot relabel the result. |
| E5 | Satisfied at Proposed scope | A changed actual environment selects a derived context. Static API observations are explicitly contextually rebound with recomputed IDs and unchanged qualified sources, making no execution claim. Previous execution observations remain in their original context and are excluded from the child. Same-environment additions retain the accepted contribution/rebase contract. |
| E6 | Satisfied at Proposed scope | Retained reads require neither execution qualification nor runtime policy, default to bounded deterministic exact-query alternatives, and report a miss explicitly. Execution on a miss and rerun of existing evidence require explicit intent; usage verification is effectful. Replay, age and cache/session eviction never initiate execution. |
| References and optional anchors | Satisfied at Proposed scope | References is included in the finite method vocabulary. Rust derives/maps an anchor or reports ambiguity; coordinates are optional advanced input and document diagnostics need none. |

| Gate | Final verdict | Scope and evidence |
|---|---|---|
| G1 — Authority | pass, Proposed scope | One execution relation; declaration origins removed; catalog job association is publication authority and terminal journal is a delivery cache. |
| G2 — Semantic fidelity | pass, Proposed scope | Exact document/object scope, native tagged payloads, canonical coordinates, qualified external locations and preserved source/runtime distinctions. |
| G3 — Validity | pass, Proposed scope | Conditional native admission, snapshot closure/FKs, resolved environment requirement, bounded methods/coordinates and explicit uncertainty. Exact field encodings and negative fixtures are implementation obligations. |
| G4 — Hidden behavior | pass, Proposed scope | Retained reads and execution/rerun requests have distinct effect and policy contracts; defaults cannot start a producer from an evidence miss. |
| G5 — Consistency and recovery | pass, Proposed scope | Warm-query settlement, retained lease ownership, one-shot cleanup, cold-resolution stages and catalog-to-journal reconciliation are specified. |
| G6 — Transformation and reuse | pass, Proposed scope | Acyclic result/binding/observation identity, attempt-only protocol metadata, explicit static contextual rebinding and non-expiring retained reads. |
| G7 — Truthful capability claims | pass, Proposed scope | The ADR remains explicitly Proposed, with required actual producer/roundtrip/recovery/client oracles rather than an implementation or qualification claim. |

**Final scoped decision: Accept.** The amended contract is sufficiently decidable to implement
without adding another architecture decision for E1–E6. All §7/§9 executable oracles remain
required; this acceptance does not close Plan 10 T6, any product acceptance gate, or Phase 4–6.
