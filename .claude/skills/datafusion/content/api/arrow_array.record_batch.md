# `arrow_array::record_batch`

Crate `arrow-array` · 5 public items · structured records in [`model/arrow_array.record_batch.json`](../model/arrow_array.record_batch.json)

## RecordBatch

`struct` · `arrow_array::record_batch::RecordBatch`

Also reachable as `arrow::array::RecordBatch`, `arrow::record_batch::RecordBatch`, `arrow_array::RecordBatch`

```rust
struct RecordBatch
```

**Implements**: `arrow_pyarrow::FromPyArrow`, `arrow_pyarrow::ToPyArrow`, `core::convert::From`, `core::ops::index::Index`, `datafusion_physical_expr_common::metrics::baseline::RecordOutput`, `datafusion_physical_plan::buffer::SizedMessage`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (22)

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
fn column(&self, index: usize) -> &ArrayRef
fn column_by_name(&self, name: &str) -> Option<&ArrayRef>
fn columns(&self) -> &[ArrayRef]
fn get_array_memory_size(&self) -> usize
fn into_parts(self) -> (SchemaRef, Vec<ArrayRef>, usize)
fn new_empty(schema: SchemaRef) -> Self
unsafe fn new_unchecked(schema: SchemaRef, columns: Vec<Arc<dyn Array>>, row_count: usize) -> Self
fn normalize(&self, separator: &str, max_level: Option<usize>) -> Result<Self, ArrowError>
fn num_columns(&self) -> usize
fn num_rows(&self) -> usize
fn project(&self, indices: &[usize]) -> Result<RecordBatch, ArrowError>
fn remove_column(&mut self, index: usize) -> ArrayRef
fn schema(&self) -> SchemaRef
fn schema_metadata_mut(&mut self) -> &mut std::collections::HashMap<String, String>
fn schema_ref(&self) -> &SchemaRef
fn slice(&self, offset: usize, length: usize) -> RecordBatch
fn try_from_iter<I, F>(value: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (F, ArrayRef)>, F: AsRef<str>
fn try_from_iter_with_nullable<I, F>(value: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (F, ArrayRef, bool)>, F: AsRef<str>
fn try_new(schema: SchemaRef, columns: Vec<ArrayRef>) -> Result<Self, ArrowError>
fn try_new_with_options(schema: SchemaRef, columns: Vec<ArrayRef>, options: &RecordBatchOptions) -> Result<Self, ArrowError>
fn with_schema(self, schema: SchemaRef) -> Result<Self, ArrowError>
```

**via `core::convert::From`**

```rust
fn from(struct_array: &StructArray) -> Self
fn from(value: StructArray) -> Self
```

**via `core::ops::index::Index`**

```rust
fn index(&self, name: &str) -> &Self::Output
```

[Full member, field, variant and typed contracts](../operations/arrow_array.record_batch.RecordBatch.md).


A two-dimensional batch of column-oriented data with a defined
[schema](arrow_schema::Schema).

A `RecordBatch` is a two-dimensional dataset of a number of
contiguous arrays, each the same length.
A record batch has a schema which must match its arrays’
datatypes.

Record batches are a convenient unit of work for various
serialization and computation functions, possibly incremental.

Use the [`record_batch!`] macro to create a [`RecordBatch`] from
literal slice of values, useful for rapid prototyping and testing.

Example:
```rust
use arrow_array::record_batch;
let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)]),
    ("c", Utf8, ["alpha", "beta", "gamma"])
);
```

---

## RecordBatchIterator

`struct` · `arrow_array::record_batch::RecordBatchIterator`

Also reachable as `arrow::array::RecordBatchIterator`, `arrow::record_batch::RecordBatchIterator`, `arrow_array::RecordBatchIterator`

```rust
struct RecordBatchIterator<I> where I: IntoIterator<Item = Result<RecordBatch, arrow_schema::ArrowError>>
```

**Implements**: `arrow_array::record_batch::RecordBatchReader`, `core::iter::traits::iterator::Iterator`

**Methods** (1)

```rust
fn new(iter: I, schema: SchemaRef) -> Self
```

**via `arrow_array::record_batch::RecordBatchReader`**

```rust
fn schema(&self) -> SchemaRef
```

**via `core::iter::traits::iterator::Iterator`**

```rust
fn next(&mut self) -> Option<Self::Item>
fn size_hint(&self) -> (usize, Option<usize>)
```

[Full member, field, variant and typed contracts](../operations/arrow_array.record_batch.RecordBatchIterator.md).


Generic implementation of [RecordBatchReader] that wraps an iterator.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray, RecordBatchIterator, RecordBatchReader};
#
let a: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
let b: ArrayRef = Arc::new(StringArray::from(vec!["a", "b"]));

let record_batch = RecordBatch::try_from_iter(vec![
  ("a", a),
  ("b", b),
]).unwrap();

let batches: Vec<RecordBatch> = vec![record_batch.clone(), record_batch.clone()];

let mut reader = RecordBatchIterator::new(batches.into_iter().map(Ok), record_batch.schema());

assert_eq!(reader.schema(), record_batch.schema());
assert_eq!(reader.next().unwrap().unwrap(), record_batch);
# assert_eq!(reader.next().unwrap().unwrap(), record_batch);
# assert!(reader.next().is_none());
```

