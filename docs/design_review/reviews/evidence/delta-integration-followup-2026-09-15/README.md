# Delta integration follow-up evidence

Verified 2026-09-15 for the [focused review](../../design_review_delta-integration-followup_2026-09-15.md).
This package tests selected CDF and provider-codec behavior. It is not product acceptance.

Pins: delta-rs `58f07cd62bfbce3649a7e1c87c696288068ae184`, selected Buoyant kernel
`8ba063f8f84fec222000f66d40d70911d7c79675`, DataFusion 55.1.0, Arrow/Parquet 59.3.0,
object_store 0.13.2. Both git revisions are retained in Cargo.lock.

## Reproduction

Run from the repository root with cached locked dependencies. Use a new absolute scratch
directory for each execution. Building this standalone manifest does not modify the service
manifest or lockfile.

```bash
CARGO_TARGET_DIR="$PWD/target" cargo build --locked --offline \
  --manifest-path docs/design_review/reviews/evidence/delta-integration-followup-2026-09-15/Cargo.toml -j 8
followup_state_dir=$(mktemp -d "$PWD/.dev-state/delta-integration-followup/reproduction.XXXXXX")
target/debug/delta-integration-followup "$followup_state_dir"
```

## Observations

- A CDF provider bound to versions [1,1] excludes changes from versions 0 and 2.
- Filtering on id and change type while projecting commit version with LIMIT returns the
  intended row; the commit-version field is UInt64.
- DeltaLogicalCodec retains a snapshot-provider revision after the underlying table advances.
- The codec ignores a caller-supplied empty expected schema.
- The decoded provider rejects INSERT because its runtime log-store handle is absent.
- DeltaLogicalCodec rejects encoding a CDF provider.

`probe-run.log` is the completed run. `probe-initial-failure.log` records the probe's earlier
incorrect Int64 assumption for the CDF version field; the corrected source uses UInt64.
The receipt records exact dependencies, commands, hashes, limits and the scratch build warning.

No remote catalog/storage, full mutation/schema evolution, retention races, crash recovery,
installed MCP client or performance qualification was run. The review's other claims are
based on the pinned skill indexes and actual source, with their evidence strength stated.
