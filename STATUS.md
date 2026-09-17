# Implementation handoff

Updated **2026-09-17**. **Plan 18 is in progress; the pivot is not complete, qualified or installed.**
The active execution authority is [Plan 18](docs/plans/18-schema-governed-runtime-and-cache-completion.md),
including its [package ledger](docs/plans/18-schema-governed-runtime-and-cache-completion.md#10-execution-ledger-and-completion-rule).
It incorporates Plan 17's remaining FP/L/Q/SC obligations and the cache review. Continue implementation.

## Current boundary

Phase 6 remains the product integration phase. HEAD is `ccf5739`; the shared tree contains substantial
pre-existing uncommitted Plan 17 work and current Plan 18 changes. Preserve them. The execution
baseline and file hashes are `.dev-state/plan18/execution/baseline.json`; planning had a separate baseline.

Source identities are state **11**, snapshot **10.0**, wire **5.0**, codec **native-json/3**.
The new materialization diagnostics changed state identity; complete endpoint response budgeting
changed wire identity. There is no historical reader or migration. Fresh activation has not run.

Preserved foundations include generated native schema/union/value contracts and typed IDs,
producer/registry/revision facts, native results, nine generated tool bindings, composed providers,
23 atomic Delta control families, exact retention roots/leases/candidate obligations, maintenance
fences, kernel metrics and owned runtime/release/diagnostic tasks. All have remaining consumer or
qualification obligations; see Plan 17 §2 and Plan 18 §10 instead of re-deriving their source history.

Plan 18 now adds:

- **Operation-owned CacheFactory:** four declared reuse families, pure admission/planning, exact
  operation/input/session binding, once-only execution-owned fill, native spill/readback, failed
  terminal states and owned cancellation/deadline. Paid query permits survive abandoned waiters;
  leases/reservations follow physical readers. Mutable/volatile/unknown sources refuse before reuse.
  Native global statistics/order claims are forwarded only when valid after coalescing.
- **Restricted immutable Delta provider cache:** typed identity/version/cohort/contract/config/build
  keys; DefaultCache TTL/eviction; bounded native codec writer; native schema/feature/CHECK validation;
  narrow recorded vendor rebinding seam. Values contain no authority. Each read attaches fresh
  protection after its complete dependency vector is admitted. Provider/plan/stream reservations
  survive eviction. Persisted descriptors and fresh-process command replay remain open.
- **Exact delivery:** checked Page fields share Arrow/Serde validation. A full-field native UDF
  measures ResultRecord through the same format projection used at delivery; DataFusion selects
  inline eligibility and ranked retained views. Oversized candidates remain measurable. Rust owns
  MCP failure previews, terminal-job failure handling, retained links and evidence-resource text.
  Python's summary/preview fitting loop and fixed 1 KiB framing allowance are deleted.
- **Pinned SDK framing:** the adapter supplies exact stdio framing, protocol era and resource URI;
  generated service identity, modern result/cache fields and actual SDK byte equality are checked.
  Oversized IDs and non-stdio profiles refuse. Local transport errors and the packaged workflow
  guide are separate from native evidence-budget qualification. Four evidence resource templates
  and nine tools use the new native boundary.
- **Diagnostics:** typed materialization activities and cache-family counters replace old index
  events. Descriptor/cache ownership and native pool reservations are visible separately.

The old CompletedIndex/eager materialization/count channel/custom index IPC machinery and eager
index benchmark were deleted. Replacements are source implementations, not closed CF/application
qualification. Proposed decisions are ADR-0052, ADR-0053 and ADR-0054.

## Verification and known failures

**2026-09-17:** `just acceptance-report` and `just acceptance-check` regenerate and validate
**0 passed / 0 failed / 0 blocked / 48 not_run**. No focused unit/probe result is promoted into
registered acceptance. Source changes require regenerating this source-bound report again.

Focused receipts are under `.dev-state/plan18/execution/`:

- Operation cache: 9 units; restricted provider cache: 3 units; exact retention-vector negatives:
  1 unit; shared Page decoder: 1 unit; native transport/recovery: 2 units.
- `result-measure-units.log`: isolated Arrow codec and exact inline byte-boundary selection across
  envelope/tool/resource profiles. `mcp-framing-units.log`: 10 Python framing units.
- `mcp-native-parity.log`: 144 native/installed-SDK serialization comparisons, with no daemon or
  client session. Reproduction is `mcp_delivery_probe` plus `fastmcp-framing/native_parity.py`.
- Production core/store/daemon strict Clippy and workspace library/binary compilation passed at
  their logged source boundaries. Two pinned vendor Parquet deprecations remain unsuppressed.
- Generated schemas/DTOs and conformance passed: four valid fixtures, eight negative cases.
  Selected Python Ruff/ty and ADR lint passed. All three provenance seals now pass: the pre-existing
  tool-guide mismatch was resolved by recovering its exact frozen bytes and sealing the wire 5.0
  guide under a new dated bundle (ADR-0054), without editing old manifests/maps.
  DataFusion/FastMCP delivery capability scans are clean.
  Earlier plan-node syntax hints do not recognize the implemented DataFusion 55.1 statistics hooks;
  Delta vacuum hints refer to the explicitly configured isolated deletion-vector fixture.
- `just doctor` passed hard prerequisites; installation availability is not execution-profile proof.

**Known compile-only failure:** `workspace-targets-check.log` contains stale integration/libtest
fixtures for binary IDs, owned environment profiles, request defaults and removed resource APIs.
Production checks do not close this CP10 fixture work. No integration tests ran in this execution.

Historical unresolved application evidence remains in [Plan 17 §2.3](docs/plans/17-schema-governed-unified-runtime-hard-pivot.md#23-failed-and-unresolved-evidence):
navigation List metadata, execution/export artifact Struct metadata, Decimal diagnostics, nullable
document kind, result clock/window admission, diagnostic pressure/shutdown, and cleanup fixtures
expecting retired `owned/*.json`. Earlier publication/CDF/export passes predate current source;
their diagnostic drops and source limits remain visible. Nothing here closes final Q/SC/CF matrices.

## Next work

**Do not run full integrations until the architecture pivot AND all legacy source/package/installed
state/client-registration deletions are complete.** Units, isolated native library probes and
static/schema/compile checks remain authorized. Intermediate discontinuity is accepted.

1. Finish native handler/page/aspect/window/recovery composition. In particular,
   `ops/artifact.rs::read_blocking` still chooses an encoded prefix procedurally. It must move
   selection into native plans while preserving UTF-8 boundaries, forward progress and exact
   resource/tool framing. Do not expand every prefix into quadratic-size result candidates.
2. Complete remaining domain/digest/witness declarations, full semantic operator/storage/mutation
   coverage, durable acquisition/producer facts and exact process grants/warm-LSP ownership.
3. Complete retention consumers, especially read-only/export exact enrollment. Shared root guards
   still exist and are insufficient for that target. Finish explicit root removal, orphan/cohort
   reclamation, maintenance/crash fences, persisted descriptors, fresh-process replay and CDF proof.
4. Complete resource accounting/pressure/shutdown consumers, remaining L01–L25 deletions and stale
   typed fixtures. Preserve the protected enforcement boundary; prepare its patch separately.
5. Build a matching candidate and verify actual retired runtime/state/client removal (CP11).
   Only then run Q01–Q13/SC01–SC10/CF01–CF10, current clients/producers/Linux/C20, durability and
   performance qualification; repair failures and activate fresh state (CP12–CP13).

Workstation defaults remain **32 GiB native memory, 64 GiB spill, 2 GiB metadata cache, 1 GiB
immutable-provider descriptor cache, 16 partitions and 16 workers/blocking threads per lane**.

## Tools and pins

**2026-09-17:** `just doctor` passed all hard prerequisites; `just toolchain-check` inside doctor
confirmed Rust **1.98.1**. Doctor observed uv **0.12.15**, just **1.58.0**, Python **3.14.7**,
`ty` **0.0.80**, rust-analyzer **1.98.1** and Griffe **2.3.0**. bwrap/Podman/Docker are present;
installation is not profile qualification.

Current lock/source pins: DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**,
delta-rs **58f07cd62bfbce3649a7e1c87c696288068ae184**, buoyant kernel
**8ba063f8f84fec222000f66d40d70911d7c79675**, FastMCP **4.0.3**, MCP **2.2.0**,
PyArrow **25.0.1**, rustdoc-types **0.61.0**. Exact capability evidence lives in the pinned
DataFusion/Delta/FastMCP skills and [compatibility matrix](docs/architecture/compatibility-matrix.md).
Use those skills and targeted probes, not Context7, for these libraries.
