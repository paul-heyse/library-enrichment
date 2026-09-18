# `arrow_array::record_batch::RecordBatch`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.RecordBatch.json).

<a id="op-87f977a95cb312da9259aa34"></a>
## RecordBatch

`struct` · `arrow_array::record_batch::RecordBatch` · arrow-array 59.3.0

```rust
struct RecordBatch
```

Source: `src/record_batch.rs:224`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

A two-dimensional batch of column-oriented data with a defined
[schema](arrow_schema::Schema).

A `RecordBatch` is a two-dimensional dataset of a number of
contiguous arrays, each the same length.
A record batch has a schema which must match its arrays’
datatypes.

Record batches are a convenient unit of work for various
serialization and computation functions, possibly incremental.

Use the [`record_batch!`](../operations/arrow_array.record_batch.md#op-4641f570cd1f55102adb1f0f) macro to create a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) from
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

<a id="op-396cdba7c94681daee1b39a0"></a>
## Output

`assoc_type` · `arrow_array::record_batch::RecordBatch::Output` · arrow-array 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [888, 2], "filename": "src/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/record_batch.rs:878`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d52fe35d6c20d34ca603620f"></a>
## claim

`function` · `arrow_array::record_batch::RecordBatch::claim` · arrow-array 59.3.0

```rust
fn claim(&self, pool: &dyn arrow_buffer::MemoryPool)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:798`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Registers all buffers in this record batch with the provided [`MemoryPool`].

This claims memory for all columns in the batch by calling [`Array::claim`]
on each column.

[`MemoryPool`]: arrow_buffer::MemoryPool
[`Array::claim`]: crate::Array::claim

<a id="op-f2ec2f88b55f549a86650555"></a>
## clone

`function` · `arrow_array::record_batch::RecordBatch::clone` · arrow-array 59.3.0

```rust
fn clone(&self) -> RecordBatch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 10], "end": [223, 15], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/record_batch.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d574501f97cf1c06edf2303d"></a>
## column

`function` · `arrow_array::record_batch::RecordBatch::column` · arrow-array 59.3.0

```rust
fn column(&self, index: usize) -> &ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:625`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Get a reference to a column's array by index.

# Panics

Panics if `index` is outside of `0..num_columns`.

<a id="op-8d7bc1ff8844ae844180faa5"></a>
## column_by_name

`function` · `arrow_array::record_batch::RecordBatch::column_by_name` · arrow-array 59.3.0

```rust
fn column_by_name(&self, name: &str) -> Option<&ArrayRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:630`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Get a reference to a column's array by name.

<a id="op-b1401ecdfbe4696675e4256f"></a>
## columns

`function` · `arrow_array::record_batch::RecordBatch::columns` · arrow-array 59.3.0

```rust
fn columns(&self) -> &[ArrayRef]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:637`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Get a reference to all columns in the record batch.

<a id="op-c6a093850c0eea8e4fe08179"></a>
## eq

`function` · `arrow_array::record_batch::RecordBatch::eq` · arrow-array 59.3.0

```rust
fn eq(&self, other: &RecordBatch) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 24], "end": [223, 33], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/record_batch.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-617537c143fdaf9bae9ff59d"></a>
## fmt

`function` · `arrow_array::record_batch::RecordBatch::fmt` · arrow-array 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 17], "end": [223, 22], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/record_batch.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4df087fe477c1782d4ca7bd"></a>
## from

`function` · `arrow_array::record_batch::RecordBatch::from` · arrow-array 59.3.0

```rust
fn from(value: StructArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [853, 1], "end": [869, 2], "filename": "src/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/record_batch.rs:854`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e444aa69c89c74b6c159517a"></a>
## from

`function` · `arrow_array::record_batch::RecordBatch::from` · arrow-array 59.3.0

```rust
fn from(struct_array: &StructArray) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [871, 1], "end": [875, 2], "filename": "src/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_array::array::struct_array::StructArray", "path": "StructArray"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/record_batch.rs:872`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d6270a8f2b64d1641f135ff"></a>
## get_array_memory_size

`function` · `arrow_array::record_batch::RecordBatch::get_array_memory_size` · arrow-array 59.3.0

```rust
fn get_array_memory_size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:810`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the total number of bytes of memory occupied physically by this batch.

Note that this does not always correspond to the exact memory usage of a
`RecordBatch` (might overestimate), since multiple columns can share the same
buffers or slices thereof, the memory used by the shared buffers might be
counted multiple times.

