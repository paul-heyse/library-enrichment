# `datafusion_physical_plan::test::exec::BlockingStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.BlockingStream.json).

<a id="op-b5360a7293aa12e351eb29b3"></a>
## BlockingStream

`struct` · `datafusion_physical_plan::test::exec::BlockingStream` · datafusion-physical-plan 55.1.0

```rust
struct BlockingStream
```

Source: `src/test/exec.rs:883`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

A [`RecordBatchStream`](../operations/datafusion_execution.stream.RecordBatchStream.md#op-6a21cda33374b9fc18902881) that is pending forever.

<a id="op-65467525ee98308cf57c346e"></a>
## Item

`assoc_type` · `datafusion_physical_plan::test::exec::BlockingStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingStream", "path": "BlockingStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [900, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/test/exec.rs:892`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9c02b178afb49a286030b61"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::BlockingStream::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingStream", "path": "BlockingStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [882, 10], "end": [882, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:882`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afa7517973a7dc15ee8a1903"></a>
## poll_next

`function` · `datafusion_physical_plan::test::exec::BlockingStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingStream", "path": "BlockingStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [900, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/test/exec.rs:894`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc803353002fb7d0c437f492"></a>
## schema

`function` · `datafusion_physical_plan::test::exec::BlockingStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::BlockingStream", "path": "BlockingStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [902, 1], "end": [906, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/test/exec.rs:903`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
