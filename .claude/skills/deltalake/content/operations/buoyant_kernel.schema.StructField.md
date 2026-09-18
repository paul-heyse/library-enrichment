# `buoyant_kernel::schema::StructField`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.StructField.json).

<a id="op-ebbc3bdce7a671f7a7d1f58d"></a>
## StructField

`struct` · `buoyant_kernel::schema::StructField` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L369).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:369`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8918ca036bd93d334b94ab9f"></a>
## add_metadata

`function` · `buoyant_kernel::schema::StructField::add_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L468).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:468`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extends `self.metadata` to include the <key, value> pairs in `metadata`.

<a id="op-a65d1fba700bb717122e05e3"></a>
## as_internal_column

`function` · `buoyant_kernel::schema::StructField::as_internal_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_internal_column(self) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L506).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:506`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Marks this field as an internal column.

<a id="op-fc46974ae862483953dd871d"></a>
## clone

`function` · `buoyant_kernel::schema::StructField::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 52], "end": [368, 57], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18facdef8cf7dd32b2ef98b7"></a>
## column_mapping_id

`function` · `buoyant_kernel::schema::StructField::column_mapping_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn column_mapping_id(&self) -> Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L519).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:519`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns this field's `delta.columnMapping.id` annotation if present and well-formed.
Returns `None` if the annotation is missing or carries a non-numeric value.

<a id="op-8ab7b61e79cb25d86c9adfd3"></a>
## create_metadata_column

`function` · `buoyant_kernel::schema::StructField::create_metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn create_metadata_column(name: impl Into<String>, spec: MetadataColumnSpec) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L425).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:425`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a metadata column of the given spec with the given name.

<a id="op-4d5cbecc4fc76ac41dea62b4"></a>
## data_type

`struct_field` · `buoyant_kernel::schema::StructField::data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
data_type: DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L374).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:374`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The data type of this field

<a id="op-5511e86aee8df1cb5adabc4b"></a>
## data_type

`function` · `buoyant_kernel::schema::StructField::data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const fn data_type(&self) -> &DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L698).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:698`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0325b5d50f3550a90280b046"></a>
## default_row_index_column

`function` · `buoyant_kernel::schema::StructField::default_row_index_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default_row_index_column() -> &'static StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L441).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:441`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the default row index metadata column used by Kernel.

<a id="op-e9c8ca9020a1a9352803db47"></a>
## deserialize

`function` · `buoyant_kernel::schema::StructField::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 28], "end": [368, 39], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87b0ce99029668679c10ad88"></a>
## eq

`function` · `buoyant_kernel::schema::StructField::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &StructField) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 41], "end": [368, 50], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14790507fc3b77b062e43245"></a>
## fmt

`function` · `buoyant_kernel::schema::StructField::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 10], "end": [368, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69453bbdedf0ddbcf7db2d18"></a>
## fmt

`function` · `buoyant_kernel::schema::StructField::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L815).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [832, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:815`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89c630064ef5021a1b189724"></a>
## get_config_value

`function` · `buoyant_kernel::schema::StructField::get_config_value` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_config_value(&self, key: &ColumnMetadataKey) -> Option<&MetadataValue>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L513).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:513`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9308243e148673661b1bf8c7"></a>
## get_metadata_column_spec

`function` · `buoyant_kernel::schema::StructField::get_metadata_column_spec` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn get_metadata_column_spec(&self) -> Option<MetadataColumnSpec>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L484).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:484`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the metadata column spec if this is a metadata column, otherwise returns None.

<a id="op-dc815b59d8b97b60b974ecc8"></a>
## is_internal_column

`function` · `buoyant_kernel::schema::StructField::is_internal_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_internal_column(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L497).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:497`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if this field is an internal column added by Kernel.

Internal columns must be removed before returning scan results to the user.

<a id="op-8791669e64f3494ea7e65f28"></a>
## is_metadata_column

`function` · `buoyant_kernel::schema::StructField::is_metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_metadata_column(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L478).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:478`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns true if this field is a metadata column.

<a id="op-b2d103e59019490b2118046a"></a>
## is_nullable

`function` · `buoyant_kernel::schema::StructField::is_nullable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn is_nullable(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L693).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:693`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a9d541a7159405830d51b33"></a>
## make_physical

`function` · `buoyant_kernel::schema::StructField::make_physical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn make_physical(&self, column_mapping_mode: ColumnMappingMode) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L736).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:736`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Applies physical name and field ID mappings to this field.

This function sets the field ID for the physical [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) only if the
`column_mapping_mode` is `Id`. The field ID is specified using the
[`ColumnMetadataKey::ParquetFieldId`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-bb9afd4dbf92fc431ada321c) metadata field. Readers should use
[`ColumnMetadataKey::ParquetFieldId`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-bb9afd4dbf92fc431ada321c) to match fields to the Parquet schema.
If a physical StructField contains a field ID, the reader must resolve columns
with that ID. Otherwise, the physical StructField's name is used. For details,
see [`read_parquet_files`].

This function also sets the physical name of a field. If `column_mapping_mode` is
`Id` or `Name`, this is specified in [`ColumnMetadataKey::ColumnMappingPhysicalName`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-f240b92caf480511b417ff31).
Otherwise, the field's logical name is used.

Returns an error if a field has invalid or inconsistent column mapping annotations (e.g.
missing when column mapping is enabled, present when disabled, or wrong type), or if a
metadata column is encountered (metadata columns should not participate in column mapping).

[`read_parquet_files`]: crate::ParquetHandler::read_parquet_files

<a id="op-ef9cbb0f4cc324c00b2ed7af"></a>
## metadata

`function` · `buoyant_kernel::schema::StructField::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
const fn metadata(&self) -> &HashMap<String, MetadataValue>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L703).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:703`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f76217b7fd0b795645fb2405"></a>
## metadata

`struct_field` · `buoyant_kernel::schema::StructField::metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
metadata: std::collections::HashMap<String, MetadataValue>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L378).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:378`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A JSON map containing information about this column

<a id="op-eadfcc497f25d39d7506ec79"></a>
## metadata_with_string_values

`function` · `buoyant_kernel::schema::StructField::metadata_with_string_values` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata_with_string_values(&self) -> HashMap<String, String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L709).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:709`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Convert our metadata into a HashMap<String, String>. Note this copies all the data so can be
expensive for large metadata

