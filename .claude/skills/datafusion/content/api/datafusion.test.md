# `datafusion::test`

Crate `datafusion` · 8 public items · structured records in [`model/datafusion.test.json`](../model/datafusion.test.json)

## assert_fields_eq

`function` · `datafusion::test::assert_fields_eq`

```rust
fn assert_fields_eq(plan: &logical_expr::LogicalPlan, expected: &[&str])
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.assert_fields_eq.md).


---

## columns

`function` · `datafusion::test::columns`

```rust
fn columns(schema: &arrow::datatypes::Schema) -> Vec<String>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.columns.md).


Returns the column names on the schema

---

## create_table_dual

`function` · `datafusion::test::create_table_dual`

```rust
fn create_table_dual() -> std::sync::Arc<dyn TableProvider>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.create_table_dual.md).


---

## make_partition

`function` · `datafusion::test::make_partition`

```rust
fn make_partition(sz: i32) -> arrow::record_batch::RecordBatch
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.make_partition.md).


Return a RecordBatch with a single Int32 array with values (0..sz)

---

## partitioned_file_groups

`function` · `datafusion::test::partitioned_file_groups`

```rust
fn partitioned_file_groups(path: &str, filename: &str, partitions: usize, file_format: &std::sync::Arc<dyn FileFormat>, file_compression_type: datasource::file_format::file_compression_type::FileCompressionType, work_dir: &std::path::Path) -> error::Result<Vec<datafusion_datasource::file_groups::FileGroup>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.partitioned_file_groups.md).


Returns file groups [`Vec<FileGroup>`] for scanning `partitions` of `filename`

---

## scan_partitioned_csv

`function` · `datafusion::test::scan_partitioned_csv`

```rust
fn scan_partitioned_csv(partitions: usize, work_dir: &std::path::Path) -> error::Result<std::sync::Arc<datafusion_datasource::source::DataSourceExec>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.scan_partitioned_csv.md).


Returns a [`DataSourceExec`] that scans "aggregate_test_100.csv" with `partitions` partitions

---

## table_with_decimal

`function` · `datafusion::test::table_with_decimal`

```rust
fn table_with_decimal() -> std::sync::Arc<dyn TableProvider>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.table_with_decimal.md).


Return a new table which provide this decimal column

---

## table_with_sequence

`function` · `datafusion::test::table_with_sequence`

```rust
fn table_with_sequence(seq_start: i32, seq_end: i32) -> error::Result<std::sync::Arc<dyn TableProvider>>
```

[Full member, field, variant and typed contracts](../operations/datafusion.test.table_with_sequence.md).


Return a new table provider that has a single Int32 column with
values between `seq_start` and `seq_end`

---
