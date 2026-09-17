# DataFusion CacheFactory: capability investigation and target-design assessment

## 1. Decision and scope

**Recommendation:** integrate DataFusion 55.1's `CacheFactory` as the explicit entry point for
**operation-scoped intermediate materialization**, replacing the current procedural materialization
boundary. Preserve the existing Delta provider-cache objective and shared native file-metadata
cache as separate capabilities. **Do not adopt the upstream custom-cache example unchanged.**

**Review decision: Revise before implementation acceptance.** The integration is worthwhile for
architectural consistency and centralized admission. It is not an already complete cache backend,
and no speedup has been established. The proposed backend's lifecycle and equivalence remain
unresolved until implemented and qualified by the focused oracles below.

**Author:** Codex. **Verified:** 2026-09-17. **Scope:** CacheFactory, related native caches, existing
operation-index consumers, planner composition and the relevant Plan 17 changes. **Evidence:**
interfaces/source **Interface-checked**; fourteen isolated library assertions **Tested**; application
integration **Proposed**. This assessment does not change Plan 17's completion ledger, production
code, binding decisions or deployment. Implementation remains paused for this investigation.

### Method and coverage

Used Context7 `/apache/datafusion` for discovery, then the DataFusion skill's API index, pinned
example and local **55.1.0** source, with **Arrow 59.3.0**. Context7 supplied no exact 55.1 identifier;
its generic guidance was not treated as version proof. Reviewed current runtime/session setup,
operation indexes, search/comparison/browse consumers, immutable-definition capture, the composed
physical planner, native cache implementations and the Delta scan's actual metadata-cache access.

Executed actual DataFusion in a small standalone Rust probe using existing compiled dependencies.
Attacked execution timing, concurrency, retention accounting, schema substitution, property loss,
cache lifetime and semantic fingerprinting. The [evidence directory](evidence/datafusion-cache-factory-2026-09-17/README.md)
contains source, logs, exact commands, fingerprints, hashes and limitations. References **E01–E11**
below resolve through its source table; **P01–P14** resolve through its probe matrix.

No full integration tests, service qualification or workload benchmarks ran. No custom service
cache was built, so cancellation, spill/recovery, retention and complete-journey performance of the
proposal were not tested. Delta was inspected only at its current scan/cache integration boundary;
this is not another Delta capability review. FastMCP and external wire contracts are unchanged.

## 2. Authority and lifecycle map

Three different cache surfaces must remain distinct:

| Surface | Authority and identity | Existing state | Recommended role |
|---|---|---|---|
| Operation materialization | One immutable admitted input binding, output contract and operation owner | `operation_index::materialize` writes native spill IPC and exposes a private StreamingTable | `CacheFactory` produces an inspectable logical node; one execution-owned fill supplies its readers |
| File statistics/listing/footer caches | Native file metadata and the cache-specific validity rules | Shared RuntimeEnv; Delta next scan consumes its file metadata cache | Continue native CacheManager/DefaultCache configuration and instrumentation |
| Immutable Delta read-provider cache | Exact table/version/cohort, semantic contract and current authority | Plan 17 FP11 specifies restricted DeltaLogicalCodec consumption | Persist only eligible provider descriptors; revalidate and reacquire retention/runtime handles on use |

An operation result cache is derived, disposable state. It is neither publication authority nor
a durable job claim, result record or replay command. Input retention remains authoritative until
all physical work/readers exit; eviction or caller cancellation is not proof of physical exit.

The existing [operation index](../../../crates/enrichment-store/src/operation_index.rs) already has
one owner and computes once for its readers. Search (`search_plan.rs:273`), comparison
(`comparison.rs:328`) and browse (`browse.rs:60,88`) consume four declared families: SearchIndex,
ComparisonKeys, OverviewChildren and OverviewNamespaces. The factory would expose that reuse in
the native plan; it does **not** newly eliminate four independent cache implementations.

Immutable-definition/control capture has a different purpose: the same captured value participates
in authoritative hashing and publication. Do not replace it with opportunistic cached recomputation.

## 3. Semantic contracts and invariants

### What CacheFactory actually supplies

Canonical interface: `datafusion::execution::session_state::CacheFactory` (E01):

```rust
fn create(&self, plan: LogicalPlan, session_state: &SessionState)
    -> datafusion_common::Result<LogicalPlan>;
```

