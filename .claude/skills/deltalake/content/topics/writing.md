# Writing data

A write is a transaction: stage Parquet files, then commit `Add` actions. The builder decides save mode, partitioning, target file size and Parquet properties; nothing is visible until the commit lands. Getting file size right matters more than it looks -- many small files make every later read pay list and open costs.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `deltalake_core::operations::write::WriteBuilder` | struct | 20 | [prose](../api/deltalake_core.operations.write.md#writebuilder) | [records](../model/deltalake_core.operations.write.json) |
| `deltalake_core::writer::record_batch::RecordBatchWriter` | struct | 17 | [prose](../api/deltalake_core.writer.record_batch.md#recordbatchwriter) | [records](../model/deltalake_core.writer.record_batch.json) |
| `deltalake_core::writer::json::JsonWriter` | struct | 11 | [prose](../api/deltalake_core.writer.json.md#jsonwriter) | [records](../model/deltalake_core.writer.json.json) |
| `deltalake_core::protocol::SaveMode` | enum | 6 | [prose](../api/deltalake_core.protocol.md#savemode) | [records](../model/deltalake_core.protocol.json) |

## Extension points

| Trait | Required | Provided | Implementors | Page |
|---|---:|---:|---:|---|
| `deltalake_core::writer::DeltaWriter` | 3 | 1 | 2 | [DeltaWriter](../traits/DeltaWriter.md) |
| `deltalake_core::datafile::DataFileWriter` | 3 | 0 | 1 | [DataFileWriter](../traits/DataFileWriter.md) |
| `deltalake_core::datafile::DeltaDataWriter` | 1 | 0 | 1 | [DeltaDataWriter](../traits/DeltaDataWriter.md) |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`WriteBuilder`** — 18 builder methods

`with_cast_safety`, `with_commit_properties`, `with_configuration`, `with_custom_execute_handler`, `with_description`, `with_input_batches`, `with_input_execution_plan`, `with_input_plan`, `with_partition_columns`, `with_replace_where`, `with_save_mode`, `with_schema_mode`, `with_session_fallback_policy`, `with_session_state`, `with_table_name`, `with_target_file_size`, `with_write_batch_size`, `with_writer_properties`

## Runnable examples (4)

- [`corpus/examples/basic_operations.rs`](../corpus/examples/basic_operations.rs)
- [`corpus/examples/load_table.rs`](../corpus/examples/load_table.rs)
- [`corpus/examples/read_delta_table.rs`](../corpus/examples/read_delta_table.rs)
- [`corpus/examples/recordbatch-writer.rs`](../corpus/examples/recordbatch-writer.rs)

## Decision rules

- Set a schema mode if incoming data may carry new columns; the default neither evolves nor merges.
- Application transaction markers persist coordination metadata; repeating a marker does not suppress sequential appends at this pin. See the reviewed replay contract.
- Partition on a low-cardinality column that queries actually filter on -- otherwise partitioning only multiplies small files.

## Anti-patterns

- Overwriting the whole table to replace one partition, when a predicate overwrite expresses it directly.
- Leaving target file size at its default for a high-frequency writer and then wondering why reads are slow.
- Partitioning on a high-cardinality column -- a run id, a uuid, a float -- which turns one small file per value into the table's dominant cost.
- Writing Parquet files into the table directory directly. Nothing the log does not list exists.

## Agent checklist

- Is a schema mode set where the schema can change?
- Are commit properties carrying an app transaction id?
- Is the partition column one queries filter on?
