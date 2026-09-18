# Native operation and dataflow inventory

Source inspection **2026-09-18**, during authorized Plan 19 implementation. This inventory maps
current consumers and missing replacements; it does not define a second operation registry or
claim final behavioral acceptance. Rust declarations remain authoritative. The remaining work is carried by [Plan 19](../plans/19-unified-runtime-and-delta-cache-completion.md), including its inherited Plan 17/18 obligations.

## Shared contracts and boundaries

- [ResearchRequest and research_operations!](../../crates/enrichment-core/src/request.rs) declare
  nine tools and the unpublished snapshot-manifest operation, their input/output types, RPC names
  and effect annotations. [Arguments](../../crates/enrichment-core/src/operation.rs) currently
  covers only four durable command kinds. The generated tool catalog is not yet a complete native
  operation/policy definition for all the routes below.
- [Server dispatch](../../crates/enrichment-daemon/src/server.rs) decodes those inputs, enters an
  owned native operation and calls handlers. Finite dispatch is generated; remaining semantic composition in handlers still needs native plans.
- [ControlStore](../../crates/enrichment-store/src/control.rs) projects the declared typed families from one
  atomic Delta catalog. [Native Delta](../../crates/enrichment-store/src/native_delta.rs) owns
  captured providers, storage/read contracts, validated CHECK installation and complete writes.
  [Semantic analysis](../../crates/enrichment-core/src/native_analysis.rs) checks field domains
  before coercion; [admission](../../crates/enrichment-store/src/admission.rs) checks relation
  rules. None of those APIs grants a process or network effect by itself.
- [ResultRecord](../../crates/enrichment-core/src/operation/results.rs),
  [per-tool relations](../../crates/enrichment-store/src/result_relations.rs),
  [result catalog](../../crates/enrichment-store/src/result_catalog.rs) and
  [native delivery](../../crates/enrichment-store/src/result_delivery.rs) retain typed results,
  exact table versions, evidence/artifact references and delivery windows. The native JSON sink
  and generated FastMCP bindings provide presentation. Remaining handlers still construct
  semantic result objects before this common retention/encoding path.
- Exact witnesses currently combine context/snapshot identities, control generation, table ID,
  version, contract and cohort, plus operation/policy/producer identities. Release/Environment/Context/Snapshot IDs are binary; other semantic ID/digest families remain.
  Cursor bindings use declared native preimages and bind field/codec contracts plus operation,
  aspect, discovery and execution policy values. Complete dependency invalidation remains FP02.

The immutable `operation.declarations` schema shares generated operation, inspection, discovery,
execution-kind and effective retention-policy tables across sessions. Catalog/schema mutations and
table DML are refused; native selection/admission plans join those providers. Relation-specific
subject and coverage rules live in the admission registry, independent of presentation metadata.

## Nine public tools

Each output below enters the shared typed envelope/result boundary. The listed L rows identify
the behavior to remove when its replacement is deployed; their whole obligations remain open.