<a id="op-9bb42393b17c1d9929074c9a"></a>
## index

`function` · `arrow_array::record_batch::RecordBatch::index` · arrow-array 59.3.0

```rust
fn index(&self, name: &str) -> &Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [877, 1], "end": [888, 2], "filename": "src/record_batch.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::ops::index::Index", "path": "Index"}, "trait_path": "core::ops::index::Index"}`

Source: `src/record_batch.rs:885`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Get a reference to a column's array by name.

# Panics

Panics if the name is not in the schema.

<a id="op-0bdf167be8669d4b8be233da"></a>
## into_parts

`function` · `arrow_array::record_batch::RecordBatch::into_parts` · arrow-array 59.3.0

```rust
fn into_parts(self) -> (SchemaRef, Vec<ArrayRef>, usize)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:397`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return the schema, columns and row count of this [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

<a id="op-9429552e23344e88c9bbf8f2"></a>
## new_empty

`function` · `arrow_array::record_batch::RecordBatch::new_empty` · arrow-array 59.3.0

```rust
fn new_empty(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:308`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a new empty [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34).

<a id="op-f3015f5b29a3c03b89ab19a7"></a>
## new_unchecked

`function` · `arrow_array::record_batch::RecordBatch::new_unchecked` · arrow-array 59.3.0

```rust
unsafe fn new_unchecked(schema: SchemaRef, columns: Vec<Arc<dyn Array>>, row_count: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:283`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a `RecordBatch` from a schema and columns, without validation.

See [`Self::try_new`](../operations/arrow_array.record_batch.RecordBatch.md#op-dd87dabf7102df1ff3d8ea95) for the checked version.

# Safety

Expects the following:

 * `schema.fields.len() == columns.len()`
 * `schema.fields[i].data_type() == columns[i].data_type()`
 * `columns[i].len() == row_count`

Note: if the schema does not match the underlying data exactly, it can lead to undefined
behavior, for example, via conversion to a `StructArray`, which in turn could lead
to incorrect access.

<a id="op-7dc702f75a33da758bc5440f"></a>
## normalize

`function` · `arrow_array::record_batch::RecordBatch::normalize` · arrow-array 59.3.0

```rust
fn normalize(&self, separator: &str, max_level: Option<usize>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:541`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Normalize a semi-structured [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) into a flat table.

Nested [`Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf)s will generate names separated by `separator`, up to a depth of `max_level`
(unlimited if `None`).

e.g. given a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) with schema:

```text
    "foo": StructArray<"bar": Utf8>
```

A separator of `"."` would generate a batch with the schema:

```text
    "foo.bar": Utf8
```

Note that giving a depth of `Some(0)` to `max_level` is the same as passing in `None`;
it will be treated as unlimited.

# Example

```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int64Array, StringArray, StructArray, RecordBatch};
# use arrow_schema::{DataType, Field, Fields, Schema};
#
let animals: ArrayRef = Arc::new(StringArray::from(vec!["Parrot", ""]));
let n_legs: ArrayRef = Arc::new(Int64Array::from(vec![Some(2), Some(4)]));

let animals_field = Arc::new(Field::new("animals", DataType::Utf8, true));
let n_legs_field = Arc::new(Field::new("n_legs", DataType::Int64, true));

let a = Arc::new(StructArray::from(vec![
    (animals_field.clone(), Arc::new(animals.clone()) as ArrayRef),
    (n_legs_field.clone(), Arc::new(n_legs.clone()) as ArrayRef),
]));

let schema = Schema::new(vec![
    Field::new(
        "a",
        DataType::Struct(Fields::from(vec![animals_field, n_legs_field])),
        false,
    )
]);

let normalized = RecordBatch::try_new(Arc::new(schema), vec![a])
    .expect("valid conversion")
    .normalize(".", None)
    .expect("valid normalization");

let expected = RecordBatch::try_from_iter_with_nullable(vec![
    ("a.animals", animals.clone(), true),
    ("a.n_legs", n_legs.clone(), true),
])
.expect("valid conversion");

assert_eq!(expected, normalized);
```

<a id="op-53bbee46069823f88fa9f57e"></a>
## num_columns

`function` · `arrow_array::record_batch::RecordBatch::num_columns` · arrow-array 59.3.0

```rust
fn num_columns(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:594`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of columns in the record batch.

# Example

```
# use std::sync::Arc;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};

let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false)
]);

let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(id_array)]).unwrap();

