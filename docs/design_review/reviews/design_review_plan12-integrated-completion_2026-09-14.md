# Plan 12 integrated completion review — 2026-09-14

## 1. Decision and scope

Review the implemented native Arrow/DataFusion pivot, legacy removal and remaining Phase 4–6
functionality. **Verdict: Accept-scoped for completed functional implementation.** Scope is the
supported local Linux service, exact admitted Rust/Python inputs and qualified execution profiles.
Performance optimization is explicitly deferred by the user; no latency target is promoted to a pass.
Configured spill limits are present, but no forced-spill or spill-limit-failure campaign was executed.

### Method and coverage

Reconciled Plans 10–12, the living design, ADR-0028–ADR-0035, native ingestion/query/publication,
inspection and durable jobs, removal checks, actual test assertions and retained client outputs.
The independent acceptance auditor completed a read-only assertion/closure review and found no
new concrete functional blocker. Independent Rust replay passed; the user stopped the redundant Python replay after a registry-only
correction. No full current-registry audit pass is claimed.
The earlier scoped architecture review supplies the representation/admission assessment; this
review adds integrated producer/client usefulness, failure recovery and deletion closure.

Evidence is representative executable behavior, not a proof for every library or input. The user
prioritized final output assessment over exhaustive intermediate validation. Remote deployment, new storage engines,
GPU workloads, arbitrary client code editing and performance tuning are outside this completion.
No unchanged old receipt can establish current-source acceptance.

## 2. Authority and lifecycle map

| Fact | Authority / revision | Lifecycle |
|---|---|---|
| Definition, binding, observation, fragment and provenance | Rust semantic IDs and ten typed snapshot relations | Bounded producer transfer, native assembly/admission, immutable publication |
| Selected context and snapshot | One relational catalog generation | Exact retained lookup; changed selection preserves pinned snapshots |
| Producer execution | Qualified Rust policy, input closure, environment and concrete job | Owned capsule, confirmed cleanup or explicit quarantine, typed observation |
| Delivered research | Admitted facts plus bounded request projection | Complete immutable overflow artifact prepared before successful catalog visibility |
| Client research brief | Calling agent | Facts are cited; inference and unverified project availability remain qualified |

DataFusion selects, joins, groups, deduplicates, sorts and limits before bounded hydration. Domain
ID/locator/scoring kernels, finite DTOs and operational journals remain ordinary Rust. Python
forwards the generated service contract or performs the separate static extraction task.

## 3. Semantic contracts and invariants

| Contract | Mechanism / failure | Executable oracle |
|---|---|---|
| Same-path kinds and source/stub alternatives remain selectable | `query.rs::symbols_at`, `inspect.rs::read`, optional definition ID; mismatch rejected | `test_same_path_definitions_can_be_selected_without_merging_kinds` |
| Full docs survive bounded extraction | Complete worker docstrings; oversized declarations become explicit gaps with retained source | `test_long_documentation_is_retained_and_oversized_declarations_are_explicit` |
| Cache expiry cannot erase unchanged library facts | Exact version/qualified context selection independent of registry freshness | `test_exact_evidence_never_expires_and_new_versions_preserve_it` |
| Failed delivery cannot publish success | Prepared delivery before catalog commit; explicit journal failure settlement | `delivery_failure_prevents_job_catalog_commit`; `failed_terminal_delivery_leaves_a_failed_job_and_restart_never_reexecutes` |
| Typed observations do not imply project-wide success | Separate static, compiler, typechecker and runtime evidence, exact environment and limitations | `test_typecheck_success_and_runtime_failure_remain_distinct_observations`; actual client briefs |
| Client acceptance requires actual usable evidence | Native call/result correlation, complete jobs/pages, independent daemon witnesses and linked trace hashes | Actual client tier; parser and receipt rejection regressions |

## 4. Derivation and execution

`dataset.rs::write_transformed_plan` uses a one-batch channel to a blocking writer which retains
input plans and the lease. Only successfully exhausted queries request finalization. Native
semantic-key projection/distinct/sort precedes incremental canonical hashing. Worker allocations
are bounded before raw parsing; the daemon has no raw AST fallback.

Successful publication binds snapshot, actual attempts and prepared delivery in one catalog
selection. Restart reads committed result bytes; interrupted unpublished jobs fail explicitly.
`jobs.rs::settle_failure` preserves a visible terminal record after late persistence failure and
retains nondurable failure state when disk writes are impossible. Resource cleanup never erases
unknown ownership or calls failed removal a success.

## 5. Representative journeys and output assessment

