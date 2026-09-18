# Implementation handoff

Updated **2026-09-18**. **Plan 19 remains active and incomplete; the pivot is not installed or
qualified.** Authority: [Plan 19](docs/plans/19-unified-runtime-and-delta-cache-completion.md),
its package ledger and implementation checkpoints. Inherited Plan 17/18 requirements remain.

## Execution boundary

- Phase 6; **CP00–CP13 remain open**. Report/check at **2026-09-18T22:16:02+00:00**:
  **0 passed / 0 failed / 0 blocked / 48 not_run**. Source:
  `32738daebe00f615729a05bd44d9eaf6460231316daa43f9389e2b20721972ae`. Scoped units are development evidence.
- Shared HEAD `efc646c`, checked 2026-09-18. Preserve extensive preexisting dirty work, including
  independently updated DataFusion/Delta skills. No source reset or commit. This slice's baseline
  is `.dev-state/plan19/execution/typed-delta-baseline/source.tar`; its changed Rust inventory is
  `.dev-state/plan19/execution/typed-delta-rust-paths.txt`.
- **No full integration, service storage/publication/CDF/export/restart journey, real/installed
  client or producer qualification, or full CI until actual CP11 architecture/source/package/
  enforcement/install/state/client deletion closure.** Unit/static/schema/compile checks permitted.
- Source: **state 26 / snapshot 16.0 / wire 12.0 / native-json/3**, executor **9**, Rust decoder
  **native-rustdoc-arrow/4**, provider **delta-immutable-cbor/1**, comparison value **4**, bundle
  **delta-evidence-bundle/2**, contract manifest/intrinsic **2/7**. No historical reader.

## Current implemented boundary

Preserve the native catalog/providers, shared caches, physical ownership/cleanup, typed immutable
definitions/row selection and generated MCP catalog. This bounded slice unifies Delta references:

- CohortId is FixedSizeBinary(16); SchemaContractId is FixedSizeBinary(32) derived directly from
  the existing native schema-manifest key. Full-field native parameters, predicates, row keys,
  cache keys and schema registry retain typed identities. Text appears at declared boundaries.
- Shared DeltaTableRef/DeltaVersionRef/TableSelection/CdfWindow replace duplicate publication,
  definition, result, discovery, retention and replay scope. External Delta table IDs remain
  opaque. Returned-state capture supplies committed references; candidates have no captured
  authority. Legacy flat binding types and the definition Binding alias are deleted.
- One exact-read admission requires current protection, namespace, semantic contract and fresh
  metadata history before cache/replay reuse. Evidence, definitions and results consume it.
  Provider ingredients retain typed exact scope; the lower-level full-snapshot cache stays separate.
- Native CDF vector/image plans retain typed cohorts and checked inclusive ranges. Retention,
  cleanup and history use shared nested references and full-field coalescing. Root removal
  uses the same scope projection. Export still rebinds physical references independently of
  semantic snapshot identity.
- The native schema registry keeps typed identity and bounded IPC bytes with a bootstrap-safe
  physical lookup. Byte-bound intrinsic expressions now explicitly reconcile integer types;
  Binary and LargeBinary pre-admission are exercised without relying on SQL coercion.
- Generated schemas/DTOs/worker Arrow/guidance and fixtures advance together. The Delta reference
  structural guard and `bundle-2026-09-18-typed-delta-references` are added; prior provenance and
  accepted ADR arguments are preserved. Protected enforcement remains unapplied.

## Latest verification and known limits

Logs: `.dev-state/plan19/execution/typed-delta-*`. Plan 19 records scope and failures/repairs.

- **16 distinct scoped units pass across runs.** Initial run: 3 core passed / 12 store failed
  through the registry byte-bound validator. After the shared fix, all 13 store units pass
  (46.302 s), including a new Binary/LargeBinary regression. Native runtime diagnostics are
  incidental unit-fixture initialization, not storage/restart/client qualification.
- Core/store/daemon **all-target Clippy passes**. Two pinned Delta Parquet deprecations and
  proc-macro-error2 future warnings remain unsuppressed. All affected terminal tests compile.
- Schema generation/conformance passes: **4 fixtures / 11 negative cases**; packaged schemas,
  DTOs, worker Arrow and guidance are reproducible. Generator format-name warnings remain.
- An incorrectly scoped explicit generated-DTO Ruff run reports 183 style findings. ADR-0007
  already excludes these generated files; they were not hand-edited or newly suppressed.
  Maintained-source Ruff and ty pass using the existing scope.
- Delta reference rule: **1 group / 13 cases passed**; source/rule review scans are empty.
  Architecture, ADR/index/register, nine provenance bundles and scoped whitespace checks pass.
  Repository-wide whitespace reports preexisting Delta-skill TSV trailing fields; preserved.
- Independent audit confirms the report digest, all 48 IDs and stale-receipt exclusion with
  no discrepancies. Report/check has no product gate receipts for this tree. Full cache/storage/replay/restart,
  producer and real-client qualification remains deferred. No installation or activation ran.

## Next unmet work

1. Finish other content/digest/reference identities and clock/unit typing, plus the full
   operator/provider/mutation/feature matrix.
2. Complete exact effect/environment/fidelity consumers, revocation/exit/unknown acknowledgement,
   remaining research/recovery routes and materialization lifetime/property cases.
3. Finish orphan/history/transaction/CDF/replay fences, metadata/predicate/kernel/writer external
   allocation ownership and queue/shutdown matrices. Author terminal cases only until CP11.
4. Close L01–L33 and Q/SC/CF/DC source/package/harness removal; actual protected enforcement,
   matching packages and legacy install/state/client retirement. Only then run CP12 and activate.

## Pins and resource policy

Verified by 2026-09-18 doctor: Rust/rust-analyzer **1.98.1**, uv **0.12.17**, just **1.58.0**,
Python **3.14.7**, ty **0.0.80**, Griffe **2.3.0**. Container tools are present, not freshly qualified.
Source/lock: DataFusion **55.1.0**, Arrow/Parquet **59.3.0**, object_store **0.13.2**, Delta
**58f07cd62bfbce3649a7e1c87c696288068ae184**, kernel **8ba063f8f84fec222000f66d40d70911d7c79675**,
FastMCP **4.0.3**, MCP **2.2.0**, PyArrow **25.0.1**, Ciborium **0.2.2**, rustdoc-types **0.61.0**.
Use the pinned DataFusion/Delta/FastMCP skills and targeted probes for these libraries.

Performance defaults remain: pool **32 GiB**, spill **64 GiB**, metadata **2 GiB**, snapshots
**4 GiB** / entry **512 MiB**, providers **1 GiB**, contracts **64 MiB**, predicate cache **100 MiB**
per row group, checkpoints control **10** / others **100**, and **16** partitions/workers/parsers/
blocking threads per lane. Do not restore tiny development pools.