assert_eq!(batch.num_columns(), 1);
```

<a id="op-66b7a01768582ecc2edff11a"></a>
## num_rows

`function` · `arrow_array::record_batch::RecordBatch::num_rows` · arrow-array 59.3.0

```rust
fn num_rows(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:616`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the number of rows in each column.

# Example

```
# use std::sync::Arc;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};

let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false)
]);

let batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(id_array)]).unwrap();

assert_eq!(batch.num_rows(), 5);
```

<a id="op-124dc8bac9a1c739ad939083"></a>
## project

`function` · `arrow_array::record_batch::RecordBatch::project` · arrow-array 59.3.0

```rust
fn project(&self, indices: &[usize]) -> Result<RecordBatch, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:455`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Projects the schema onto the specified columns

<a id="op-97dafcf1081767849d8e2f8d"></a>
## remove_column

`function` · `arrow_array::record_batch::RecordBatch::remove_column` · arrow-array 59.3.0

```rust
fn remove_column(&mut self, index: usize) -> ArrayRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:668`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Remove column by index and return it.

Return the `ArrayRef` if the column is removed.

# Panics

Panics if `index`` out of bounds.

# Example

```
use std::sync::Arc;
use arrow_array::{BooleanArray, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let bool_array = BooleanArray::from(vec![true, false, false, true, true]);
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("bool", DataType::Boolean, false),
]);

let mut batch = RecordBatch::try_new(Arc::new(schema), vec![Arc::new(id_array), Arc::new(bool_array)]).unwrap();

let removed_column = batch.remove_column(0);
assert_eq!(removed_column.as_any().downcast_ref::<Int32Array>().unwrap(), &Int32Array::from(vec![1, 2, 3, 4, 5]));
assert_eq!(batch.num_columns(), 1);
```

<a id="op-fd853fe5365c7c7c7efd8daa"></a>
## schema

