# `datafusion_physical_plan::memory::LazyMemoryStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.memory.LazyMemoryStream.json).

<a id="op-5e18f704e7db84c833797a82"></a>
## LazyMemoryStream

`struct` · `datafusion_physical_plan::memory::LazyMemoryStream` · datafusion-physical-plan 55.1.0

```rust
struct LazyMemoryStream
```

Source: `src/memory.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Stream that generates record batches on demand

<a id="op-ad97fb2812ff2578c15b32f5"></a>
## Item

`assoc_type` · `datafusion_physical_plan::memory::LazyMemoryStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryStream", "path": "LazyMemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [444, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/memory.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9dfcdb6e574d985e943b6745"></a>
## poll_next

`function` · `datafusion_physical_plan::memory::LazyMemoryStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(std::pin::Pin<&mut self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryStream", "path": "LazyMemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 1], "end": [444, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/memory.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d7e971a3a6f008c7c1424a5"></a>
## schema

`function` · `datafusion_physical_plan::memory::LazyMemoryStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::memory::LazyMemoryStream", "path": "LazyMemoryStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [446, 1], "end": [450, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/memory.rs:447`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