| Tool and declared input → output | Current native selection and durable dependencies | Effects and witnesses | Remaining semantic owner / deletion |
|---|---|---|---|
| `resolve_library`: ResolveRequest → ResolveData | [resolve handler](../../crates/enrichment-daemon/src/ops/resolve.rs), request admission, source/environment selection, native normalization and atomic publication; releases, contexts, environments, attempts, snapshots, selections and search projections | Resolve command/claim/policy; captured HTTP destination, producer/environment and publication vector; finite fetch/process drivers | Protected exact-version registry/revision captures now feed native selection and coverage. Remaining source families, source/freshness/environment branches, capture qualification and exact action scope. FP07/FP08; L04/L06/L07/L10 |
| `library_overview`: OverviewRequest → OverviewData | [browse](../../crates/enrichment-store/src/browse.rs) assembles namespace/count/ordered-child relations over the two declared materializations; shared research plans provide discovery/coverage | Read lease and selected snapshot/control vector; independent discovery continuations | Native paired counts, typed empty namespaces and bounded child assembly now replace the procedural projection. Complete consumer/property/physical matrices and actual MCP qualification remain CP04/CP06/CP12. L09/L11/L21/L25 |
| `search_evidence`: SearchRequest → SearchData | [search_plan](../../crates/enrichment-store/src/search_plan.rs) and native scoring use shared factors, rank/ties and key pages from the selected search projection | Exact snapshot/projection checkpoint and search key; read ownership | Native selection and cursor witnesses replace handler selection hashes/sort/dedup. [Handler](../../crates/enrichment-daemon/src/ops/search.rs) still assembles hits/evidence; result composition remains. FP02/FP09/FP10; L08/L09/L11 |
| `inspect_symbol`: InspectRequest → InspectData | [inspect handler](../../crates/enrichment-daemon/src/ops/inspect.rs), native symbol/navigation/text/callable/coverage reads; execution aspects enter durable Inspect commands | Exact definition/source/environment/aspect selection; claim and per-action grant for execution; retained observations remain epistemically distinct | Native binding/aspect order, execution-kind anti-joins, shared source-version policy and coverage-qualified outcomes are implemented. Result composition, runtime/LSP lowering and exact method/document/position scope remain. FP07–FP10; L05/L06/L09/L10/L11/L23 |
| `compare_releases`: CompareRequest → CompareData | [native comparison](../../crates/enrichment-store/src/comparison.rs), configuration contribution and changed-key plans; comparison publications bind both exact snapshots | Compare command when preparation is needed; before/after environment, evidence and retained-result dependencies | [Handler](../../crates/enrichment-daemon/src/ops/compare.rs) composition, page identity and complete field-level callable comparison. FP09/FP10; L04/L09/L11/L23 |
| `verify_usage`: VerifyRequest → VerificationData | [verify handler](../../crates/enrichment-daemon/src/ops/verify.rs), native execution policy, immutable process/effect definitions and result usability/publication | Verify command/claim/grant, exact environment and producer; capsule driver owns subprocess, sandbox, framing and cancellation mechanics | Command-to-argv/input/environment/output/network/budget enforcement across every driver, derived environments and physical crash/revocation closure. FP06–FP08/FP10; L02/L06/L10/L11 |
| `read_artifact`: ReadArtifactRequest → ArtifactSliceData | [artifact handler](../../crates/enrichment-daemon/src/ops/artifact.rs), native receipt authorization and retained result-section selection; immutable blob read on tracked blocking callback | Captured artifact descriptor/digest/length and catalog pin; selected result byte window | Native format/window/section/cursor decisions and durable protection through physical reads are implemented. Encoded page fitting, result composition and full artifact/result/export dependency closure remain. FP02/FP09/FP10/FP12; L09/L11/L22 |
| `job_control`: JobRequest → JobData | [Jobs](../../crates/enrichment-daemon/src/jobs.rs) delegates interest/state/claim/count/shutdown selection to [JobStore](../../crates/enrichment-store/src/control_jobs.rs); typed retained terminal replay | Command key, caller interest, claim fence/predecessor, policy and complete terminal result; cancellation signals owned work | Residual dispatch/policy, independent-process/lost-ack races, exact physical reuse/reconciliation and replay invalidation. FP06/FP08/FP10/FP11; L02/L10/L11 |
| `service_status`: StatusRequest → StatusData | [status_plan](../../crates/enrichment-store/src/status_plan.rs) selects typed component/readiness observations; native diagnostic aggregates supply counters | Runtime configuration/qualification/producer observations; no acquisition side effects | Component requirements now consume the same enabled execution routes as effect admission; the daemon captures facts and transports native selections. Full MCP outcome remains unqualified. FP06/FP09/FP13; L10/L12 |

## Resources and retained reads

The [FastMCP adapter](../../python/enrichment_mcp/server.py) is the current registration authority
for these resource templates. It forwards daemon results; template registration is still separate
from the Rust-generated tool catalog.

