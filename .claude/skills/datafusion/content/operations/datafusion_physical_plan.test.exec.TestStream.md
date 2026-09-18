# `datafusion_physical_plan::test::exec::TestStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.exec.TestStream.json).

<a id="op-8fe27b0cbeb317a73c1cd43e"></a>
## TestStream

`struct` · `datafusion_physical_plan::test::exec::TestStream` · datafusion-physical-plan 55.1.0

```rust
struct TestStream
```

Source: `src/test/exec.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Iterator over batches

<a id="op-0ab75f319f332313126feee4"></a>
## Item

`assoc_type` · `datafusion_physical_plan::test::exec::TestStream::Item` · datafusion-physical-plan 55.1.0

```rust
Item
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [110, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/test/exec.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-213a2458e41a4737b901380f"></a>
## default

`function` · `datafusion_physical_plan::test::exec::TestStream::default` · datafusion-physical-plan 55.1.0

```rust
fn default() -> TestStream
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 24], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/exec.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85bf87ca6ff5b68296eb6238"></a>
## fmt

`function` · `datafusion_physical_plan::test::exec::TestStream::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/exec.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4583c646d6254f1fe5af22c9"></a>
## index

`function` · `datafusion_physical_plan::test::exec::TestStream::index` · datafusion-physical-plan 55.1.0

```rust
fn index(&self) -> BatchIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [90, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a handle to the index counter for this stream

<a id="op-9872655bc95e730ca67dc596"></a>
## new

`function` · `datafusion_physical_plan::test::exec::TestStream::new` · datafusion-physical-plan 55.1.0

```rust
fn new(data: Vec<RecordBatch>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [90, 2], "filename": "src/test/exec.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/exec.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create an iterator for a vector of record batches. Assumes at
least one entry in data (for the schema)

<a id="op-342742fbdc51e30d281c7a8e"></a>
## poll_next

`function` · `datafusion_physical_plan::test::exec::TestStream::poll_next` · datafusion-physical-plan 55.1.0

```rust
fn poll_next(Pin<&mut self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [110, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/test/exec.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6507a255e012ed2c0c9501f"></a>
## schema

`function` · `datafusion_physical_plan::test::exec::TestStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [117, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "datafusion_execution::stream::RecordBatchStream", "path": "RecordBatchStream"}, "trait_path": "datafusion_execution::stream::RecordBatchStream"}`

Source: `src/test/exec.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Get the schema

<a id="op-126c7cdb696fffd4f6b9f5b8"></a>
## size_hint

`function` · `datafusion_physical_plan::test::exec::TestStream::size_hint` · datafusion-physical-plan 55.1.0

```rust
fn size_hint(&self) -> (usize, Option<usize>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::exec::TestStream", "path": "TestStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [92, 1], "end": [110, 2], "filename": "src/test/exec.rs"}, "trait": {"args": null, "id": "futures_core::stream::Stream", "path": "Stream"}, "trait_path": "futures_core::stream::Stream"}`

Source: `src/test/exec.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