Current actual A01/A02 outputs recover complete jobs and artifacts, distinguish local fixtures from
public registries and preserve project-availability uncertainty. A03 obtains breadth before depth
and uses real compile/runtime probes for default-feature claims. Its recommendation correctly
recognizes the square-only fixture, but its statement that no external explanatory documentation
exists is unsupported by a failed Context7 match; this reviewer does not accept that negative
inference as a library fact. The service preserves scope and does not publish agent prose as
extracted evidence. Likewise, the declared MSRV is not tested by a newer compiler.

Actual A04 Serde 1.0.228 research now encounters the two `serde::Serialize` definitions and
successfully selects the macro by returned definition ID. It separates unversioned Context7
1.0-series guidance from exact feature/dependency evidence, then completes a real derive compile
probe in the qualified synthetic environment. It retains untested no_std/target/project-lock limits.
A05 directly inspects the requested trait without enumerating the library and reports that its
signature-only result is a trait header, not a complete inventory of associated methods.

A06 has no enrichment server registration, acknowledges the missing service and distinguishes
its permitted direct fixture-file reads from service or runtime evidence. The Codex upgrade
brief identifies the new Python module/function, unchanged source/stub conflicts and complete
paged inputs, while preserving `partial`, `comparable: false` and unspecified-environment limits.
Both runtime briefs report the exact observed Python signature while keeping the disagreeing
stub and distinguishing introspection from actual function invocation. Claude's upgrade brief
completed comparison and attempted typechecks which failed explicitly on the deliberately
incomplete static fixture wheels; no successful typecheck was invented. Its overview discrepancy
exposed a genuine service defect, now fixed by native ranking which prefers documented observations
within the selected public binding. A deterministic native regression catches the former result.

Two other brief inferences are not accepted as facts: a startup Griffe status cannot establish
post-extraction unavailability, and an absolute fixture path inside a contained runtime does not
establish permission to write that path on the host. The service's qualification update and actual
containment tests govern those meanings. The client tier passed 11 checks before the overview
repair; its receipt remains tied to that earlier source. The correction passed its targeted native
and real Python checks plus full CI. The user explicitly stopped further replay. The source-backed
fixture cases additionally exercise Unicode docs, aliases/namespaces, source/stub disagreement,
release comparison, exact citations, explicit limitations and restart/offline reuse.

Publication interruption tests send actual SIGKILL at declared durability barriers, then require
coherent old/new visibility and no producer replay. Two independent adapters exercise surviving
interests when another caller cancels. Contained producer tests verify actual Python and Rust
operations before accepting unchanged canary bytes; actual cleanup failures quarantine admission.

A new inspection discriminator reuses existing identity, native query and generated DTO machinery.
It does not add a new pipeline. Changing a release or qualified producer input changes the relevant
identity; refreshing unchanged selection adds actual attribution without regenerating documentation.

## 6. Charter gates

These are design-review judgments, separate from the machine acceptance states.

| Gate | Verdict | Evidence / required closure |
|---|---|---|
| G1 authority | pass | One native store/catalog, core-owned identities; D1–D13 deletion checks and typed producer boundary |
| G2 semantic fidelity | pass | Qualified alternatives, source excerpts, full docs, exact selection, typed comparison; final briefs reviewed separately |
| G3 validity | pass | Native admission/corruption rejection, bounded worker failures, generated wire checks, path/ID mismatch rejection |
| G4 hidden effects | pass | Retained inspection starts no producer; explicit execution intent and policy; real six-case language/profile canary |
| G5 consistency/recovery | pass | Precommit delivery, actual SIGKILL/restart, surviving subscribers, failed cleanup quarantine, leases through exhausted streams |
| G6 transformation/reuse | pass | Native semantic identity, qualified producer dependency closure, exact offline reuse, same-context later snapshot preserving old bytes |
| G7 truthful claims | pass | Actual implementation/producer/client evidence is identified by source and scope; interrupted replay and unexecuted spill/performance work remain explicit |

## 7. Findings and obligation closure

Applicable principles concern identity/authority, typed semantics, transformations, explicit effects,
ownership, dependency-qualified reuse, provenance, generated boundaries and falsifiable claims.
Numeric approximation and historical migration do not apply: the model is discrete and this is an
explicit design-stage hard cutover. Performance measurement applies only at its recorded scope.

| Finding | Principles | Correction and oracle |
|---|---|---|
| Corrected: overview could hide existing source docs behind an undocumented stub | DM-08, DM-24, DM-42 | Native representative ordering; deterministic native and real Python overview regressions |
| Corrected: identical public-path candidates could not be selected | DM-06, DM-11, DM-16 | ADR-0035; real source/stub selection and restart regression |
| Corrected: successful paged research could be rejected after restarting a read | DM-24, DM-42, DM-53 | Cursor-chain validation; actual Codex upgrade trace plus restart regression |
| Corrected: scenario teardown could retain a false successful entry | DM-30, DM-54, DM-59 | One post-cleanup outcome, duplicate-summary rejection and retained-output hashes |
| Performance target deferred, not met or waived by a fabricated measurement | DM-39, DM-59 | Existing five-run receipt and explicit user prioritization; no tuning campaign required now |