| Resource | Dataflow / identity | Remaining obligation |
|---|---|---|
| Workflow guidance | Packaged product skill text; no evidence query or effect | Keep packaged guidance consistent with generated tools and target architecture. L13/L14 |
| Artifact URI | Same artifact-read operation and captured receipt/window as the tool | One native page/authorization contract, exact bounded tool/resource parity. L09/L11/L22 |
| Context overview URI | Same overview input with exact context selection | Shared native selection/result semantics; current/fallback policy and witness qualification. L11/L13 |
| Snapshot manifest URI | ManifestRequest → ManifestData through [manifest](../../crates/enrichment-daemon/src/ops/manifest.rs); exact snapshot/control vector and native publication predecessor/CDF change query | Complete dependency protection and typed resource/result projection; never infer ordering from CDF timestamps. L11/L12/L22 |
| Job result URI | Job control/retained result recovery from its terminal publication | Read-only complete retained replay, missing-artifact refusal and exact result/section witnesses. L02/L11/L22 |

## Internal operations and physical owners

| Route | Native input/output and authority | Physical mechanism / remaining replacement |
|---|---|---|
| Startup/recovery | [Service](../../crates/enrichment-daemon/src/service.rs), native control/job/physical-owner records, captured process identity and [retention reconciliation](../../crates/enrichment-store/src/retention.rs) | Filesystem/process observations are inputs to native transitions. Finish all foreign/remote-owner, lost-ack and candidate/orphan recovery boundaries. FP06/FP08/FP12; L01/L02/L10 |
| Provider discovery/open | Captured table/store/session contract → bounded atomic listing/catalog/schema/table providers in [native_discovery](../../crates/enrichment-store/src/native_discovery.rs) | Root-qualified object-store access and native Delta opener. Complete all mutation/protocol/feature and concurrency oracles. FP05; L03/L14/L19 |
| Acquisition/producer normalization | Native HTTP/acquisition/producer facts → generated evidence schemas through native Rust/Python normalizers | Fetch, archive, parser and extraction worker remain mechanical boundaries. Persist remaining registry/source facts and qualify whole callable observations. FP07; L05–L07/L21/L23 |
| Publication/search maintenance | Candidate obligation and exact table vector → native admission, atomic publication roots and typed projection checkpoint | Native Delta writes/CDF scans; physical completion precedes settlement. Finish incremental/rebuild faults, candidate cleanup and full dependency witnesses. FP06/FP11/FP12; L01/L04/L12 |
| Immutable definitions | Native policy/process/effect records → exact immutable table/version/contract binding and atomic retained root | [Definition store](../../crates/enrichment-store/src/immutable_definitions.rs) enrolls before reads and records writer obligations before files. Full result/replay dependency closure remains. FP06/FP08/FP12; L09/L10/L12 |
| Result capture/recovery | Complete ResultRecord → per-tool Delta version, native result index/receipt and selected retained root | IPC/JSON/file offsets are encoding mechanics. Close all semantic handler/page consumers and output-before-obligation gaps. FP10/FP12; L03/L11/L22 |
| Export/verify | Context/snapshot selection → native transitive result/artifact closure, rebased exact destination vectors and [bundle](../../crates/enrichment-store/src/bundle.rs) inventory | Owned finite supervisor, bounded verified copying/encoding and staging publication. Finish all raw artifacts/retained-results, crash/rename and replay/export protections. FP10–FP12; L01/L04/L11/L12 |
| Replay/cache | Exact native command/result/projection records; typed in-process ingredients; persisted bounded CBOR/raw-IPC immutable provider descriptors for service checkpoints | Cold restoration checks durable metadata/history, exact namespace, contract, configuration and current read protection. Portable exports explicitly rebuild from native Delta bindings. Complete command replay/unknown acknowledgment and full invalidation matrices. CP02/08/09; L09/L12 |
| Retention/control maintenance | Root/lease/run/obligation declarations → protected-version/log-floor/claim selection; fenced native control OPTIMIZE/checkpoint/log compaction | Native VACUUM and bounded log deletion have focused helper proofs. Typed configurable data/log/transaction horizons are natively admitted, projected to Delta creation properties and captured in maintenance rows. Native reverse-dependency selection now retires roots with cleanup obligations in one control commit. Catalog visibility, rooted enrollment and cache invalidation consume that authority. Shared maintenance routes execute OPTIMIZE or protected-version VACUUM/log cleanup; completed unselected cohorts use native DELETE and partial obligation settlement. Incomplete writer directories, native artifact/definition retirement, durable maintenance selections and historical-floor admission are implemented. Export/orphan ownership and all physical fault/restart/horizon qualification remain. FP12; L01/L03/L12 |
| Operator qualification | Native execution policy and physical ownership records admit fixed qualification operations | Same capsule executor under narrow operator scope; does not authorize arbitrary caller actions. Complete real profiles/C20/revocation/crash evidence. FP08; L02/L10 |
| Capsule/storage cleanup | Native physical owner/storage reservation and absence observations → durable release/reconciliation | OS process/container/file removal stays a bounded driver. Storage release uses the retained release queue; old `owned/*.json` fixtures are deleted; private ignored qualification fixtures read native owners and reservations. FP06/FP08/FP12; L02/L10 |
| Operator cache/evidence/development reset | [maintenance](../../crates/enrichment-daemon/src/maintenance.rs) takes Config, StatePaths and Scope; native ownership validation/recovery and physical drain precede a bounded inventory/digest Report and optional apply | Explicit scoped file removal holds daemon/cache/storage/exclusive evidence locks and rechecks inventory after all native writers exit. It is separate from native table reclamation. Control-selected inventories and complete preview/apply/paired epoch-marker qualification remain. FP12/FP13/FP15; L01/L12/L24 |
| Diagnostics | Generated Event union (query, operation, service, failure, index and all 20 kernel variants) → one native Delta history and native aggregates | Bounded reserved ingress, unobserved writer, tracked native descendants and joined actor close. Builder metrics/full lineage, retention and fault/pressure matrices remain. FP13; L12 |
| Shutdown | Stop/join transport roots, reconcile jobs/execution, drain tracked native work, release ownership, join diagnostics, drain final kernel children | [Runtime](../../crates/enrichment-store/src/runtime.rs) and [task context](../../crates/enrichment-store/src/task_context.rs) hold physical tokens independently of caller cancellation. Explicit startup/export supervisors own their close; a stuck callback causes a deadline failure, never a successful physical-exit claim. Full external-process/crash qualification remains. FP08/FP12/FP13; L02/L10/L12 |