Register with `SessionStateBuilder::with_cache_factory(Some(factory))`. Existing-state construction
preserves the factory (P06), which fits our shared session template and isolated operation catalogs.
The factory is invoked by explicit `DataFrame::cache()` calls. It is **not automatic common-subplan
memoization**, a SQL result cache or an object-store cache. Returning an ordinary logical plan is
legal (P02); returning an extension requires a planner that understands it (P05).

| Capability / limitation | Exact evidence | Target consequence |
|---|---|---|
| Replace the cache logical plan and refuse admission before execution | E01/E02; P02/P07 | Central admission for a finite declared cacheable family |
| No required async fill, backing store, TTL, eviction or single-flight behavior | Trait only returns LogicalPlan; example implements these concerns itself | Factory is an extension seam; backend and ownership still need implementation |
| No automatic output-equivalence check | P08 accepts a factory's Int64-to-Utf8 schema replacement | Enforce the generated semantic output contract at admission and lowering |
| Default `.cache()` eagerly collects partitions into MemTable | E02; P01 | Not suitable as an implicit lazy materialization node |
| Default retention is outside the pool's reservation accounting in the tested path | P09: retained array accounting 4,195,072 bytes; 1,024-byte pool; zero reservation | Keep explicit retained-buffer ownership/reservation; a large pool alone does not govern that path |
| Tested Arrow field metadata survives the default path; keys/order annotations do not | P09/P12 | Distinguish schema metadata from provider constraints, ordering, partitioning and validity guarantees |
| Custom logical nodes can remain visible to native analysis, optimization and EXPLAIN | Interface-checked via example and existing NativePlanner | Make materialization an explicit plan boundary, with execution-only effects |

P09 measures Arrow buffer accounting, not RSS, and begins with preallocated source buffers. It
does not establish that arbitrary operators ignore their memory pool. Its tiny cap is a diagnostic
fixture. Keep the workstation-oriented **32 GiB native pool, 64 GiB spill and 2 GiB metadata-cache
defaults** (E11); no lower production limit is recommended.

### Required application contract — Proposed

| Invariant | Representation / enforcement | Failure behavior and oracle |
|---|---|---|
| Only declared finite intermediates are cacheable | Generated family contract plus a typed bound request visible in the plan; factory admits the initial four families | Refuse unbound/unknown families, command/effect plans and unsupported inputs before execution |
| Reuse belongs to one captured meaning | Operation identity plus immutable input versions/cohorts, output semantic contract and relevant function/policy/build witnesses | Wrong-operation use refuses; altered semantic dependencies obtain a new binding |
| Consumer rewrites cannot alter the shared base | Stable admitted binding survives equivalent rewrites; consumer predicates/projections/limits remain outside materialization unless proven safe | Filter-first then full/count reader must return the full admitted relation to the latter |
| Metadata and physical properties are truthful | Existing generated schemas/contract checks; validate lowering; advertise only established ordering/partitioning/constraints | Wrong domain metadata or unsound property claims fail a focused conformance test |
| A ready entry means a complete successful fill | Owned state distinguishes unstarted, filling, ready, failed and cancelled; failures never mean empty success | Mid-stream error exposes no partial ready entry; all waiters see the defined terminal result |
| Resource and retention lifetime follows physical work | Existing operation/physical-exit ownership, memory reservations and native DiskManager spill ownership | Dropping one waiter cannot free buffers/leases in use by another; operation shutdown drains fill |

Use the existing family/schema/policy declarations as authority. `CacheFactory::create` has no
family argument, so the bound request must be carried explicitly in the input node or immutable
session binding; do not infer eligibility from table names, field names or ambient mutable globals.
An operation-unique handle with captured dependencies is sufficient for this scoped reuse; a new
daemon-global semantic-plan hash registry is unnecessary.

## 4. Derivation and execution design

### Recommended native route

1. Bind the admitted finite intermediate and its input/semantic witnesses. `.cache()` delegates to
   the installed factory, which constructs the materialization node **without polling input**.
2. Analyze and optimize normally with the shared-base boundary intact. Equivalent node rebuilds
   retain the same binding; meaningful changes require a new one. Cache keys never depend merely
   on formatted SQL, a schema's physical shape or the example's bare `LogicalPlan` hash.
3. Add a cache `ExtensionPlanner` to the existing
   [NativePlanner](../../../crates/enrichment-store/src/arrow_contract.rs), alongside contract,
   retention, command and Delta planners. It constructs a physical plan only. Do not install the
   example's standalone QueryPlanner in place of this composition.