| Obligation | Closure mechanism and evidence |
|---|---|
| O1 semantic model | Definition/binding/observation separation; typed unknown/external references and same-path alternatives |
| O2 Arrow fidelity | Typed Arrow tests, native admission and nested alternative comparison; no JSON evidence column fallback |
| O3 metadata | Explicit base/derived schemas and roles, projection/view/portable bundle checks |
| O4 relational integrity | Native key/FK/provenance admission; missing/changed bytes rejected before publication |
| O5 publication | Precommitted delivery, catalog generation, actual durability-barrier interruption/restart tests |
| O6 reusable admission | Warm admission identity, changed-file rejection and reader/provider/stream/export leases |
| O7 research | Native search/browse/inspection and usable actual breadth/depth and known-symbol clients |
| O8 comparison | Cold and pinned comparisons, partial/failed/cancelled/restarted journeys and typed alternatives |
| O9 resources | Worker allocation boundaries, query-result bounds, configured spill limits, decoder timeout/abort/reaping and actual owned-container cleanup |
| O10 physical behavior | Executed projection/scan/plan tests remain; performance tuning and broader variant campaigns deferred by user |
| O11 deployment/operations | Current three binaries, qualified real producers, installer recovery, clients, portable export and explicit cleanup |
| O12 evidence integrity | Source/command/log/native hashes, linked retained client traces, real test mapping and recorded user-stopped independent replay |

D1–D13 are individually mapped in the [execution ledger](../../plans/12-architecture-first-execution-ledger.md#d1d13-deletion-closure).
The current removal gate checks deleted modules, dependencies, Python engine imports and supported
native deployment consumers; scoped Rust rules guard the forbidden entry points. Ordinary bounded
DTOs, journals, test corruption helpers and specialized kernels are justified boundaries, not legacy
engines. The exact owned cutover removed old physical evidence roots after container reconciliation;
no migration reader, dual writer or engine selector remains in the supported build.

## 8. Alternatives and architectural leverage

| Choice | Consequence | Decision |
|---|---|---|
| Keep old corpus collectors/readers alongside native execution | Competing meaning and lifecycle paths, repeated correctness work | Removed |
| One typed native engine with bounded kernels and worker boundaries | Shared joins, selection, comparison, admission and publication; explicit failure ownership | Implemented |
| Add Delta, a second index or custom optimizer now | New state/identity machinery without a demonstrated functional need | Outside scope |

The blueprint §15 ergonomics questions are addressed by semantic MCP operations, bounded breadth/
depth discovery, producer/projection extensions, preserved source IDs and no excluded platform.
These are observed routes, not a claim that every external library is completely understood.

## 9. Verification

Full implementation CI passed **390 Rust and 189 Python tests** on source
`4b434e3f7fd604068cf5e59205d7a0a54de09b442ce1a0d3267913babfd0db0a`, followed by **12 real
contained cases** and **2 live cases**. Source/native receipts validated. The ten actual client
journeys passed eleven checks before the overview correction; the correction subsequently passed
its deterministic native regression, real Python/MCP journey and full CI. No failing implementation
case remains hidden by a registry change.

P03/C17 registry links were then corrected to existing target tests without changing their
assertions. That metadata-only edit changed the receipt source fingerprint. The independent auditor
reproduced all 390 Rust tests on the corrected source and reviewed all 152 registry references
(147 distinct); the user then explicitly stopped redundant full-suite replay. Its Python audit is
interrupted, not passed. The generated report retains strict source-binding rules and is not
manually promoted. This limits current-registry gate certification, not the completed functional
implementation's recorded validation.

Schema, lint, dependency policy, architecture/rule fixtures, provenance and state checks passed.
Local links, documentation spelling and diff whitespace were also checked. Detailed logs and the
interrupted audit boundary are in the execution ledger and STATUS.

## 10. Exceptions and limits

Performance tuning is deferred by explicit user instruction. The retained five-run optimized
workload had a 2.20897-second median, exceeding the former 1.5-second target; the target was not
changed and no pass is claimed. The shipped skill and frozen specification remain byte-identical.
The existing supported-scope limitations for external re-exports, hosted build configuration,
partial semantic support and environment-specific runtime observations remain explicit.

## 11. Decision

| Decision | Priority | Required action |
|---|---|---|
| Accept-scoped | completed functionality | Preserve recorded evidence and the user-stopped audit boundary; performance tuning remains deferred |