## Inventory boundary

This closes the documentation mapping for FP00 action 1 at the inspected source boundary. It does
not close FP00: field-level domain/reference inventory, declaration-driven policy/dispatch for
every row, capability-to-route executable evidence, protected enforcement installation and the
one-real-extension/policy-change oracles remain. All L rows above refer to full Plan 17 obligations,
not permission to preserve a semantic handler under a different name.


## Plan 19 retained row and physical release ownership

`TableVersion.row: Option<RowKey>` is the shared retained selection for cohort, result and definition
tables. `PendingRow` is a pre-write intent, never read authority. Native recovery resolves exited
writers; native anti-joins select unreferenced rows, and Delta DELETE/VACUUM perform reclamation.
Artifacts use the same root/lease/obligation closure and partial-dependency settlement; the final
unlink driver requires both the native maintenance fence and exclusive root lock. Receipt publication
and its standalone root are atomic. Physical file counts require successful deletion and directory
synchronization. Release capacity is reserved before readers, temporary inputs or quarantine owners
are admitted and retained until the corresponding physical release finishes.


## Native research continuation and coverage

Page boundaries, exact requested coverage and comparison interpretation are selected by `page_plan`,
`coverage`, `comparison_policy` and `comparison_page`. The scope/evidence mapping is one immutable
catalog declaration. Every cursor selection includes runtime and service-policy witnesses in addition
to its immutable inputs and request fields. Alternative byte costs are format facts supplied by a
borrowed-Arrow UDF; native cumulative windows select the prefix, and the sink verifies its byte witness.
Typed callable/field differences now use closed ComparisonValue and native field-fact set
operators. Remaining aspect/recovery composition and installed route qualification remain CP06 work. These owners do not authorize the CP11 integration barrier.


