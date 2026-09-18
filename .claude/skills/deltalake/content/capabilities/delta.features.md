# Separate feature vocabulary, protocol admission and operation support

A recognized feature enum is not an operation-support guarantee. Join table protocol requirements, the consumer's resolved Cargo features, checker admission and the selected operation's restrictions.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| ProtocolChecker and operation preconditions | You need to decide whether this operation is admitted | Checker acceptance still does not certify every semantic/performance path |
| Feature/operation matrix | You are discovering potentially usable functionality | Unknown cells remain unknown; follow evidence and profile |
| Explicit table feature upgrade | An operation requires a new protocol capability | Can affect other readers/writers; inspect the exact builder and table requirements |

## Contract

**profile.** Documentation was captured with a broader feature envelope than the local runtime probe. The runtime profile excludes nanosecond-timestamps; its acceptance is tested separately from vocabulary.
Claim `delta.features.1`; source_observation; evidence: upstream, source.

**admission.** The protocol probe rejects recognized identityColumns and timestampNanos reader requirements in its profile, while accepting deletionVectors.
Claim `delta.features.2`; runtime_observation; evidence: runtime.

**operation.** Deletion-vector scans, one delete and optimize are tested on the retained fixture; column mapping scan succeeds while CDF rejects it. These are separate cells, not universal support.
Claim `delta.features.3`; runtime_observation; evidence: runtime.

## Implementation

- Read content/profile.json and index/coverage.tsv before interpreting a missing symbol or a cfg-gated source insertion.
- Use catalogs/protocol-matrix.json for discovery and then inspect the operation's preconditions and fixtures.

## Effects

- admit/reject access
- possible protocol upgrade via mutation

## Errors

- Recognized-but-unsupported, excluded Cargo feature, unsupported table protocol and operation restriction must not collapse into one boolean.

## Limits and unknowns

- Variant, identity, generated-column and every cloud/profile combination are not runtime-qualified by this matrix.

## Exact contracts

- [`deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_read_from_protocol`](../operations/deltalake_core.kernel.transaction.protocol.ProtocolChecker.md#op-9a65d13a01a3236a76050780) — `fn can_read_from_protocol(&self, protocol: &Protocol) -> Result<(), TransactionError>`
- [`deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_write_to`](../operations/deltalake_core.kernel.transaction.protocol.ProtocolChecker.md#op-9f087de5991d3e750d8260bd) — `fn can_write_to(&self, snapshot: &dyn TableReference) -> Result<(), TransactionError>`
- [`deltalake_core::kernel::transaction::protocol::ProtocolChecker::can_commit`](../operations/deltalake_core.kernel.transaction.protocol.ProtocolChecker.md#op-44ead6f0afa26e598fc49940) — `fn can_commit(&self, snapshot: &dyn TableReference, actions: &[Action], operation: &DeltaOperation) -> Result<(), TransactionError>`
- [`deltalake_core::table::DeltaTable::add_feature`](../operations/deltalake_core.table.DeltaTable.md#op-a1986b8a1a4acb7259034938) — `fn add_feature(self) -> AddTableFeatureBuilder`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/transaction/protocol.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: recognized_feature_and_excluded_profile_are_not_reader_support, column_mapping_reads_logical_names_but_cdf_rejects_mapping, deletion_vectors_filter_rows_and_optimize_keeps_logical_rows
