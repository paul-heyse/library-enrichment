# `datafusion_physical_plan::test::TestPartitionStream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.TestPartitionStream.json).

<a id="op-682df6ea97ec4b1b8ccd560c"></a>
## TestPartitionStream

`struct` · `datafusion_physical_plan::test::TestPartitionStream` · datafusion-physical-plan 55.1.0

```rust
struct TestPartitionStream
```

Source: `src/test.rs:517`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-896cf5d0223f6731fc6da346"></a>
## batches

`struct_field` · `datafusion_physical_plan::test::TestPartitionStream::batches` · datafusion-physical-plan 55.1.0

```rust
batches: Vec<arrow::array::RecordBatch>
```

Source: `src/test.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e531dbfba9c6772d31d7792"></a>
## execute

`function` · `datafusion_physical_plan::test::TestPartitionStream::execute` · datafusion-physical-plan 55.1.0

```rust
fn execute(&self, _ctx: Arc<TaskContext>) -> SendableRecordBatchStream
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestPartitionStream", "path": "TestPartitionStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [540, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::streaming::PartitionStream", "path": "PartitionStream"}, "trait_path": "datafusion_physical_plan::streaming::PartitionStream"}`

Source: `src/test.rs:533`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a18b7e5671b58a6b75e46d99"></a>
## fmt

`function` · `datafusion_physical_plan::test::TestPartitionStream::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestPartitionStream", "path": "TestPartitionStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [516, 10], "end": [516, 15], "filename": "src/test.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test.rs:516`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d5e94d772d551ce8605e605"></a>
## new_with_batches

`function` · `datafusion_physical_plan::test::TestPartitionStream::new_with_batches` · datafusion-physical-plan 55.1.0

```rust
fn new_with_batches(batches: Vec<RecordBatch>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestPartitionStream", "path": "TestPartitionStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [522, 1], "end": [528, 2], "filename": "src/test.rs"}, "trait": null, "trait_path": null}`

Source: `src/test.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Create a new stream partition with the provided batches

<a id="op-70c86e8dc9ead6b85b5bd363"></a>
## schema

`struct_field` · `datafusion_physical_plan::test::TestPartitionStream::schema` · datafusion-physical-plan 55.1.0

```rust
schema: arrow_schema::SchemaRef
```

Source: `src/test.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb27083309b1569181e3c87"></a>
## schema

`function` · `datafusion_physical_plan::test::TestPartitionStream::schema` · datafusion-physical-plan 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::test::TestPartitionStream", "path": "TestPartitionStream"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 1], "end": [540, 2], "filename": "src/test.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::streaming::PartitionStream", "path": "PartitionStream"}, "trait_path": "datafusion_physical_plan::streaming::PartitionStream"}`

Source: `src/test.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