### Shared research presentation and borrowed delivery — 2026-09-17

`research_citations` owns the citation key/excerpt/sample plan for all current handlers.
`research_overview` derives coverage kinds from the immutable catalog and computes presentation
counts and limitations. `research_fragments` keeps fragment text/completeness paired, attaches
recovery conditionally, and selects truncation; `research_outcomes` qualifies empty pages.
Handlers supply exact inputs and recovery templates and encode the resulting records.

`native_transport` borrows arrays/scalars, formats declared fields and measures the common MCP
stream. It no longer decodes complete result DTOs or reserves an input-byte multiplier. Envelope
shape and native field projection share one declaration. Selected research and full RPC frames keep encoded bytes under native pool reservations through socket delivery, without a JSON Value reconstruction. Complete external allocation/lifetime coverage remains CP09.


### Native source and private-directory consumers — 2026-09-17

`research_inspection` supplies NULL-aware ancillary and exact content consensus. `research_source`
selects source coordinates, bounded windows, safe candidate suffixes and the observed member.
The physical parser preserves original line delimiters and executes only the selected read.

`private_directory` is shared by document input scans, worker Arrow facts, Rust package extraction,
Python distribution extraction, revision extraction and inspection. Each creates a durable obligation
before bytes, retains ownership through blocking tasks/providers/streams, and uses reserved releases.
Workers enroll independent direct-child process witnesses before receiving source-bearing requests.
Native cleanup refuses any surviving owner, including a child after creator exit. TAR/ZIP physical
bounds include implicit directories; deletion rechecks file/directory identities. The old unowned
staging provider, extraction Scratch/TempDir cleanup and unpacked cache accessor are removed.
Remaining CP05/07/09 work includes worker grant/supervision qualification, external memory accounting,
remaining orphan qualification and full crash/restart/close matrices.

`source_capture` centralizes file/document/media/source-role policy across all acquisition families.
`static_worker::lower/prepare` select and retain the exact Python worker effect from the current
claim and source inventory. Only its private Prepared value can reach OwnedChild; a changed
physical input inventory, wrong job/package/ecosystem or non-owned source/output is refused.
The handler copies bounded protocol bytes and consumes the selected Arrow output. Native grant
checks cover dispatch, active parsing and result usability. Child reaping and pending output-file
flush retain paid resources through physical completion; the Rust decoder shares the async child supervisor with separately declared parser admission. Final installed worker, source-lineage and crash qualification remain unrun.


`bundle::export` enrolls its sibling staging directory in primary native cleanup before bytes.
`PrivateDirectory::Export` shares multi-process recovery with producer inputs. `InputContext`
retains it through native task and release descendants. Target writers leave scope and final
writes drain before control/manifest sealing; the owned blocking publication retains it through
fsync and rename. The final destination is outside cleanup's directory identity. Actual export,
restart, crash and unknown-publication qualification remains CP12.


`research_resolution` reads protected metadata, coverage and acquisition relations for consensus,
distinct gaps and hosted evidence. Freshness is selected from actual registry receipts; no handler
rendering clock stands in for a lookup. Native array operations replace only the policy-owned
retained/reselected note. Raw metadata/coverage hydration APIs and ten-artifact handler limits are
removed. Whole-root maintenance additionally requires native physical quiescence before its
bounded filesystem driver can remove current payloads.


`rustdoc_decoder_plan` now retains exact decoder effects and validates generated request/report
contracts under the current claim. `documentation_plan` owns inventory page selection and Unicode
windows; `DocumentFact` generates the staging layout. `status_components` supplies one roster to
configuration and native status, and `status_plan::components` uses actual per-ecosystem route
availability. Parser address envelopes remain with physical children; file-metadata lookup counters
remain separate from reader validation and live external allocation. Source state is 19, decoder
protocol 3; installed/client/storage qualification and full CP09 accounting remain open.


