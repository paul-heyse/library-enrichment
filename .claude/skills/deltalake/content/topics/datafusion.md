# The DataFusion seam

Delta tables reach DataFusion as a `TableProvider` built from a snapshot, so the provider is pinned to the version it was built from and does not refresh itself. Pushdown is where the performance is: partition and statistics pruning happen during scan planning, and a scan configured without them reads everything and filters afterwards.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::delta_datafusion::table_provider::TableProviderBuilder` | struct | 13 | [prose](../api/deltalake_core.delta_datafusion.table_provider.md#tableproviderbuilder) | [records](../model/deltalake_core.delta_datafusion.table_provider.json) |
| `deltalake_core::delta_datafusion::table_provider::DeltaScanConfig` | struct | 11 | [prose](../api/deltalake_core.delta_datafusion.table_provider.md#deltascanconfig) | [records](../model/deltalake_core.delta_datafusion.table_provider.json) |
| `deltalake_core::delta_datafusion::table_provider::next::DeltaScan` | struct | 17 | [prose](../api/deltalake_core.delta_datafusion.table_provider.next.md#deltascan) | [records](../model/deltalake_core.delta_datafusion.table_provider.next.json) |
| `deltalake_core::delta_datafusion::table_provider::next::scan::exec::DeltaScanExec` | struct | 21 | [prose](../api/deltalake_core.delta_datafusion.table_provider.next.scan.exec.md#deltascanexec) | [records](../model/deltalake_core.delta_datafusion.table_provider.next.scan.exec.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::delta_datafusion::DataFusionMixins` | 3 | 0 | 3 | [DataFusionMixins](../traits/DataFusionMixins.md) |

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Reuse one `SessionContext` across queries; building one per query discards every cache.
- Rebuild the provider when you need a newer version -- it is bound to the snapshot it was made from.
- Check `EXPLAIN` for the file count actually scanned before concluding pruning works.

## Anti-patterns

- Registering a provider once and expecting it to see later commits.
- Overriding the session's object store registry and losing the table's own backend wiring.

## Agent checklist

- Is the session reused?
- Does EXPLAIN show pruning reaching the file level?
- Is the provider rebuilt when freshness matters?