<a id="op-3f98bbb6096fe7a4ff0af3c4"></a>
## name

`function` · `buoyant_kernel::schema::StructField::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn name(&self) -> &String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc306fe6efb66ea24aadf21c"></a>
## name

`struct_field` · `buoyant_kernel::schema::StructField::name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L371).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:371`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Name of this (possibly nested) column

<a id="op-beb28d52395a2d8493250306"></a>
## new

`function` · `buoyant_kernel::schema::StructField::new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(name: impl Into<String>, data_type: impl Into<DataType>, nullable: bool) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L405).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:405`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new field

<a id="op-f01acf1dace5f76cc8da3cf2"></a>
## not_null

`function` · `buoyant_kernel::schema::StructField::not_null` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn not_null(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L420).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:420`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new non-nullable field

<a id="op-355a305e82bb928393eb44b5"></a>
## nullable

`struct_field` · `buoyant_kernel::schema::StructField::nullable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
nullable: bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L376).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:376`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Denotes whether this Field can be null

<a id="op-74c63e950d41f65ac868909a"></a>
## nullable

`function` · `buoyant_kernel::schema::StructField::nullable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn nullable(name: impl Into<String>, data_type: impl Into<DataType>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L415).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:415`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new nullable field

<a id="op-92726280cb88a3db68064e38"></a>
## physical_name

`function` · `buoyant_kernel::schema::StructField::physical_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn physical_name(&self, column_mapping_mode: ColumnMappingMode) -> &str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L641).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:641`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Get the physical name for this field as it should be read from parquet.

When `column_mapping_mode` is `None`, always returns the logical name (even if physical
name metadata is present). When mode is `Id` or `Name`, returns the physical name from
metadata if present, otherwise returns the logical name.

NOTE: Caller affirms that the schema was already validated by
[`crate::table_configuration::TableConfiguration::try_new`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-d0372a1b1d50b1dd56b7c65c), to ensure that annotations are
always and only present when column mapping mode is enabled.

<a id="op-e199888d3b0274365f71e3ff"></a>
## serialize

`function` · `buoyant_kernel::schema::StructField::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 17], "end": [368, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:368`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6cbc2c0eb6165c5cdf7a576"></a>
## to_schema_field

`function` · `buoyant_kernel::schema::StructField::to_schema_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_schema_field(self) -> StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L153).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [156, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::ToSchemaField", "path": "ToSchemaField"}, "trait_path": "buoyant_kernel::schema::ToSchemaField"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:153`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47c79d400a99746435a26c38"></a>
## try_from_arrow

`function` · `buoyant_kernel::schema::StructField::try_from_arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_arrow(arrow_field: &ArrowField) -> Result<Self, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L430).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "crate::schema::StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [429, 1], "end": [493, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::field::Field", "path": "Field"}}}}}], "constraints": []}}, "id": "buoyant_kernel::engine::arrow_conversion::TryFromArrow", "path": "TryFromArrow"}, "trait_path": "buoyant_kernel::engine::arrow_conversion::TryFromArrow"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:430`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5c39b133e5df297c1ed42a1"></a>
## with_metadata

`function` · `buoyant_kernel::schema::StructField::with_metadata` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_metadata(self, metadata: impl IntoIterator<Item = (impl Into<String>, impl Into<MetadataValue>)>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L456).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:456`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Replaces `self.metadata` with the list of <key, value> pairs in `metadata`.

<a id="op-430664e80adcde7ec3ab9088"></a>
## with_name

`function` · `buoyant_kernel::schema::StructField::with_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_name(&self, new_name: impl Into<String>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L678).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 1], "end": [812, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:678`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Change the name of a field. The field will preserve its data type and nullability. Note that
this allocates a new field.

<a id="op-28524f651cbb400876a69029"></a>
## can_read_as

`function` · `buoyant_kernel::schema::StructField::can_read_as` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn can_read_as(&self, read_field: &Self) -> Result<(), Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/compare.rs#L84).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "super::StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [90, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/compare.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::compare::SchemaComparison", "path": "SchemaComparison"}, "trait_path": "buoyant_kernel::schema::compare::SchemaComparison"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/compare.rs:84`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `Ok` if this [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) can be read as `read_field`. Three requirements must
be satisfied:
    1. The read schema field mustn't be non-nullable if this [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) is nullable.
    2. The both this field and `read_field` must have the same name.
    3. You can read this data type as the `read_field`'s data type.

<a id="op-6c49886350b7ca6d7d2ab5c9"></a>
## into_field

`function` · `buoyant_kernel::schema::StructField::into_field` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn into_field(self) -> StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/struct_patch.rs#L588).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructField", "path": "crate::schema::StructField"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [587, 1], "end": [591, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs"}, "trait": {"args": null, "id": "buoyant_kernel::struct_patch::SchemaPatchItem", "path": "SchemaPatchItem"}, "trait_path": "buoyant_kernel::struct_patch::SchemaPatchItem"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/struct_patch.rs:588`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