### Native inspection/execution consumers — 2026-09-18

| Consumer | Native authority | Bounded physical boundary |
|---|---|---|
| Inspection aspect routes/events/full-text recovery | `research_catalog`, `research_inspection`, full-witness cursor encoder | Perform requested reads and record typed pages/failures |
| Retained execution reuse and context ambiguity | `inspection_execution_plan::sufficient/retained_choice/admit_execution` | Open/protect candidates, compare exact static inputs, dispatch selected request |
| Inspection publication and presentation | `inspection_execution_plan::summarize/render` | Store canonical bytes; encode native-selected result |
| Verification initial/settled result | `probe_plan::lower/settled` | Capture process facts; join actual cleanup before settled projection |
| Execution facts and ingress | `execution_fact_plan::lower/admit` | Generated bounded Arrow row ingress/egress; no handler fact identity/class policy |
| Prepared consumer dependencies | `capsule_input_plan` and exact prepared inventory | Bounded read, digest verification, owned blob write; reader lifetime retains cleanup permit |
| Semantic acquisition inputs | `ingest::bound_input_plan` | Preserve original acquisition receipts; no digest-first receipt map |

Runtime SQL uses one DuckDB dialect captured in the session witness. Full-field semantic admission
recognizes concrete lawful higher-order/collection functions and validates lambda bodies; metadata
erasure and cross-domain operations remain refusals. The obsolete producer scaffold, per-producer
parser, source dependency walker and silent status default configuration are deleted. Remaining
all-family/route/deletion and external-memory inventories stay open under Plan 19.


### Shared producer provenance and preparation — 2026-09-18

`producer_run_plan` owns qualified input/receipt roles, canonical maps, clocks/log requirements
and run composition for resolve, rustdoc, inspection and verification; control/ingest/attempt
admission shares its rules. Operational receipt exclusion uses the full native receipt identity.
Equal content from an independent source remains a separate acquisition.

`preparation_plan` uses bounded Cargo TOML syntax facts to select source admission, configuration
removal, package/version/library identity and consumer manifests. `producer_plan` owns environment
options and preparation readiness from the exact command, producer identity, exit and cleanup.
`python_distribution` owns source/stub/native-file mapping, metadata authority, headers and
metadata/filename/interpreter/WHEEL-tag agreement. It uses native parsed versions and set operators.
The archive driver only captures/extracts bounded bytes and performs selected I/O.

`OwnedBytes` keeps preallocated file buffers charged until their last owner drops. Shared preparation
callbacks retain cleanup/storage ownership through physical exit; captured distribution texts retain
a separate reservation. Deleted counterparts include BuildSpec, recursive Cargo policy, procedural
Python inventory/validation and the unbounded blob read. Complete parser/external allocation and
real-worker lineage qualification remain open. No integration barrier is implied.

The common `NativePlanner` now validates optimized logical meaning and physical child schemas/
invariants, including internal Delta builder consumers. Post-analysis admission complements the
pre-coercion check. This is not full physical-expression/operator qualification.


### Explicit immutable bundle and native export policy — 2026-09-18

`bundle_plan` selects exact checksum/file coverage, canonical ordering, catalog scope and the
complete deduplicated blob copy inventory. Physical capture/hashing and copying remain bounded
owned I/O. Live-source export enrolls exact durable artifact dependencies and current source roots;
its blocking copies retain those leases and primary staging ownership through physical exit.

After target writers and releases drain, an exclusive fence seals the target data root. The seal
belongs to the bundle's checksum inventory and `delta-evidence-bundle/2` contract. `ImmutableRoot`
is required by immutable control/evidence opening; the exact vector and seal survive native
provider/plan/stream retention. Normal blob and Delta mutation entry points reject sealed roots,
including already opened writers. Foreign table namespaces are refused before Delta mutation.
Root-lock-only captured-read and generic read-only repository APIs are removed. Control inspection
can read its own records but cannot grant dependent artifact/result reads. Bundles remain bytewise
unchanged by verification. Version 1 has no compatibility route.

