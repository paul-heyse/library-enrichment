# `datafusion_physical_plan::common`

Crate `datafusion-physical-plan` · 7 public items · structured records in [`model/datafusion_physical_plan.common.json`](../model/datafusion_physical_plan.common.json)

## build_checked_file_list

`function` · `datafusion_physical_plan::common::build_checked_file_list`

```rust
fn build_checked_file_list(dir: &str, ext: &str) -> datafusion_common::Result<Vec<String>>
```

Recursively builds a list of files in a directory with a given extension

---

## build_file_list

`function` · `datafusion_physical_plan::common::build_file_list`

```rust
fn build_file_list(dir: &str, ext: &str) -> datafusion_common::Result<Vec<String>>
```

Recursively builds a list of files in a directory with a given extension

---

## can_project

`function` · `datafusion_physical_plan::common::can_project`

```rust
fn can_project(schema: &arrow::datatypes::SchemaRef, projection: Option<&[usize]>) -> datafusion_common::Result<()>
```

Checks if the given projection is valid for the given schema.

---

## collect

`function` · `datafusion_physical_plan::common::collect`

```rust
async fn collect(stream: super::SendableRecordBatchStream) -> datafusion_common::Result<Vec<arrow::record_batch::RecordBatch>>
```

Create a vector of record batches from a stream

---

## compute_record_batch_statistics

`function` · `datafusion_physical_plan::common::compute_record_batch_statistics`

```rust
fn compute_record_batch_statistics(batches: &[Vec<arrow::record_batch::RecordBatch>], schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> Statistics
```

Computes the statistics for an in-memory RecordBatch

Only computes statistics that are in arrows metadata (num rows, byte size and nulls)
and does not apply any kernel on the actual data.

---

## project_plan_to_schema

`function` · `datafusion_physical_plan::common::project_plan_to_schema`

```rust
fn project_plan_to_schema(input: std::sync::Arc<dyn ExecutionPlan>, expected_schema: &arrow::datatypes::SchemaRef) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>>
```

Align `input`'s physical plan schema with `expected_schema`.

This helper is intended for operators that combine independently planned children but
expose a single declared output schema. It returns `input` unchanged when schemas already
match exactly. Otherwise, it validates that projection can safely produce the expected
schema, then wraps `input` in a [`ProjectionExec`] that keeps columns in their existing
positional order and aliases them to `expected_schema`'s field names.

[`ProjectionExec`] can rename fields. When the expected field is nullable and the input
field is not, this helper also widens nullability with a same-type [`CastExpr`]. It rejects
differences that projection cannot safely normalize exactly, such as data type, metadata,
schema metadata, and nullability narrowing.

---

## spawn_buffered

`function` · `datafusion_physical_plan::common::spawn_buffered`

```rust
fn spawn_buffered(input: super::SendableRecordBatchStream, buffer: usize) -> super::SendableRecordBatchStream
```

If running in a tokio context spawns the execution of `stream` to a separate task
allowing it to execute in parallel with an intermediate buffer of size `buffer`.
At most `buffer` record batches will be produced ahead of the consumer.

---
