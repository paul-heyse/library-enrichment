# Native operation and dataflow inventory

Source inspection **2026-09-17**, during authorized Plan 17 implementation. This inventory maps
current consumers and missing replacements; it does not define a second operation registry or
claim final behavioral acceptance. Rust declarations remain authoritative. Every row's remaining
work belongs to [Plan 17](../plans/17-schema-governed-unified-runtime-hard-pivot.md).

## Shared contracts and boundaries

- [ResearchRequest and research_operations!](../../crates/enrichment-core/src/request.rs) declare
  nine tools and the unpublished snapshot-manifest operation, their input/output types, RPC names
  and effect annotations. [Arguments](../../crates/enrichment-core/src/operation.rs) currently
  covers only four durable command kinds. The generated tool catalog is not yet a complete native
  operation/policy definition for all the routes below.
- [Server dispatch](../../crates/enrichment-daemon/src/server.rs) decodes those inputs, enters an
  owned native operation and calls handlers. Finite dispatch is generated; remaining semantic composition in handlers still needs native plans.
- [ControlStore](../../crates/enrichment-store/src/control.rs) projects 23 typed families from one
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
| `library_overview`: OverviewRequest → OverviewData | [overview handler](../../crates/enrichment-daemon/src/ops/overview.rs), SnapshotReader namespace/count/discovery/coverage queries over exact evidence tables | Read lease and selected snapshot/control vector; independent discovery continuations | Defaults/eligibility and coverage-qualified outcomes use shared native declarations. Remaining evidence and result composition. FP09/FP10; L09/L11/L21 |
| `search_evidence`: SearchRequest → SearchData | [search_plan](../../crates/enrichment-store/src/search_plan.rs) and native scoring use shared factors, rank/ties and key pages from the selected search projection | Exact snapshot/projection checkpoint and search key; read ownership | Native selection and cursor witnesses replace handler selection hashes/sort/dedup. [Handler](../../crates/enrichment-daemon/src/ops/search.rs) still assembles hits/evidence; result composition remains. FP02/FP09/FP10; L08/L09/L11 |
| `inspect_symbol`: InspectRequest → InspectData | [inspect handler](../../crates/enrichment-daemon/src/ops/inspect.rs), native symbol/navigation/text/callable/coverage reads; execution aspects enter durable Inspect commands | Exact definition/source/environment/aspect selection; claim and per-action grant for execution; retained observations remain epistemically distinct | Native binding/aspect order, execution-kind anti-joins, shared source-version policy and coverage-qualified outcomes are implemented. Result composition, runtime/LSP lowering and exact method/document/position scope remain. FP07–FP10; L05/L06/L09/L10/L11/L23 |
| `compare_releases`: CompareRequest → CompareData | [native comparison](../../crates/enrichment-store/src/comparison.rs), configuration contribution and changed-key plans; comparison publications bind both exact snapshots | Compare command when preparation is needed; before/after environment, evidence and retained-result dependencies | [Handler](../../crates/enrichment-daemon/src/ops/compare.rs) composition, page identity and complete field-level callable comparison. FP09/FP10; L04/L09/L11/L23 |
| `verify_usage`: VerifyRequest → VerificationData | [verify handler](../../crates/enrichment-daemon/src/ops/verify.rs), native execution policy, immutable process/effect definitions and result usability/publication | Verify command/claim/grant, exact environment and producer; capsule driver owns subprocess, sandbox, framing and cancellation mechanics | Command-to-argv/input/environment/output/network/budget enforcement across every driver, derived environments and physical crash/revocation closure. FP06–FP08/FP10; L02/L06/L10/L11 |
| `read_artifact`: ReadArtifactRequest → ArtifactSliceData | [artifact handler](../../crates/enrichment-daemon/src/ops/artifact.rs), native receipt authorization and retained result-section selection; immutable blob read on tracked blocking callback | Captured artifact descriptor/digest/length and catalog pin; selected result byte window | Native format/window/section/cursor decisions and durable protection through physical reads are implemented. Encoded page fitting, result composition and full artifact/result/export dependency closure remain. FP02/FP09/FP10/FP12; L09/L11/L22 |
| `job_control`: JobRequest → JobData | [Jobs](../../crates/enrichment-daemon/src/jobs.rs) delegates interest/state/claim/count/shutdown selection to [JobStore](../../crates/enrichment-store/src/control_jobs.rs); typed retained terminal replay | Command key, caller interest, claim fence/predecessor, policy and complete terminal result; cancellation signals owned work | Residual dispatch/policy, independent-process/lost-ack races, exact physical reuse/reconciliation and replay invalidation. FP06/FP08/FP10/FP11; L02/L10/L11 |
| `service_status`: StatusRequest → StatusData | [status_plan](../../crates/enrichment-store/src/status_plan.rs) selects typed component/readiness observations; native diagnostic aggregates supply counters | Runtime configuration/qualification/producer observations; no acquisition side effects | Remaining [status](../../crates/enrichment-daemon/src/status.rs) capability/readiness policy must share ordinary operation policy; full MCP outcome remains unqualified. FP06/FP09/FP13; L10/L12 |

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
| Replay/cache | Exact native command/result/projection records exist; restricted logical-plan cache consumer is absent | Implement fresh-process replay and real DeltaLogicalCodec allowlist/revalidation/live-handle rebinding. No physical-plan codec fallback. FP11; L09/L12 |
| Retention/control maintenance | Root/lease/run/obligation declarations → protected-version/log-floor/claim selection; fenced native control OPTIMIZE/checkpoint/log compaction | Native VACUUM and bounded log deletion have focused helper proofs. Typed configurable data/log/transaction horizons are natively admitted, projected to Delta creation properties and captured in maintenance rows. Explicit root removal, service-wide orphan/cohort reclamation and full policy-driven maintenance consumers remain. FP12; L01/L03/L12 |
| Operator qualification | Native execution policy and physical ownership records admit fixed qualification operations | Same capsule executor under narrow operator scope; does not authorize arbitrary caller actions. Complete real profiles/C20/revocation/crash evidence. FP08; L02/L10 |
| Capsule/storage cleanup | Native physical owner/storage reservation and absence observations → durable release/reconciliation | OS process/container/file removal stays a bounded driver. Storage release uses the retained release queue; old `owned/*.json` fixtures remain to replace. FP06/FP08/FP12; L02/L10 |
| Operator cache/evidence/development reset | [maintenance](../../crates/enrichment-daemon/src/maintenance.rs) takes Config, StatePaths and Scope; native ownership validation/recovery and physical drain precede a bounded inventory/digest Report and optional apply | Explicit scoped file removal holds daemon/cache/storage/exclusive evidence locks and rechecks inventory after all native writers exit. It is separate from native table reclamation. Control-selected inventories and complete preview/apply/paired epoch-marker qualification remain. FP12/FP13/FP15; L01/L12/L24 |
| Diagnostics | Generated Event union (query, operation, service, failure, index and all 20 kernel variants) → one native Delta history and native aggregates | Bounded reserved ingress, unobserved writer, tracked native descendants and joined actor close. Builder metrics/full lineage, retention and fault/pressure matrices remain. FP13; L12 |
| Shutdown | Stop/join transport roots, reconcile jobs/execution, drain tracked native work, release ownership, join diagnostics, drain final kernel children | [Runtime](../../crates/enrichment-store/src/runtime.rs) and [task context](../../crates/enrichment-store/src/task_context.rs) hold physical tokens independently of caller cancellation. Explicit startup/export supervisors own their close; a stuck callback causes a deadline failure, never a successful physical-exit claim. Full external-process/crash qualification remains. FP08/FP12/FP13; L02/L10/L12 |

## Inventory boundary

This closes the documentation mapping for FP00 action 1 at the inspected source boundary. It does
not close FP00: field-level domain/reference inventory, declaration-driven policy/dispatch for
every row, capability-to-route executable evidence, protected enforcement installation and the
one-real-extension/policy-change oracles remain. All L rows above refer to full Plan 17 obligations,
not permission to preserve a semantic handler under a different name.