4. On execution, acquire or join one owned fill for that binding. All physical plans built from
   the same admitted logical handle must share it, including count/page and concurrent readers.
   Fill through existing native execution/spill mechanisms. Do not recursively call an admission
   path that reacquires the outer operation's capacity-one permit.
5. Make the completed relation readable only after a successful full fill. Readers retain their
   backing buffers/spill and leases. Count and pagination are DataFusion operations over the same
   relation; retire `CompletedIndex.rows` as a separate result channel when consumers are switched.
6. Release the operation-owned materialization after physical work and readers finish. It does not
   survive restart, require durable cache replay or participate in publication authority.

The backing representation can remain native spill IPC initially. A reservation-backed Arrow
memory tier with native spill overflow may improve latency on this workstation, but that is a
separate physical choice to measure. Do not claim a performance benefit from the factory hook
alone, or add a multi-tier caching platform before the existing four consumers need it.

### Why the upstream example must change

The skill's `CacheNodePlanner::plan_extension` calls `collect_partitioned(...)` before returning
the physical plan (E03). P03 executes the input during physical planning; **P04 also executes it
during EXPLAIN without ANALYZE**. Its lookup/fill/insert sequence permits overlapping misses;
P10 evaluates the same input twice. Its map lives with the example query planner, and P11 shows
entries outliving all DataFrame handles. Those are concrete behaviors of the example, not inherent
requirements of the CacheFactory trait or a finding that our current runtime already uses it.

### Additional native cache leverage

`datafusion_execution::cache::cache_manager::CacheManager` owns three separate caches (E04):
file statistics (default **20 MiB**), file listing (**1 MiB**, no default TTL), and embedded file
metadata (**50 MiB**). Our RuntimeEnv overrides metadata to **2 GiB**; the actual Delta next scan
passes this cache to `CachedParquetFileReaderFactory` (E10). This is already integrated capability.
Do not infer that Delta also consumes ListingTable's statistics/list caches from their availability.

`datafusion_execution::cache::default_cache::DefaultCache<K,V>` supplies byte-accounted LRU,
optional insertion TTL, an injectable clock, limit/TTL updates, entry inspection/hits, clear/remove
and optional table-scoped invalidation (E05; P13 covers the non-TTL subset). Its `CacheKey` and
`CacheValue` contracts let application entries use it without implementing a second LRU algorithm.
Consider it for FP11's bounded in-process descriptor lookup if that consumer needs one. It does
not provide Delta persistence, semantic validation or shared asynchronous initialization.

Eviction removes the registry's reference. A reader's cloned Arc remains alive (P13), so cache
byte occupancy is not a bound on every live allocation. A retained value must own its reservation
until its last reader exits. TTL expiration is not retention release or semantic invalidation.
Avoid adding LRU machinery to a four-family operation cache whose operation lifetime already
provides the required disposal boundary.

`datafusion_execution::cache::SchemaFingerprint` deliberately ignores top-level field/schema
metadata for file-statistics validity (E06; P14). It cannot substitute for our generated semantic
contract identity. Context7 also surfaced CLI cache-inspection table functions; their registration
in our service was not established. Use the inspected native `Cache::list_entries`/limit surfaces
for proposed typed telemetry rather than presuming those SQL functions are available.

## 5. Representative journeys

| Journey | Required behavior |
|---|---|
| Search count plus cursor page | Bind the score/key relation once; fill once at execution; count and filtered page read the same captured input. Factory registration alone does not make separate bindings share |
| Add another reusable finite intermediate | Add eligibility/output contract through existing declarations and compose the same node; no new per-tool cache implementation |
| Change a policy or semantic domain with unchanged Arrow physical types | New semantic binding; no reuse merely because SchemaFingerprint or physical schema matches |
| Explain or reject a plan | EXPLAIN without ANALYZE and admission/optimization produce no cache fill, command effect or upstream batch evaluation |
| Concurrent readers and abandoned waiter | One operation-owned fill; one dropped waiter does not cancel others. Operation cancellation owns cleanup, and failure never publishes a partial ready entry |
| Spill-backed versus memory-backed reads | Same rows, nulls, field metadata and declared ordering; backing layout is not semantic authority |
| Process restart | Disposable operation entries disappear. FP11 reconstructs separately permitted immutable providers and reacquires retention; it never revives an old operation cache handle |

## 6. Acceptance gates

These are design-review gates for **the proposed integration**, not the repository's acceptance IDs.
No gate pass below certifies production behavior that has not been implemented.

