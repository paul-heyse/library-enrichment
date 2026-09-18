# `deltalake_core::table::blind::BlindDeltaTable`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.table.blind.BlindDeltaTable.json).

<a id="op-35b09ea65b0f509818992b56"></a>
## BlindDeltaTable

`struct` · `deltalake_core::table::blind::BlindDeltaTable` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct BlindDeltaTable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L64).

Source: `crates/core/src/table/blind.rs:64`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A Delta table optimized for blind append-only write operations.

`BlindDeltaTable` loads only the table metadata (protocol, schema, properties)
without scanning file statistics. This makes it ideal for:

- Large tables with many files where stats parsing is expensive
- Append-only workloads that don't need to read existing data
- High-throughput write scenarios

# Type Safety

This type intentionally does not expose methods like `files()` or `log_data()`
that would require loading file statistics. This prevents accidental use of
operations like merge or delete that are incompatible with append-only tables.

# Kernel Transaction API

This implementation uses the Kernel Transaction API directly for commits,
bypassing the `CommitBuilder` used by the standard `DeltaTable`. This provides
a more lightweight commit path optimized for append-only workloads.

<a id="op-b26df338c70868bd92be03c4"></a>
## arrow_schema

`function` · `deltalake_core::table::blind::BlindDeltaTable::arrow_schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn arrow_schema(&self) -> DeltaResult<ArrowSchemaRef>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L158).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:158`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table schema (Arrow format).

<a id="op-21c1a2684b082c71938f5ecb"></a>
## clone

`function` · `deltalake_core::table::blind::BlindDeltaTable::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> BlindDeltaTable
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 17], "end": [63, 22], "filename": "crates/core/src/table/blind.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/table/blind.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41c3c9cdc110f81ffa719484"></a>
## commit

`function` · `deltalake_core::table::blind::BlindDeltaTable::commit` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn commit(&mut self, adds: Vec<Add>) -> DeltaResult<u64>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L189).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:189`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Commit add actions to the table.

This commits the provided Add actions as an append operation to the Delta table
using the Kernel Transaction API directly.

# Arguments

* `adds` - The Add file actions to commit

# Returns

The new version number after the commit.

<a id="op-2428147a371aeb28a1ac138c"></a>
## fmt

`function` · `deltalake_core::table::blind::BlindDeltaTable::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L63).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 10], "end": [63, 15], "filename": "crates/core/src/table/blind.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/table/blind.rs:63`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34d71103226516c3724f53c"></a>
## is_append_only

`function` · `deltalake_core::table::blind::BlindDeltaTable::is_append_only` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_append_only(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L173).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:173`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Check if this table has the `appendOnly` property set to true.

<a id="op-224de2532fddf93b76b89c6f"></a>
## log_store

`function` · `deltalake_core::table::blind::BlindDeltaTable::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn log_store(&self) -> LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:123`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a reference to the underlying log store.

<a id="op-46cee16b9ee5410587876711"></a>
## metadata

`function` · `deltalake_core::table::blind::BlindDeltaTable::metadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata(&self) -> &Metadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L143).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:143`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table metadata.

<a id="op-6361dae79439a48e799f3400"></a>
## object_store

`function` · `deltalake_core::table::blind::BlindDeltaTable::object_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn object_store(&self) -> ObjectStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:128`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a reference to the object store.

<a id="op-446969e38f7ada18317eb729"></a>
## protocol

`function` · `deltalake_core::table::blind::BlindDeltaTable::protocol` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn protocol(&self) -> &Protocol
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L148).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:148`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table protocol.

<a id="op-328898f9214fd6497a5e6106"></a>
## schema

`function` · `deltalake_core::table::blind::BlindDeltaTable::schema` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn schema(&self) -> KernelSchemaRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:153`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table schema (kernel format).

<a id="op-5e6496dc8cae683c1b2237f2"></a>
## snapshot

`function` · `deltalake_core::table::blind::BlindDeltaTable::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn snapshot(&self) -> &Snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L168).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:168`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get a reference to the underlying snapshot.

<a id="op-db1b0ccfb6803944ae1f1597"></a>
## table_properties

`function` · `deltalake_core::table::blind::BlindDeltaTable::table_properties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_properties(&self) -> &TableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L163).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:163`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table properties.

<a id="op-6dee012227b65bf7e411c3c2"></a>
## table_url

`function` · `deltalake_core::table::blind::BlindDeltaTable::table_url` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn table_url(&self) -> &Url
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L133).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:133`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table URL.

<a id="op-922d2439ce9dd0d6df72f6da"></a>
## try_new

`function` · `deltalake_core::table::blind::BlindDeltaTable::try_new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new(table_uri: impl AsRef<str>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L83).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:83`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`BlindDeltaTable`](../operations/deltalake_core.table.blind.BlindDeltaTable.md#op-35b09ea65b0f509818992b56) from a table URI.

This loads only the table metadata without parsing file statistics.

# Arguments

* `table_uri` - The URI of the Delta table (e.g., "s3://bucket/table", "/path/to/table")

# Example

```ignore
let table = BlindDeltaTable::try_new("s3://bucket/my-table").await?;
```

<a id="op-5042342cd31a1e29882e7745"></a>
## try_new_with_log_store

`function` · `deltalake_core::table::blind::BlindDeltaTable::try_new_with_log_store` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new_with_log_store(log_store: LogStoreRef) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L109).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:109`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`BlindDeltaTable`](../operations/deltalake_core.table.blind.BlindDeltaTable.md#op-35b09ea65b0f509818992b56) from an existing log store.

# Arguments

* `log_store` - The log store to use

<a id="op-3e1e2e484c92f18827343310"></a>
## try_new_with_options

`function` · `deltalake_core::table::blind::BlindDeltaTable::try_new_with_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
async fn try_new_with_options(table_uri: impl AsRef<str>, storage_options: HashMap<String, String>) -> DeltaResult<Self>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L93).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:93`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create a new [`BlindDeltaTable`](../operations/deltalake_core.table.blind.BlindDeltaTable.md#op-35b09ea65b0f509818992b56) with storage options.

# Arguments

* `table_uri` - The URI of the Delta table
* `storage_options` - Backend-specific storage options

<a id="op-1bfe9b78649d8b18460be602"></a>
## version

`function` · `deltalake_core::table::blind::BlindDeltaTable::version` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn version(&self) -> u64
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L138).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::table::blind::BlindDeltaTable", "path": "BlindDeltaTable"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [246, 2], "filename": "crates/core/src/table/blind.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/table/blind.rs:138`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get the table version.

<a id="op-5072eb3e52886bb0063c2d9c"></a>
## log_store

`struct_field` · `deltalake_core::table::blind::BlindDeltaTable::log_store` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
log_store: logstore::LogStoreRef
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L65).

Source: `crates/core/src/table/blind.rs:65`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b72c297e27d3da06d1bd91e9"></a>
## snapshot

`struct_field` · `deltalake_core::table::blind::BlindDeltaTable::snapshot` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
snapshot: kernel::Snapshot
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/table/blind.rs#L66).

Source: `crates/core/src/table/blind.rs:66`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
