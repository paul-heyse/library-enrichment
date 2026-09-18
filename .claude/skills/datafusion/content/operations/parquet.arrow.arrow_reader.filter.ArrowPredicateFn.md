# `parquet::arrow::arrow_reader::filter::ArrowPredicateFn`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_reader.filter.ArrowPredicateFn.json).

<a id="op-6817a0a6bc2078986ae8a2d4"></a>
## ArrowPredicateFn

`struct` · `parquet::arrow::arrow_reader::filter::ArrowPredicateFn` · parquet 59.3.0

```rust
struct ArrowPredicateFn<F>
```

Source: `src/arrow/arrow_reader/filter.rs:106`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

An [`ArrowPredicate`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicate.md#op-bf093cc5f111fb8b2d44eb01) created from an [`FnMut`] and a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec)

See [`RowFilter`](../operations/parquet.arrow.arrow_reader.filter.RowFilter.md#op-a310d73e2b8ee5aca56c0c1f) for more information on applying filters during the
Parquet decoding process.

The function is passed `RecordBatch`es with only the columns specified in
the [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec).

The function must return a [`BooleanArray`](../operations/arrow_array.array.boolean_array.BooleanArray.md#op-da9041df4f2e5d0ab93f8505) that has the same length as the
input `batch` where each row indicates whether the row should be returned:
* `true`: the row should be returned
* `false` or `null`: the row should not be returned

# Example:

Given an input schema: `"a:int64", "b:int64"`, you can create a predicate that
evaluates `b > 0` like this:

```
# use std::sync::Arc;
# use arrow::compute::kernels::cmp::gt;
# use arrow_array::{BooleanArray, Int64Array, RecordBatch};
# use arrow_array::cast::AsArray;
# use arrow_array::types::Int64Type;
# use parquet::arrow::arrow_reader::ArrowPredicateFn;
# use parquet::arrow::ProjectionMask;
# use parquet::schema::types::{SchemaDescriptor, Type};
# use parquet::basic; // note there are two `Type`s that are different
# // Schema for a table with one columns: "a" (int64) and "b" (int64)
# let descriptor = SchemaDescriptor::new(
#  Arc::new(
#    Type::group_type_builder("my_schema")
#      .with_fields(vec![
#        Arc::new(
#         Type::primitive_type_builder("a", basic::Type::INT64)
#          .build().unwrap()
#        ),
#        Arc::new(
#         Type::primitive_type_builder("b", basic::Type::INT64)
#          .build().unwrap()
#        ),
#     ])
#     .build().unwrap()
#  )
# );
// Create a mask for selecting only the second column "b" (index 1)
let projection_mask = ProjectionMask::leaves(&descriptor, [1]);
// Closure that evaluates "b > 0"
let predicate = |batch: RecordBatch| {
   let scalar_0 = Int64Array::new_scalar(0);
   let column = batch.column(0).as_primitive::<Int64Type>();
   // call the gt kernel to compute `>` which returns a BooleanArray
   gt(column, &scalar_0)
 };
// Create ArrowPredicateFn that can be passed to RowFilter
let arrow_predicate = ArrowPredicateFn::new(projection_mask, predicate);
```

Unresolved upstream links (retained, not inferred): ``FnMut``.

<a id="op-f23b4b4df3d5c609827f9aad"></a>
## evaluate

`function` · `parquet::arrow::arrow_reader::filter::ArrowPredicateFn::evaluate` · parquet 59.3.0

```rust
fn evaluate(&mut self, batch: RecordBatch) -> Result<BooleanArray, ArrowError>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::filter::ArrowPredicateFn", "path": "ArrowPredicateFn"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow_array::RecordBatch"}}], "output": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "arrow_array::BooleanArray"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "id": "core::ops::function::FnMut", "path": "FnMut"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [122, 1], "end": [133, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": {"args": null, "id": "parquet::arrow::arrow_reader::filter::ArrowPredicate", "path": "ArrowPredicate"}, "trait_path": "parquet::arrow::arrow_reader::filter::ArrowPredicate"}`

Source: `src/arrow/arrow_reader/filter.rs:130`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-240a5d1e05483ba588116277"></a>
## new

`function` · `parquet::arrow::arrow_reader::filter::ArrowPredicateFn::new` · parquet 59.3.0

```rust
fn new(projection: ProjectionMask, f: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::filter::ArrowPredicateFn", "path": "ArrowPredicateFn"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow_array::RecordBatch"}}], "output": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "arrow_array::BooleanArray"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "id": "core::ops::function::FnMut", "path": "FnMut"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [111, 1], "end": [120, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/arrow_reader/filter.rs:117`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a new [`ArrowPredicateFn`](../operations/parquet.arrow.arrow_reader.filter.ArrowPredicateFn.md#op-6817a0a6bc2078986ae8a2d4) that invokes `f` on the columns
specified in `projection`.

<a id="op-d9a6579704ab31651db471da"></a>
## projection

`function` · `parquet::arrow::arrow_reader::filter::ArrowPredicateFn::projection` · parquet 59.3.0

```rust
fn projection(&self) -> &ProjectionMask
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "parquet::arrow::arrow_reader::filter::ArrowPredicateFn", "path": "ArrowPredicateFn"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"parenthesized": {"inputs": [{"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow_array::RecordBatch"}}], "output": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::array::boolean_array::BooleanArray", "path": "arrow_array::BooleanArray"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "id": "core::ops::function::FnMut", "path": "FnMut"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [122, 1], "end": [133, 2], "filename": "src/arrow/arrow_reader/filter.rs"}, "trait": {"args": null, "id": "parquet::arrow::arrow_reader::filter::ArrowPredicate", "path": "ArrowPredicate"}, "trait_path": "parquet::arrow::arrow_reader::filter::ArrowPredicate"}`

Source: `src/arrow/arrow_reader/filter.rs:126`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