| Gate | Verdict | Evidence / scope rationale | Required action |
|---|---|---|---|
| G1 — Authority | Pass, scoped design | §2 assigns disposable operation state, native file metadata and durable provider descriptors distinct owners; existing family declarations remain authority | Keep this separation in implementation; no second policy registry |
| G2 — Semantic fidelity | Unresolved | P08/P12 show the hook/default path alone does not establish schema/property fidelity | Implement contract-preserving node and check lowerings/consumer equivalence |
| G3 — Validity | Unresolved | P07 proves the rejection seam exists; service eligibility and wrong-owner checks are not implemented through it | Bind explicit family/inputs and add negative admission tests |
| G4 — Hidden behavior | Unresolved; upstream example fails | P03/P04 execute input during planning/EXPLAIN; proposed execution-time replacement is not built | Zero-input-poll oracle at every preparation stage |
| G5 — Consistency and recovery | Unresolved | P10 duplicates concurrent fills; cancellation/failure/physical-exit behavior of replacement is untested | Owned shared fill, completion boundary and fault/cancellation matrix |
| G6 — Transformation and reuse | Unresolved | Bare example key and default property loss do not prove required reuse semantics; P14 rules out a tempting physical-only key | Rewrite, dependency-change, count/page and representation oracles |
| G7 — Truthful capability claims | Pass, scoped investigation | Exact interfaces and fourteen library probes establish the limited claims made; performance and application guarantees explicitly remain Proposed | Keep custom backend and full-journey claims unqualified until their oracles run |

## 7. Principle findings

| Finding, severity | Principle IDs | Concrete evidence / gap | Consequence | Proposed correction | Verification |
|---|---|---|---|---|---|
| F1 High: example materializes in inspection | DM-20, DM-28 | E03 calls `collect_partitioned` in `plan_extension`; P04 observes input evaluation | EXPLAIN runs input computation and consumes resources; a permitted effectful input could have consequences | Plan-only factory/planner; execution-owned fill | Existing P03/P04; future unit oracle asserts zero input polls for bind/analyze/optimize/physical-plan/EXPLAIN |
| F2 High: neither hook nor default path enforces full service contract | DM-07, DM-22, DM-24, DM-42 | P08 replaces schema; P12 drops key/order annotations | Wrong output can be admitted or required semantic guarantees lost if the hook is treated as validation | Reuse generated output checks and truthful physical properties | Existing P08/P12; future unit negatives for domain, nullability, keys and ordering across rewrites |
| F3 High: example lacks operation-owned shared initialization | DM-29, DM-30, DM-35 | P10 double evaluation; P11 entry survives DataFrame drops; no cancellation ownership in example | Concurrent misses duplicate expensive work; retained values/leases can outlive intended scope | One owned fill per admitted binding, explicit terminal state and physical-exit cleanup | Existing P10/P11; future unit tests with interrupted source, dropped waiter and capacity-one owner |
| F4 High: convenience keys/accounting are insufficient | DM-31, DM-32, DM-45 | P09 unreserved retention; P13 live value after eviction; P14 equal physical fingerprint despite domain change | Apparent eviction/cap compliance does not release live allocations; incorrect semantic reuse can bypass changed authority | Complete captured dependencies; value-owned reservation/lease; no old-handle authority revival | Existing P09/P13/P14; future dependency-by-dependency and last-reader-release units |
| F5 Medium: current materialization boundary remains procedural | DM-18, DM-25, DM-27, DM-56 | E07 materializes before returning a provider; search/comparison read `CompletedIndex.rows` | Reuse and count acquisition are not one inspectable native operation structure | Replace entry point with factory/node; native aggregate over shared relation | Future count/page differential unit and source deletion check; no present regression rule enforces this route |
| F6 Observation: native bounded caches can avoid a new eviction subsystem | DM-57, DM-58 | E04/E05/P13 provide generic LRU and inspection; Delta already consumes footer cache | A new bespoke LRU would repeat library mechanisms; a universal cache would mix lifetimes | Reuse DefaultCache only where a concrete bounded lookup needs it | P13 demonstrates available mechanisms; no new global cache proposed |

**Applicable-principle verdicts:** satisfied here means satisfied for this investigation's scoped
claim, not for the unfinished service implementation.