---

## RecordBatchOptions

`struct` · `arrow_array::record_batch::RecordBatchOptions`

Also reachable as `arrow::array::RecordBatchOptions`, `arrow::record_batch::RecordBatchOptions`, `arrow_array::RecordBatchOptions`

```rust
struct RecordBatchOptions
```

**Fields**: `match_field_names`, `row_count`

**Derives**: Debug, Default

**Methods** (3)

```rust
fn new() -> Self
fn with_match_field_names(self, match_field_names: bool) -> Self
fn with_row_count(self, row_count: Option<usize>) -> Self
```

[Full member, field, variant and typed contracts](../operations/arrow_array.record_batch.RecordBatchOptions.md).


Options that control the behaviour used when creating a [`RecordBatch`].

---

## RecordBatchReader

`trait` · `arrow_array::record_batch::RecordBatchReader`

Also reachable as `arrow::array::RecordBatchReader`, `arrow::record_batch::RecordBatchReader`, `arrow_array::RecordBatchReader`

```rust
trait RecordBatchReader: Iterator<Item = Result<RecordBatch, arrow_schema::ArrowError>>
```

**Implementors** (9)

- `alloc::boxed::Box`
- `arrow_array::ffi_stream::ArrowArrayStreamReader`
- `arrow_array::record_batch::RecordBatchIterator`
- `arrow_avro::reader::Reader`
- `arrow_csv::reader::BufReader`
- `arrow_ipc::reader::FileReader`
- `arrow_ipc::reader::StreamReader`
- `arrow_json::reader::Reader`
- `parquet::arrow::arrow_reader::ParquetRecordBatchReader`

**Methods** (1)

```rust
fn schema(&self) -> SchemaRef
```

[Full member, field, variant and typed contracts](../operations/arrow_array.record_batch.RecordBatchReader.md).


Trait for types that can read `RecordBatch`'s.

To create from an iterator, see [RecordBatchIterator].

---

## RecordBatchWriter

`trait` · `arrow_array::record_batch::RecordBatchWriter`

Also reachable as `arrow::array::RecordBatchWriter`, `arrow::record_batch::RecordBatchWriter`, `arrow_array::RecordBatchWriter`

```rust
trait RecordBatchWriter
```

**Implementors** (5)

- `arrow_csv::writer::Writer`
- `arrow_ipc::writer::FileWriter`
- `arrow_ipc::writer::StreamWriter`
- `arrow_json::writer::Writer`
- `parquet::arrow::arrow_writer::ArrowWriter`

**Methods** (2)

```rust
fn close(self) -> Result<(), ArrowError>
fn write(&mut self, batch: &RecordBatch) -> Result<(), ArrowError>
```

[Full member, field, variant and typed contracts](../operations/arrow_array.record_batch.RecordBatchWriter.md).


Trait for types that can write `RecordBatch`'s.

---
