# Opening tables

A table is a URL plus a log store, and opening one is separate from reading it. `DeltaTableBuilder` resolves the URL to a backend and produces an unloaded `DeltaTable`; `load()` replays the log into a snapshot. Pin a version or timestamp at build time if you want reproducibility -- the default is whatever the log says now, which changes under you. `BlindDeltaTable` skips statistics entirely when you only intend to append.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::table::builder::DeltaTableBuilder` | struct | 16 | [prose](../api/deltalake_core.table.builder.md#deltatablebuilder) | [records](../model/deltalake_core.table.builder.json) |
| `deltalake_core::table::DeltaTable` | struct | 49 | [prose](../api/deltalake_core.table.md#deltatable) | [records](../model/deltalake_core.table.json) |
| `deltalake_core::table::blind::BlindDeltaTable` | struct | 17 | [prose](../api/deltalake_core.table.blind.md#blinddeltatable) | [records](../model/deltalake_core.table.blind.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::logstore::factories::LogStoreFactory` | 1 | 0 | 9 | [LogStoreFactory](../traits/LogStoreFactory.md) |
| `deltalake_core::logstore::factories::ObjectStoreFactory` | 1 | 0 | 9 | [ObjectStoreFactory](../traits/ObjectStoreFactory.md) |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`DeltaTableBuilder`** — 9 builder methods

`with_allow_http`, `with_datestring`, `with_io_runtime`, `with_log_buffer_size`, `with_skip_stats`, `with_storage_backend`, `with_storage_options`, `with_timestamp`, `with_version`

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Reproducible read: pin with `with_version` or `with_datestring` at build time. Without one you get whatever the log says at load, which is not the same table twice.
- Append-only writer: `BlindDeltaTable` avoids materialising file statistics you are not going to use.
- Credentials belong in `with_storage_options`, not in the URL.

## Anti-patterns

- Calling `update()` to refresh a table. It is not a refresh: `update()` consumes the table and returns an `UpdateBuilder`, the DML operation. Refreshing is `load()`, `update_state()` or `update_incremental()`. The names are one character apart and the mistake compiles.
- Rebuilding a `DeltaTable` per query. Log replay is the expensive part; reuse the loaded table and refresh it with `update_incremental` when you need freshness.
- Treating an unloaded table as empty. Before `load()` there is no snapshot, not an empty one.
- Reading `version()` without knowing whether the state was refreshed since. It reports the loaded version, not the table's current one.

## Agent checklist

- Is the version or timestamp pinned where reproducibility matters?
- Do storage options carry credentials rather than the URL?
- Is a refresh `load`/`update_incremental`, and not `update`?
