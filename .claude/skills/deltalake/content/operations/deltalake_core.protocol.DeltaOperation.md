# `deltalake_core::protocol::DeltaOperation`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.protocol.DeltaOperation.json).

<a id="op-81f68d17e9975b7e4af595f4"></a>
## DeltaOperation

`enum` · `deltalake_core::protocol::DeltaOperation` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum DeltaOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L232).

Source: `crates/core/src/protocol/mod.rs:232`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Operation performed when creating a new log entry with one or more actions.
This is a key element of the `CommitInfo` action.

<a id="op-d4c1fa66b63d277e29f60039"></a>
## AddColumn

`variant` · `deltalake_core::protocol::DeltaOperation::AddColumn` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AddColumn
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L235).

Source: `crates/core/src/protocol/mod.rs:235`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a Delta `Add Column` operation.
Used to add new columns or field in a struct

<a id="op-cc2ed626ee4745bcd2c6a09f"></a>
## AddConstraint

`variant` · `deltalake_core::protocol::DeltaOperation::AddConstraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AddConstraint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L278).

Source: `crates/core/src/protocol/mod.rs:278`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add constraints to a table

<a id="op-e6e363cfc4a1497ce82ac7f8"></a>
## AddFeature

`variant` · `deltalake_core::protocol::DeltaOperation::AddFeature` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
AddFeature
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L284).

Source: `crates/core/src/protocol/mod.rs:284`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Add table features to a table

<a id="op-b3fe81d9f27862f0d3e25c4b"></a>
## Create

`variant` · `deltalake_core::protocol::DeltaOperation::Create` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Create
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L243).

Source: `crates/core/src/protocol/mod.rs:243`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a Delta `Create` operation.
Would usually only create the table, if also data is written,
a `Write` operations is more appropriate

<a id="op-9b03e40aa3b80a2233d98303"></a>
## Delete

`variant` · `deltalake_core::protocol::DeltaOperation::Delete` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Delete
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L267).

Source: `crates/core/src/protocol/mod.rs:267`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Delete data matching predicate from delta table

<a id="op-ecfd690d5fbb52f4d122162b"></a>
## DropColumnNotNull

`variant` · `deltalake_core::protocol::DeltaOperation::DropColumnNotNull` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DropColumnNotNull
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L383).

Source: `crates/core/src/protocol/mod.rs:383`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drop the `NOT NULL` constraint on a column, making it nullable

<a id="op-9a182610b0834196730656ee"></a>
## DropConstraint

`variant` · `deltalake_core::protocol::DeltaOperation::DropConstraint` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
DropConstraint
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L290).

Source: `crates/core/src/protocol/mod.rs:290`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Drops constraints from a table

<a id="op-ea403aadf0c37d1901b2a12c"></a>
## FileSystemCheck

`variant` · `deltalake_core::protocol::DeltaOperation::FileSystemCheck` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
FileSystemCheck
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L343).

Source: `crates/core/src/protocol/mod.rs:343`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a `FileSystemCheck` operation

<a id="op-a040347eb3f35ccaee7fe042"></a>
## Merge

`variant` · `deltalake_core::protocol::DeltaOperation::Merge` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Merge
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L297).

Source: `crates/core/src/protocol/mod.rs:297`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Merge data with a source data with the following predicate

<a id="op-0eb5a74d563c7c64c512e8ff"></a>
## Optimize

`variant` · `deltalake_core::protocol::DeltaOperation::Optimize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Optimize
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L334).

Source: `crates/core/src/protocol/mod.rs:334`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a `Optimize` operation

<a id="op-19621a59897f66261968acab"></a>
## Restore

`variant` · `deltalake_core::protocol::DeltaOperation::Restore` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Restore
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L346).

Source: `crates/core/src/protocol/mod.rs:346`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a `Restore` operation

<a id="op-25d9d79a1bcf3f09ba0f7f40"></a>
## SetTableProperties

`variant` · `deltalake_core::protocol::DeltaOperation::SetTableProperties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
SetTableProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L327).

Source: `crates/core/src/protocol/mod.rs:327`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set table properties operations

<a id="op-ce86c7e9b8284f06ce3b8b4b"></a>
## StreamingUpdate

`variant` · `deltalake_core::protocol::DeltaOperation::StreamingUpdate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
StreamingUpdate
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L316).

Source: `crates/core/src/protocol/mod.rs:316`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a Delta `StreamingUpdate` operation.

<a id="op-2e1da11271af9548e4352b85"></a>
## Update

`variant` · `deltalake_core::protocol::DeltaOperation::Update` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Update
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L273).

Source: `crates/core/src/protocol/mod.rs:273`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update data matching predicate from delta table

<a id="op-bbac03cf83686a893ac8c128"></a>
## UpdateFieldMetadata

