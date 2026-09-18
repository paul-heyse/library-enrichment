# `datafusion_datasource::memory::MemSink`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.memory.MemSink.json).

<a id="op-80942009aa20419ae6bb9e86"></a>
## MemSink

`struct` · `datafusion_datasource::memory::MemSink` · datafusion-datasource 55.1.0

```rust
struct MemSink
```

Source: `src/memory.rs:862`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Implements for writing to a [`MemTable`]

[`MemTable`]: <https://docs.rs/datafusion/latest/datafusion/datasource/memory/struct.MemTable.html>

<a id="op-dd8a21e2f6e72731f5025c9d"></a>
## fmt

`function` · `datafusion_datasource::memory::MemSink::fmt` · datafusion-datasource 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemSink", "path": "MemSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [868, 1], "end": [874, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/memory.rs:869`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ba61bbd86c54e66b2d8cf8e"></a>
## fmt_as

`function` · `datafusion_datasource::memory::MemSink::fmt_as` · datafusion-datasource 55.1.0

```rust
fn fmt_as(&self, t: DisplayFormatType, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemSink", "path": "MemSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [876, 1], "end": [889, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_physical_plan::display::DisplayAs", "path": "DisplayAs"}, "trait_path": "datafusion_physical_plan::display::DisplayAs"}`

Source: `src/memory.rs:877`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8a09bebd39edf005759af13"></a>
## schema

`function` · `datafusion_datasource::memory::MemSink::schema` · datafusion-datasource 55.1.0

```rust
fn schema(&self) -> &SchemaRef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemSink", "path": "MemSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [904, 1], "end": [935, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/memory.rs:905`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bf879da73ffe04dd464939a"></a>
## try_new

`function` · `datafusion_datasource::memory::MemSink::try_new` · datafusion-datasource 55.1.0

```rust
fn try_new(batches: Vec<PartitionData>, schema: SchemaRef) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemSink", "path": "MemSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [891, 1], "end": [901, 2], "filename": "src/memory.rs"}, "trait": null, "trait_path": null}`

Source: `src/memory.rs:895`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Creates a new [`MemSink`](../operations/datafusion_datasource.memory.MemSink.md#op-80942009aa20419ae6bb9e86).

The caller is responsible for ensuring that there is at least one partition to insert into.

<a id="op-83180c1b6874222bc28274aa"></a>
## write_all

`function` · `datafusion_datasource::memory::MemSink::write_all` · datafusion-datasource 55.1.0

```rust
async fn write_all(&self, data: SendableRecordBatchStream, _context: &Arc<TaskContext>) -> Result<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_datasource::memory::MemSink", "path": "MemSink"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [904, 1], "end": [935, 2], "filename": "src/memory.rs"}, "trait": {"args": null, "id": "datafusion_datasource::sink::DataSink", "path": "DataSink"}, "trait_path": "datafusion_datasource::sink::DataSink"}`

Source: `src/memory.rs:909`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
