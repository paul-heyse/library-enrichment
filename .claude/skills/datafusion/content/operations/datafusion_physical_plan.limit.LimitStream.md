# `datafusion_physical_plan::limit::LimitStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.limit.LimitStream.json).

<a id="op-e06577b4a09e09f040bc019f"></a>
## LimitStream

`struct` · `datafusion_physical_plan::limit::LimitStream` · datafusion-physical-plan 55.1.0

```rust
struct LimitStream
```

Source: `src/limit.rs:586`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A Limit stream skips `skip` rows, and then fetch up to `fetch` rows.

<a id="op-4b67931a9bbafe64e273f282"></a>
## Item

`assoc_type` · `datafusion_physical_plan::limit::LimitStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LimitStream", "path": "LimitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 1], "end": [701, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/limit.rs:675`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b51d5957a57fac042cf0033b"></a>
## new

`function` · `datafusion_physical_plan::limit::LimitStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(input: SendableRecordBatchStream, skip: usize, fetch: Option<usize>, baseline_metrics: BaselineMetrics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LimitStream", "path": "LimitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [600, 1], "end": [672, 2], "filename": "src/limit.rs"}, "trait": null, "trait_path": null}`

Source: `src/limit.rs:601`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48a9cc4d04bd67af7e40748f"></a>
## poll_next

`function` · `datafusion_physical_plan::limit::LimitStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LimitStream", "path": "LimitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [674, 1], "end": [701, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/limit.rs:677`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eeeeb6ddb7b5874bd18e26ad"></a>
## schema

`function` · `datafusion_physical_plan::limit::LimitStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::limit::LimitStream", "path": "LimitStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [703, 1], "end": [708, 2], "filename": "src/limit.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/limit.rs:705`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the schema
