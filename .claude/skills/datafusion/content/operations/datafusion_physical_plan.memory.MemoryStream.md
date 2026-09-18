# `datafusion_physical_plan::memory::MemoryStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.memory.MemoryStream.json).

<a id="op-f3baf435a0fc5d1a28128b0d"></a>
## MemoryStream

`struct` · `datafusion_physical_plan::memory::MemoryStream` · datafusion-physical-plan 55.1.0

```rust
struct MemoryStream
```

Source: `src/memory.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Iterator over batches

<a id="op-dab2ccf441ed5f3ba1eb30a8"></a>
## Item

`assoc_type` · `datafusion_physical_plan::memory::MemoryStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [139, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/memory.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1573080350f46efd8578554"></a>
## poll_next

`function` · `datafusion_physical_plan::memory::MemoryStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(std::pin::Pin<&mut self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [139, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/memory.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edfef12c3891692cfe5c58f8"></a>
## schema

`function` · `datafusion_physical_plan::memory::MemoryStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [146, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/memory.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the schema

<a id="op-2c60b39715b25afa4a19a13b"></a>
## size_hint

`function` · `datafusion_physical_plan::memory::MemoryStream::size_hint` · datafusion-physical-plan 55.1.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [139, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/memory.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-109bbb26bb3e86b1d322708c"></a>
## try_new

`function` · `datafusion_physical_plan::memory::MemoryStream::try_new` · datafusion-physical-plan 55.1.0

```rust
fn try_new(data: Vec<RecordBatch>, schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [89, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create an iterator for a vector of record batches

<a id="op-a415d9091a44815312ae1bee"></a>
## with_fetch

`function` · `datafusion_physical_plan::memory::MemoryStream::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [89, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the number of rows to produce

<a id="op-a898ad4cdffc3b406d04b0d8"></a>
## with_reservation

`function` · `datafusion_physical_plan::memory::MemoryStream::with_reservation` · datafusion-physical-plan 55.1.0

```rust
fn with_reservation(self, reservation: MemoryReservation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::MemoryStream", "path": "MemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [89, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Set the memory reservation for the data