Native/physical-mechanism units pass. Complete export/storage/crash/lifetime qualification remains
Plan 19 CP12 after the deletion barrier; this is implemented source, not activated acceptance.


### Shared positive storage and evidence proof cache — 2026-09-18

The runtime's native positive-contract cache also owns successful file, snapshot and attempt
validation proofs. It replaces three repository-local map/threshold implementations and uses
one existing byte budget. Proof keys include physical file/table incarnation, exact descriptor
and catalog identity, runtime meaning and effective validation limits as applicable. Current
read authority stays outside the cache. Warm attempt proofs still validate their physical bytes.
Authority invalidation clears the bounded proof cache; no inventory is copied. Proof/log-vector
reservations survive eviction while a caller owns the proof. Occupancy remains declared native
accounting, separate from allocator/RSS and unfinished external-allocation qualification.


### Native retained validation and owned reader descendants — 2026-09-18

`execution_documents` selects retained payload/document closure and coordinates in native plans;
`semantic_utf8_range` is the bounded syntax kernel. `snapshot_validation` composes native semantic
component, coverage, count and input-size admission. Exact artifact protection travels into hash,
document and retained-result readers; positive proof reuse does not replace it. Contract-verification
providers additionally retain their namespace flight and shared snapshot reservation.

Provider planning/execution and stream polling compose retention with the current physical input
scope. DataFusion async/blocking descendants inherit the owners through its existing task hooks,
including after cancellation of the caller or stream. The shared local object store captures file
ranges into pool-admitted `OwnedBytes`; `Bytes::from_owner` carries reservations through clones and
slices. File metadata/preconditions remain the upstream implementation, invoked on the I/O lane;
owned byte reads and synchronization use that lane's blocking pool. A full-file get is one lazy
admitted buffer, while selective range reads remain selective. Decoded metadata, predicate and
writer allocations are separate unfinished accounting work; none of these figures claims RSS.

### Native overview, acquisition and Arrow handoff — 2026-09-18

`browse` replaces the removed overview projection with native ordered aggregates and typed empty
collections. Operation materializations seal their logical base in their binding; their composed
extension planner prepares its physical child without a fill or consumer rewrite of the base.
`producer_run_plan` supplies both qualified physical input receipts and fact provenance.
`research_resolution::acquisition_presentation` supplies summary, coverage, ordered handles and
partial state for the three acquisition families; the native hosted-build plan owns fallback.

Retained result validation and bundle copying share one bounded dependency union across all selected
roots. Physical table counts, including execution tables, share the snapshot validation plan.
HTTP/cache and artifact range reads use paid shared bytes; Rustdoc/cache artifact reads carry exact
leases into callbacks. `owned_batch` transfers prepaid capacity into Arrow's own buffer reservations
at result/fold/cache-output boundaries. Buffer lifetime is independent of reader/stream lifetime.
These handoffs do not cover all decoder or external workspace, and complete lifecycle/pressure and
actual service qualification remain CP09/CP12 work.


Source metadata policy is native: typed PyPI/revision facts feed release identity/metadata,
single distinct import-root binding and commit/tree admission. Unknown external source fields
remain available in immutable raw artifacts. `control/admission.rs` owns publication replay,
selection predecessor conflicts, claim scope and transaction-key selection. The physical control
writer interprets its finite disposition and appends admitted Arrow rows. Unknown acknowledgement
reconciliation refreshes the same incarnation and uses complete semantic row fingerprints with
native occurrence counts. It does not infer success from a cache hit or transaction-marker TTL,
and a failed reconciliation cannot initiate a write retry. Actual fault/restart proof remains CP12.


The shared finite Arrow ingress is an immutable captured-batch provider with shared buffer claims.
It exposes no mutable MemTable handle and makes no unproved relational-constraint claim. Comparison
and search consume this same boundary; their real native reconciliation/ranking/count/page consumers
now have isolated Arrow/cache evidence. Comparison registers its typed alternative relations once and
uses one changed-key materialization for every consumer. Native input mutability refusal remains
part of cache admission. Installed, storage and retention qualification remain separate CP12 work.