`variant` · `deltalake_core::protocol::DeltaOperation::UpdateFieldMetadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UpdateFieldMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L371).

Source: `crates/core/src/protocol/mod.rs:371`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Set table field metadata operations

<a id="op-b3dc7dbd3d52f1f4bc925d72"></a>
## UpdateTableMetadata

`variant` · `deltalake_core::protocol::DeltaOperation::UpdateTableMetadata` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
UpdateTableMetadata
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L377).

Source: `crates/core/src/protocol/mod.rs:377`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Update table metadata operations

<a id="op-0d06dc058ac5d9a3bbfe23dc"></a>
## VacuumEnd

`variant` · `deltalake_core::protocol::DeltaOperation::VacuumEnd` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VacuumEnd
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L365).

Source: `crates/core/src/protocol/mod.rs:365`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents the end of `Vacuum` operation

<a id="op-33dcdfa9c92c7714aee7b03c"></a>
## VacuumStart

`variant` · `deltalake_core::protocol::DeltaOperation::VacuumStart` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
VacuumStart
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L355).

Source: `crates/core/src/protocol/mod.rs:355`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents the start of `Vacuum` operation

<a id="op-aa5eee63e05cc045dcd74745"></a>
## Write

`variant` · `deltalake_core::protocol::DeltaOperation::Write` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Write
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L257).

Source: `crates/core/src/protocol/mod.rs:257`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Represents a Delta `Write` operation.
Write operations will typically only include `Add` actions.

<a id="op-bd33016b8b35f386e44e8eef"></a>
## changes_data

`function` · `deltalake_core::protocol::DeltaOperation::changes_data` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn changes_data(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L439).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:439`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Denotes if the operation changes the data contained in the table

<a id="op-289b3d410d5a5e5e7b0cb975"></a>
## clone

`function` · `deltalake_core::protocol::DeltaOperation::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> DeltaOperation
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 41], "end": [230, 46], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/protocol/mod.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0bcf992f2d692671dbcbd71"></a>
## deserialize

`function` · `deltalake_core::protocol::DeltaOperation::deserialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 21], "end": [230, 32], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/core/src/protocol/mod.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27dd31308339b5c5913255de"></a>
## fmt

`function` · `deltalake_core::protocol::DeltaOperation::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 34], "end": [230, 39], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/protocol/mod.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a5957264397b4cfe605b07a"></a>
## get_commit_info

`function` · `deltalake_core::protocol::DeltaOperation::get_commit_info` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_commit_info(&self) -> CommitInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L464).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:464`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Retrieve basic commit information to be added to Delta commits

<a id="op-87e39935540b80a8686e512a"></a>
## name

`function` · `deltalake_core::protocol::DeltaOperation::name` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &str
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L391).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:391`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A human readable name for the operation

<a id="op-d0da279175549deff02cf382"></a>
## operation_parameters

`function` · `deltalake_core::protocol::DeltaOperation::operation_parameters` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn operation_parameters(&self) -> DeltaResult<HashMap<String, Value>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L421).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:421`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parameters configured for operation.

<a id="op-bad259c19f927685ccc9a34b"></a>
## read_predicate

`function` · `deltalake_core::protocol::DeltaOperation::read_predicate` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_predicate(&self) -> Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L475).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:475`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Get predicate expression applied when the operation reads data from the table.

<a id="op-89c085126beedcdcda852439"></a>
## read_whole_table

`function` · `deltalake_core::protocol::DeltaOperation::read_whole_table` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn read_whole_table(&self) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L487).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [389, 1], "end": [494, 2], "filename": "crates/core/src/protocol/mod.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/protocol/mod.rs:487`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Denotes if the operation reads the entire table

<a id="op-3e598d571cd2ecfedaf367a7"></a>
## serialize

`function` · `deltalake_core::protocol::DeltaOperation::serialize` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/protocol/mod.rs#L230).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [230, 10], "end": [230, 19], "filename": "crates/core/src/protocol/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/core/src/protocol/mod.rs:230`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d281399aec88f22f5c46e1c"></a>
## Error

`assoc_type` · `deltalake_core::protocol::DeltaOperation::Error` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = DeltaTableError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L501).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "crate::protocol::DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [509, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::OptimizeInput", "path": "OptimizeInput"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `crates/core/src/operations/optimize.rs:501`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4953d099f03dc30d1c54aa21"></a>
## try_from

`function` · `deltalake_core::protocol::DeltaOperation::try_from` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(opt_input: OptimizeInput) -> Result<Self, Self::Error>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/optimize.rs#L503).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::protocol::DeltaOperation", "path": "crate::protocol::DeltaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [500, 1], "end": [509, 2], "filename": "crates/core/src/operations/optimize.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "deltalake_core::operations::optimize::OptimizeInput", "path": "OptimizeInput"}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `crates/core/src/operations/optimize.rs:503`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.
