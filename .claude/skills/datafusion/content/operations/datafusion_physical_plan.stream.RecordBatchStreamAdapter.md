# `datafusion_physical_plan::stream::RecordBatchStreamAdapter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.stream.RecordBatchStreamAdapter.json).

<a id="op-4a13828d046a7d966d2434bf"></a>
## RecordBatchStreamAdapter

`struct` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter` · datafusion-physical-plan 55.1.0

```rust
struct RecordBatchStreamAdapter<S>
```

Source: `src/stream.rs:406`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Combines a [`Stream`] with a [`SchemaRef`](../operations/arrow_schema.schema.SchemaRef.md#op-e48a1bb89d62307cfab4093a) implementing
[`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6) for the combination

See [`Self::new`](../operations/datafusion_physical_plan.stream.RecordBatchStreamAdapter.md#op-4b47057bf5f5c28dc1d9917d) for an example

Unresolved upstream links (retained, not inferred): ``Stream``.

<a id="op-e9944e6027e529f5b101ed42"></a>
## Item

`assoc_type` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow::record_batch::RecordBatch"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "datafusion_common::Result"}}}}, "name": "Item"}]}}, "id": "futures_core::stream::Stream", "path": "Stream"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [462, 1], "end": [493, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2d0eed8658df77230e6ebd7"></a>
## fmt

`function` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [454, 1], "end": [460, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stream.rs:455`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b47057bf5f5c28dc1d9917d"></a>
## new

`function` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::new` · datafusion-physical-plan 55.1.0

```rust
fn new(schema: SchemaRef, stream: S) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [422, 1], "end": [452, 2], "filename": "src/stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/stream.rs:446`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a new [`RecordBatchStreamAdapter`](../operations/datafusion_physical_plan.stream.RecordBatchStreamAdapter.md#op-4a13828d046a7d966d2434bf) from the provided schema and stream.

Note to create a [`SendableRecordBatchStream`](../operations/datafusion_execution.stream.SendableRecordBatchStream.md#op-7cf25e4c554567392af11cb6) you pin the result

# Example
```
# use arrow::array::record_batch;
# use datafusion_execution::SendableRecordBatchStream;
# use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
// Create stream of Result<RecordBatch>
let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)])
)
.expect("created batch");
let schema = batch.schema();
let stream = futures::stream::iter(vec![Ok(batch)]);
// Convert the stream to a SendableRecordBatchStream
let adapter = RecordBatchStreamAdapter::new(schema, stream);
// Now you can use the adapter as a SendableRecordBatchStream
let batch_stream: SendableRecordBatchStream = Box::pin(adapter);
// ...
```

<a id="op-2801a3e9b5b548b7796fd817"></a>
## poll_next

`function` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow::record_batch::RecordBatch"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "datafusion_common::Result"}}}}, "name": "Item"}]}}, "id": "futures_core::stream::Stream", "path": "Stream"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [462, 1], "end": [493, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23ad482c6f187bbb792cf53b"></a>
## schema

`function` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow::record_batch::RecordBatch"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "datafusion_common::Result"}}}}, "name": "Item"}]}}, "id": "futures_core::stream::Stream", "path": "Stream"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [495, 1], "end": [502, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/stream.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f436a308af5c27e0670cba80"></a>
## size_hint

`function` · `datafusion_physical_plan::stream::RecordBatchStreamAdapter::size_hint` · datafusion-physical-plan 55.1.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "S"}}], "constraints": []}}, "id": "datafusion_physical_plan::stream::RecordBatchStreamAdapter", "path": "RecordBatchStreamAdapter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "S"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_array::record_batch::RecordBatch", "path": "arrow::record_batch::RecordBatch"}}}], "constraints": []}}, "id": "datafusion_common::error::Result", "path": "datafusion_common::Result"}}}}, "name": "Item"}]}}, "id": "futures_core::stream::Stream", "path": "Stream"}}}], "generic_params": [], "type": {"generic": "S"}}}]}, "is_negative": false, "span": {"begin": [462, 1], "end": [493, 2], "filename": "src/stream.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/stream.rs:487`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
