# `datafusion_physical_plan::coop::CooperativeStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.coop.CooperativeStream.json).

<a id="op-449130afa9608dcbf1fd3b4f"></a>
## CooperativeStream

`struct` · `datafusion_physical_plan::coop::CooperativeStream` · datafusion-physical-plan 55.1.0

```rust
struct CooperativeStream<T> where T: RecordBatchStream + Unpin
```

Source: `src/coop.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A stream that passes record batches through unchanged while cooperating with the Tokio runtime.
It consumes cooperative scheduling budget for each returned [`RecordBatch`](../operations/arrow_array.record_batch.RecordBatch.md#op-87f977a95cb312da9259aa34),
allowing other tasks to execute when the budget is exhausted.

See the [module level documentation](crate::coop) for an in-depth discussion.

<a id="op-ef7d6eb90e213b3319f491ad"></a>
## Item

`assoc_type` · `datafusion_physical_plan::coop::CooperativeStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::coop::CooperativeStream", "path": "CooperativeStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [138, 1], "end": [205, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/coop.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbe1b0a1580471c9151622c5"></a>
## new

`function` · `datafusion_physical_plan::coop::CooperativeStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(inner: T) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::coop::CooperativeStream", "path": "CooperativeStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [122, 1], "end": [136, 2], "filename": "src/coop.rs"}, "trait": null, "trait_path": null}`

Source: `src/coop.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Creates a new `CooperativeStream` that wraps the provided stream.
The resulting stream will cooperate with the Tokio scheduler by consuming a unit of
scheduling budget when the wrapped `Stream` returns a record batch.

<a id="op-4d7e97f864902f63554a24c9"></a>
## poll_next

`function` · `datafusion_physical_plan::coop::CooperativeStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::coop::CooperativeStream", "path": "CooperativeStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [138, 1], "end": [205, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/coop.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b8574fc82a0e11c95a2855f"></a>
## schema

`function` · `datafusion_physical_plan::coop::CooperativeStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> Arc<Schema>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_plan::coop::CooperativeStream", "path": "CooperativeStream"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Unpin", "path": "Unpin"}}}], "generic_params": [], "type": {"generic": "T"}}}]}, "is_negative": false, "span": {"begin": [207, 1], "end": [214, 2], "filename": "src/coop.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/coop.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
