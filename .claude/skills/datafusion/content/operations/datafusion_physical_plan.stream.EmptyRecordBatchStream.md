# `datafusion_physical_plan::stream::EmptyRecordBatchStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.stream.EmptyRecordBatchStream.json).

<a id="op-4c3e56f6243479f72408d0c6"></a>
## EmptyRecordBatchStream

`struct` · `datafusion_physical_plan::stream::EmptyRecordBatchStream` · datafusion-physical-plan 55.1.0

```rust
struct EmptyRecordBatchStream
```

Source: `src/stream.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881)
that will produce no results

<a id="op-f1c6a7eccda97acd262534e4"></a>
## Item

`assoc_type` · `datafusion_physical_plan::stream::EmptyRecordBatchStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::EmptyRecordBatchStream", "path": "EmptyRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [524, 1], "end": [533, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:525`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6db4f7a5cafbfcca2f7d4b0"></a>
## new

`function` · `datafusion_physical_plan::stream::EmptyRecordBatchStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::EmptyRecordBatchStream", "path": "EmptyRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [516, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create an empty RecordBatchStream

<a id="op-38ab294499e097d1c6350c32"></a>
## poll_next

`function` · `datafusion_physical_plan::stream::EmptyRecordBatchStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::EmptyRecordBatchStream", "path": "EmptyRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [524, 1], "end": [533, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:527`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9946fdbcd5449aef3c57674"></a>
## schema

`function` · `datafusion_physical_plan::stream::EmptyRecordBatchStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::stream::EmptyRecordBatchStream", "path": "EmptyRecordBatchStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [522, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/stream.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