| Principles | Verdict and boundary |
|---|---|
| DM-03, DM-04 | Satisfied at interface/assessment scope: physical backends can vary; custom behavior and limits are explicit |
| DM-02, DM-11, DM-12, DM-13, DM-23 | Unresolved for new implementation: §2 specifies owners and distinct identities; integration has not enforced them |
| DM-06, DM-07, DM-08, DM-19, DM-22, DM-24, DM-28, DM-29, DM-30, DM-31, DM-32, DM-35, DM-40, DM-42, DM-43, DM-44, DM-45, DM-46, DM-48 | Unresolved for proposed backend: §3–§5 specify contracts, rejection, lineage and lifecycle; service oracles remain open |
| DM-20 | Violated by the upstream example if adopted as-is (P04); unresolved for the replacement |
| DM-16, DM-17, DM-18, DM-25, DM-26, DM-27, DM-36, DM-50, DM-56, DM-57, DM-58 | Unresolved implementation recommendations; existing declarations/native physical mechanisms identify a bounded route rather than a new platform |
| DM-39, DM-59 | Satisfied for this review: library observations have receipts; latency/RSS/throughput benefits remain hypotheses |
| DM-53, DM-54, DM-60 | Satisfied for library characterization through P01–P14; unresolved for new service behavior and durable regression coverage |

**Applicability:** authority/types/revisions, composition/lowering, effects/concurrency, physical
representation/provider boundaries, provenance and verification all bear on this narrow cache
decision. Durable publication/migration protocols, domain relationship modeling, Python conversion,
cross-language wire generation and product discovery are unchanged and not re-reviewed. The review
does not convert that exclusion into assurance about the rest of Plan 17. No maturity score is useful.

## 8. Alternatives and architectural leverage

| Alternative | Duplication / locality | Correctness and operational risk | Cost | Performance evidence | Decision |
|---|---|---|---|---|---|
| Current operation_index | Already one materializer and four declared consumers | Existing bounded spill/lifetime protections; eager execution remains outside the native plan | Lowest immediate change | No new comparative benchmark | Baseline for comparison; preserve useful native spill primitives during replacement |
| Factory + one operation materialization node | One native admission/plan boundary; existing declarations stay authoritative | Requires shared fill, contract fidelity and cancellation implementation | Moderate, concentrated in runtime/index/planner and consumers | Hypothesis only | Recommended target, subject to §9 oracles |
| Simpler: ordinary DataFrame `.cache()` plus cloned handle | Very little code | Eager/unreserved retention and property loss; does not meet current governed lifetime contract as-is | Low code, additional controls still necessary | P01/P09/P12 characterize behavior, not workload speed | Reject as complete target replacement |
| Factory returning an existing ordinary provider scan | Avoids a new logical-node planner when provider contract is sufficient | A pre-materialized scan preserves eager work outside the plan; a lazy provider still needs the same shared state and admission | Potentially smaller physical adapter | Unmeasured | Valid bounded fallback, but no architectural gain if it only wraps the existing eager call |
| Daemon-wide plan/result cache | Broad reuse possibilities | New invalidation/authority/retention complexity; example key/lifetime insufficient | High | No demonstrated cross-operation need | Do not introduce |

The extension is justified by existing repeated-reader intermediates and the requested DataFusion
pivot. Specialized buffering, Arrow IPC, reservation and state synchronization remain ordinary
Rust mechanisms behind the contract. They do not need another DSL, engine or cache-policy registry.

## 9. Verification and measurement plan

| Claim / risk | Evidence now | Required oracle and expected result |
|---|---|---|
| Factory lifecycle and native cache semantics | Tested P01–P14; source E01–E06 | Retain this version-specific evidence; change pin only with renewed characterization |
| No preparation-time fill | Example fails P03/P04; replacement Proposed | Focused unit: zero polls/side effects at admission, analysis, optimization, physical planning and plain EXPLAIN; execution fills exactly once |
| Shared-base correctness under rewrites | Proposed | Filtered first reader followed by full/count reader; projection/limit/order changes; separately planned clones share one complete base; compare exact expected fixture rows |
| Semantic admission/invalidation | Proposed | Refuse effects/unsupported volatile or unbound mutable inputs and wrong operation; alter each schema/policy/function/snapshot witness separately |
| Shared initialization and cleanup | Example concurrent miss tested; replacement Proposed | Delayed source, two readers, one cancelled waiter, upstream failure, capacity-one runtime, owner cancellation; no duplicate fill, partial ready entry or premature release |
| Backing store and ownership | Default retention/LRU observed; replacement Proposed | Memory pressure, spill quota, corrupt IPC, final reader drop and physical descendants; reservations/leases survive until physical completion and then release |
| Planner composition | Existing composed planner Interface-checked | Focused contract/retention/command/Delta/cache lowering cases preserve each route; missing cache planner refuses |
| Performance and product behavior | Not run | After pivot/deletions: existing versus replacement complete search/comparison/browse journeys, cold/warm conditions, fill/replay counts, p50/p95, pool/RSS, spill/I/O, planning and diagnostics overhead |

