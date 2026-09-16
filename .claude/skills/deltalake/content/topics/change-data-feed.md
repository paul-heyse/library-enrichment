# Change data feed

With change data feed enabled, DML writes `_change_data` files recording per-row before and after images. Reading them gives row-level change history rather than a snapshot diff. The feature must be on *before* the changes happen -- it is not retroactive, and that is the mistake worth avoiding.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::operations::load_cdf::CdfLoadBuilder` | struct | 9 | [prose](../api/deltalake_core.operations.load_cdf.md#cdfloadbuilder) | [records](../model/deltalake_core.operations.load_cdf.json) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Enable the feature before you need the history, not when you discover you needed it.
- CDF output includes change type and commit version columns; treat them as part of the schema.

## Anti-patterns

- Expecting change data for versions written before the feature was enabled.
- Treating `update_preimage` rows as current state. Each update emits a preimage and a postimage; consuming both as if they were rows doubles the change count and applies the old values last.
- Checkpointing a consumer by commit timestamp rather than version. Timestamps are not monotonic across writers; versions are.
- Using CDF as an audit log without a retention policy: the files vacuum away with everything else.

## Agent checklist

- Was the feature enabled before the window you are reading?
- Does retention cover the window you intend to replay?