`function` · `arrow_array::record_batch::RecordBatch::schema` · arrow-array 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:423`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns the [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) of the record batch.

<a id="op-8bcd7650e1a185f3893975e3"></a>
## schema_metadata_mut

`function` · `arrow_array::record_batch::RecordBatch::schema_metadata_mut` · arrow-array 59.3.0

```rust
fn schema_metadata_mut(&mut self) -> &mut std::collections::HashMap<String, String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:449`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Mutable access to the metadata of the schema.

This allows you to modify [`Schema::metadata`](../operations/arrow_schema.schema.Schema.md#op-174b4a403c267fd5b115e3e8) of [`Self::schema`](../operations/arrow_array.record_batch.RecordBatch.md#op-fd853fe5365c7c7c7efd8daa) in a convenient and fast way.

Note this will clone the entire underlying `Schema` object if it is currently shared

# Example
```
# use std::sync::Arc;
# use arrow_array::{record_batch, RecordBatch};
let mut batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// Initially, the metadata is empty
assert!(batch.schema().metadata().get("key").is_none());
// Insert a key-value pair into the metadata
batch.schema_metadata_mut().insert("key".into(), "value".into());
assert_eq!(batch.schema().metadata().get("key"), Some(&String::from("value")));
```

<a id="op-eec36f4603bdd60ca37b613b"></a>
## schema_ref

`function` · `arrow_array::record_batch::RecordBatch::schema_ref` · arrow-array 59.3.0

```rust
fn schema_ref(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:428`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Returns a reference to the [`Schema`](../operations/arrow_schema.schema.Schema.md#op-144709aa539d6163483c2050) of the record batch.

<a id="op-a79af188f3899a383689384f"></a>
## slice

`function` · `arrow_array::record_batch::RecordBatch::slice` · arrow-array 59.3.0

```rust
fn slice(&self, offset: usize, length: usize) -> RecordBatch
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:681`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Return a new RecordBatch where each column is sliced
according to `offset` and `length`

# Panics

Panics if `offset` with `length` is greater than column length.

<a id="op-7e0096a223015f46a6639e49"></a>
## try_from_iter

`function` · `arrow_array::record_batch::RecordBatch::try_from_iter` · arrow-array 59.3.0

```rust
fn try_from_iter<I, F>(value: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (F, ArrayRef)>, F: AsRef<str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:733`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a `RecordBatch` from an iterable list of pairs of the
form `(field_name, array)`, with the same requirements on
fields and arrays as [`RecordBatch::try_new`](../operations/arrow_array.record_batch.RecordBatch.md#op-dd87dabf7102df1ff3d8ea95). This method is
often used to create a single `RecordBatch` from arrays,
e.g. for testing.

The resulting schema is marked as nullable for each column if
the array for that column is has any nulls. To explicitly
specify nullibility, use [`RecordBatch::try_from_iter_with_nullable`](../operations/arrow_array.record_batch.RecordBatch.md#op-f643ec87c60307a59eb45956)

Example:
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};

let a: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
let b: ArrayRef = Arc::new(StringArray::from(vec!["a", "b"]));

let record_batch = RecordBatch::try_from_iter(vec![
  ("a", a),
  ("b", b),
]);
```
Another way to quickly create a [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34) is to use the [`record_batch!`](../operations/arrow_array.record_batch.md#op-4641f570cd1f55102adb1f0f) macro,
which is particularly helpful for rapid prototyping and testing.

Example:

```rust
use arrow_array::record_batch;
let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)]),
    ("c", Utf8, ["alpha", "beta", "gamma"])
);
```

<a id="op-f643ec87c60307a59eb45956"></a>
## try_from_iter_with_nullable

`function` · `arrow_array::record_batch::RecordBatch::try_from_iter_with_nullable` · arrow-array 59.3.0

```rust
fn try_from_iter_with_nullable<I, F>(value: I) -> Result<Self, ArrowError> where I: IntoIterator<Item = (F, ArrayRef, bool)>, F: AsRef<str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:770`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a `RecordBatch` from an iterable list of tuples of the
form `(field_name, array, nullable)`, with the same requirements on
fields and arrays as [`RecordBatch::try_new`](../operations/arrow_array.record_batch.RecordBatch.md#op-dd87dabf7102df1ff3d8ea95). This method is often
used to create a single `RecordBatch` from arrays, e.g. for
testing.

Example:
```
# use std::sync::Arc;
# use arrow_array::{ArrayRef, Int32Array, RecordBatch, StringArray};

let a: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
let b: ArrayRef = Arc::new(StringArray::from(vec![Some("a"), Some("b")]));

// Note neither `a` nor `b` has any actual nulls, but we mark
// b an nullable
let record_batch = RecordBatch::try_from_iter_with_nullable(vec![
  ("a", a, false),
  ("b", b, true),
]);
```

<a id="op-dd87dabf7102df1ff3d8ea95"></a>
## try_new

`function` · `arrow_array::record_batch::RecordBatch::try_new` · arrow-array 59.3.0

```rust
fn try_new(schema: SchemaRef, columns: Vec<ArrayRef>) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:263`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a `RecordBatch` from a schema and columns.

Expects the following:

 * `!columns.is_empty()`
 * `schema.fields.len() == columns.len()`
 * `schema.fields[i].data_type() == columns[i].data_type()`
 * `columns[i].len() == columns[j].len()`

If the conditions are not met, an error is returned.

# Example

```
# use std::sync::Arc;
# use arrow_array::{Int32Array, RecordBatch};
# use arrow_schema::{DataType, Field, Schema};

let id_array = Int32Array::from(vec![1, 2, 3, 4, 5]);
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false)
]);

let batch = RecordBatch::try_new(
    Arc::new(schema),
    vec![Arc::new(id_array)]
).unwrap();
```

<a id="op-b3154cbd0c6729748077fc9a"></a>
## try_new_with_options

`function` · `arrow_array::record_batch::RecordBatch::try_new_with_options` · arrow-array 59.3.0

```rust
fn try_new_with_options(schema: SchemaRef, columns: Vec<ArrayRef>, options: &RecordBatchOptions) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:299`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Creates a `RecordBatch` from a schema and columns, with additional options,
such as whether to strictly validate field names.

See [`RecordBatch::try_new`](../operations/arrow_array.record_batch.RecordBatch.md#op-dd87dabf7102df1ff3d8ea95) for the expected conditions.

<a id="op-7cc7244954adb614374b7d85"></a>
## with_schema

`function` · `arrow_array::record_batch::RecordBatch::with_schema` · arrow-array 59.3.0

```rust
fn with_schema(self, schema: SchemaRef) -> Result<Self, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [234, 1], "end": [816, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:407`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Override the schema of this [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34)

Returns an error if `schema` is not a superset of the current schema
as determined by [`Schema::contains`]

See also [`Self::schema_metadata_mut`](../operations/arrow_array.record_batch.RecordBatch.md#op-8bcd7650e1a185f3893975e3).

Unresolved upstream links (retained, not inferred): ``Schema::contains``.