Use unit tests during the pivot. Full integration and installed MCP journeys remain behind the
user's completed-pivot/deletion barrier. No new full-suite stop is introduced by this recommendation.
Measure the whole count/page operation, not just cache hits; current materialization already fills
once, so a faster memory tier must justify its extra retained memory and cannot claim saved fills
that the baseline already avoided.

## 10. Exceptions and unresolved decisions

No MUST-level exception is requested. Adoption of the illustrative planner is rejected for this
service's inspection contract. The replacement is **not accepted as implemented** while G2–G6
remain unresolved. No binding decision is changed by this review, and no ADR or plan ledger is
silently amended.

The operation lifetime and four-family scope are recommended now. Initial backing choice remains
an implementation decision: native spill IPC is the simpler preservation of existing guarantees;
reservation-backed memory with spill overflow is the performance candidate. FP14's complete-journey
measurements decide whether the added tier pays for itself. Cross-operation result reuse remains
excluded unless a concrete repeated workload and complete dependency/retention contract justify it.

Owner: the Plan 17 implementation. Revisit triggers: broader eligible input families, cross-operation
reuse, a DataFusion upgrade, or measurements showing materialization cost dominates the operation.

## 11. Decision and implementation changes

**Revise the target design to adopt the scoped factory route, then implement it with the existing
pivot.** The hook is useful; the example's backend semantics are unsuitable. Unresolved application
gates prevent an acceptance claim, not a recommendation to use the verified extension interface.

The following are concrete proposed additions to
[Plan 17](../../plans/17-schema-governed-unified-runtime-hard-pivot.md), in dependency order.
The plan file itself is unchanged by this investigation.

| Priority | Proposed plan change | Principles | Acceptance evidence / regression protection |
|---|---|---|---|
| 1 — FP01/FP02/FP05 | Declare operation-materialization eligibility/binding through existing family and semantic contracts; install CacheFactory once in the runtime template; explicitly reject unsupported/effectful inputs | DM-07, DM-19, DM-22, DM-32 | Wrong family/owner/schema/dependency tests; factory survives bound-session creation |
| 2 — FP05/FP13 | Compose a cache extension with NativePlanner and implement execution-only, operation-owned shared fill; retain permits/leases/reservations through physical exit | DM-20, DM-28, DM-30, DM-35, DM-45 | No-poll preparation oracle; concurrent/cancelled waiter, capacity-one and spill/failure units |
| 3 — FP09/FP14 | Route SearchIndex, ComparisonKeys, OverviewChildren and OverviewNamespaces through the same cached logical handle; express counts/pages in native plans | DM-18, DM-24, DM-25, DM-27 | Exact rows, counts, filters, ordering, one-fill/repeated-reader differential tests |
| 4 — FP11 | Keep DeltaLogicalCodec immutable-provider cache separate; use native DefaultCache only if a bounded descriptor lookup is needed; complete semantic revalidation and fresh handle/retention binding | DM-12, DM-31, DM-32, DM-43 | Existing Q08/SC08 obligations plus bounded lookup/invalidation tests; physical fingerprint never grants semantic reuse |
| 5 — FP13/FP14 | Record cache fill/wait/read/failure and retained/spill bytes through existing typed telemetry; inspect native cache entries/limits where useful; measure optional memory tier | DM-36, DM-39, DM-50 | No double-counted buffers or eviction-as-release claims; final complete-journey comparison after deletion barrier |
| 6 — FP15 | Delete replaced procedural materialization entry points, CompletedIndex row-count side channel and obsolete consumer paths/tests; retain native mechanisms that implement the target | DM-56, DM-57, DM-60 | Source/import/deletion audit; no parallel legacy cache path or compatibility mode |
| 7 — FP16 | Include new cache contracts in final native-resource, semantic-schema, replay and MCP journey qualification | DM-53, DM-54, DM-59 | Matching final-source receipts only; isolated library probes never substitute for product acceptance |

This is a focused addition to the unified runtime, not a reason to restart the pivot or build a
general caching platform. It makes the existing materialization policy and execution boundary
visible to DataFusion while retaining the ownership guarantees that the hook itself does not supply.
