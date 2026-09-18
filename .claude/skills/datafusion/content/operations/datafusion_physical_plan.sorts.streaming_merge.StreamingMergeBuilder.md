# `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.streaming_merge.StreamingMergeBuilder.json).

<a id="op-8ca119bd5560940129155dad"></a>
## StreamingMergeBuilder

`struct` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder` · datafusion-physical-plan 55.1.0

```rust
struct StreamingMergeBuilder<'a>
```

Source: `src/sorts/streaming_merge.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6ec8f19d1d44d5b178877cf"></a>
## build

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::build` · datafusion-physical-plan 55.1.0

```rust
fn build(self) -> Result<SendableRecordBatchStream>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb8d137d7351f0b4a937af4a"></a>
## default

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> StreamingMergeBuilder<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [87, 10], "end": [87, 17], "filename": "src/sorts/streaming_merge.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sorts/streaming_merge.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab657f2c12f784d69fdd804f"></a>
## new

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::new` · datafusion-physical-plan 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1876876039e7ecc127b8ba4b"></a>
## with_batch_size

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_batch_size` · datafusion-physical-plan 55.1.0

```rust
fn with_batch_size(self, batch_size: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:142`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a97e3794ab7aa7165a28382"></a>
## with_expressions

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_expressions` · datafusion-physical-plan 55.1.0

```rust
fn with_expressions(self, expressions: &'a LexOrdering) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e07600639f130fb580eca6b"></a>
## with_fetch

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_fetch` · datafusion-physical-plan 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-798cf0b597167ab469a09f1b"></a>
## with_metrics

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_metrics` · datafusion-physical-plan 55.1.0

```rust
fn with_metrics(self, metrics: BaselineMetrics) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bed72c6fbb359f97eb4c8887"></a>
## with_reservation

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_reservation` · datafusion-physical-plan 55.1.0

```rust
fn with_reservation(self, reservation: MemoryReservation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2567951ced010b37c99f44b2"></a>
## with_round_robin_tie_breaker

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_round_robin_tie_breaker` · datafusion-physical-plan 55.1.0

```rust
fn with_round_robin_tie_breaker(self, enable_round_robin_tie_breaker: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

See [SortPreservingMergeExec::with_round_robin_repartition] for more
information.

[SortPreservingMergeExec::with_round_robin_repartition]: crate::sorts::sort_preserving_merge::SortPreservingMergeExec::with_round_robin_repartition

<a id="op-a029de51bc7b438b9421a0dc"></a>
## with_schema

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_schema` · datafusion-physical-plan 55.1.0

```rust
fn with_schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f45a11233888c24e516d0173"></a>
## with_sorted_spill_files

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_sorted_spill_files` · datafusion-physical-plan 55.1.0

```rust
fn with_sorted_spill_files(self, sorted_spill_files: Vec<SortedSpillFile>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f602ed020d758cc5f2e1864"></a>
## with_spill_manager

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_spill_manager` · datafusion-physical-plan 55.1.0

```rust
fn with_spill_manager(self, spill_manager: SpillManager) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4359e8871e7478b429b23f9"></a>
## with_streams

`function` · `datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder::with_streams` · datafusion-physical-plan 55.1.0

```rust
fn with_streams(self, streams: Vec<SendableRecordBatchStream>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_physical_plan::sorts::streaming_merge::StreamingMergeBuilder", "path": "StreamingMergeBuilder"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [101, 1], "end": [275, 2], "filename": "src/sorts/streaming_merge.rs"}, "trait": null, "trait_path": null}`

Source: `src/sorts/streaming_merge.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
