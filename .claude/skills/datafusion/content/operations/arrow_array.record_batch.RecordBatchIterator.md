# `arrow_array::record_batch::RecordBatchIterator`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.record_batch.RecordBatchIterator.json).

<a id="op-d3fc556920f24888e8206871"></a>
## RecordBatchIterator

`struct` · `arrow_array::record_batch::RecordBatchIterator` · arrow-array 59.3.0

```rust
struct RecordBatchIterator<I> where I: IntoIterator<Item = Result<RecordBatch, arrow_schema::ArrowError>>
```

Source: `src/record_batch.rs:915`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Generic implementation of [RecordBatchReader](../operations/arrow_array.record_batch.RecordBatchReader.md#op-e728e9cf0c3686077413de28) that wraps an iterator.

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

<a id="op-f97c3f167f7dcd2e56ca8eda"></a>
## Item

`assoc_type` · `arrow_array::record_batch::RecordBatchIterator::Item` · arrow-array 59.3.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "arrow_array::record_batch::RecordBatchIterator", "path": "RecordBatchIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [938, 1], "end": [951, 2], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record_batch.rs:942`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fe4b8faab0822f7f72c7e17e"></a>
## new

`function` · `arrow_array::record_batch::RecordBatchIterator::new` · arrow-array 59.3.0

```rust
fn new(iter: I, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "arrow_array::record_batch::RecordBatchIterator", "path": "RecordBatchIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [923, 1], "end": [936, 2], "filename": "src/record_batch.rs"}, "trait": null, "trait_path": null}`

Source: `src/record_batch.rs:930`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a new [RecordBatchIterator](../operations/arrow_array.record_batch.RecordBatchIterator.md#op-d3fc556920f24888e8206871).

If `iter` is an infallible iterator, use `.map(Ok)`.

<a id="op-55a1891138608d82773172da"></a>
## next

`function` · `arrow_array::record_batch::RecordBatchIterator::next` · arrow-array 59.3.0

```rust
fn next(&mut self) -> Option<Self::Item>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "arrow_array::record_batch::RecordBatchIterator", "path": "RecordBatchIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [938, 1], "end": [951, 2], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record_batch.rs:944`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f43fb1d6a07d7ebb5c56a8c9"></a>
## schema

`function` · `arrow_array::record_batch::RecordBatchIterator::schema` · arrow-array 59.3.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "arrow_array::record_batch::RecordBatchIterator", "path": "RecordBatchIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [953, 1], "end": [960, 2], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "arrow_array::record_batch::RecordBatchReader", "path": "RecordBatchReader"}, "trait_path": "arrow_array::record_batch::RecordBatchReader"}`

Source: `src/record_batch.rs:957`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8001c5d04085fbc4dc9dc27"></a>
## size_hint

`function` · `arrow_array::record_batch::RecordBatchIterator::size_hint` · arrow-array 59.3.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "I"}}], "constraints": []}}, "id": "arrow_array::record_batch::RecordBatchIterator", "path": "RecordBatchIterator"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "I"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "RecordBatch"}}}, {"type": {"resolved_path": {"args": null, "id": "arrow_schema::error::ArrowError", "path": "arrow_schema::ArrowError"}}}], "constraints": []}}, "id": "core::result::Result", "path": "Result"}}}}, "name": "Item"}]}}, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}}}], "generic_params": [], "type": {"generic": "I"}}}]}, "is_negative": false, "span": {"begin": [938, 1], "end": [951, 2], "filename": "src/record_batch.rs"}, "trait": {"args": null, "id": "core::iter::traits::iterator::Iterator", "path": "Iterator"}, "trait_path": "core::iter::traits::iterator::Iterator"}`

Source: `src/record_batch.rs:948`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.