Persisted provider digests are native SHA-256 extension values over FixedSizeBinary(32), shared
with cache verification witnesses. Schema declarations and derived plans use one bounded Arrow
shape traversal, including dictionary, union, list-view and run-end children; semantic manifests
capture these meanings independently of physical shape. Coverage requests are typed native input
relations: deduplication, acquisition fallback and exact execution-artifact reference admission
precede the shared qualified assessment. Cursor records retain their native witnesses and share
one bounded escaped-JSON/hex format boundary across all continuation families.


Catalog metadata has native row declarations for relations, fields, rules, conditions and bounded
inventory. There is no parallel encoder schema. Metadata tables are immutable captured providers
whose Arrow buffers retain pool claims. Reference and metadata field paths are segment sequences;
root lookup uses exact unqualified names and nested lookup uses native field access. Discovery
bounds report truncation and never grant constraints or evidence completeness.


### Field references and captured inputs

The Arrow field rule owns each relation reference, target segments and paired scope keys. Evidence
and control admission compile that declaration into DataFusion anti-joins in their bound namespace;
metadata discovery emits the same declaration. Nullable parents and collection members preserve
presence. The optimizer receives only separately proven primary/unique keys. Embedded evidence
references do not obtain control authority from sharing a physical value type.

Every store single/multiple-batch ingress uses the shared captured provider. Its constructor requires
the native memory pool, checks full schema equality and pays shared Arrow buffers through their final
reader. Generated declaration tables use the same constructor. The wrapper exposes no mutation
method and grants no row constraints; DataFusion continues to own scan and execution mechanics.

### Intrinsic field contracts

One native predicate compiler owns requiredness, vocabularies, value bounds, tagged presence and
collection rules for evidence and control. Presentation roles have no executable meaning. A type's
vocabulary and a field's contextual rule occupy distinct metadata keys and both must pass admission.
Set uniqueness uses native array distinct/cardinality; sequences retain order and duplicates.

Dictionary, run-end and list-view encodings are projected through Arrow's native cast kernels;
union children use native union selection plus DataFusion's active-tag predicate. The projection
UDFs preserve complete child Fields. Encoded storage never silently skips value/reference admission.
Run-end offsets remain storage mechanics; executable value rules belong to the values Field.
Schema shape admission, scalar semantics, Delta storage support and wire-format support remain
distinct checks: recognizing a container does not promise every sink supports its representation.

Manifest comparison takes captured DataFrames from the runtime. Core compiles the difference
relation; it does not create a mutable, unaccounted source during positive-cache verification.


## Native operational identity and diagnostic consumers — 2026-09-18

`JobId`, `InterestId` and `AttemptId` share one UUID declaration with distinct 16-byte Arrow
domains. Control/claim/producer/publication joins, SQL parameters and collection values retain
complete Fields. Attempt selection and acquisition maps no longer decode identity text. Wire,
path, transaction and diagnostic boundaries render canonical prefixes. Job and qualification
clocks are typed native timestamps; HTTP cache digest bytes use the native SHA-256 declaration.

Record construction, collection wrapping/unwrapping and scalar coalescing preserve declared
children while native DataFusion kernels perform selection and aggregation. Typed output
decoding distinguishes a derived projection from a source declaration: the source reference
scope remains mandatory, while a projection can omit siblings without erasing identity domains.

Both invariant execution paths share bounded native witness projection. Native telemetry now
retains the same `Diagnostic` used for delivery, including typed recovery actions and witness
sequence semantics. The separate Failure declaration and recovery JSON codec were removed.
Execution readiness constructs the same tagged recovery alternatives. Wire version ownership
is a native vocabulary, consumed by the generated adapter contract. Current source state is
22, snapshot 12.0 and wire 8.0. Physical and installed qualification remain in Plan 19.
