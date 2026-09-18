# `deltalake_catalog_unity::models::ColumnInfo`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.ColumnInfo.json).

<a id="op-7208d76751025ab28febcfda"></a>
## ColumnInfo

`struct` · `deltalake_catalog_unity::models::ColumnInfo` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ColumnInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L369).

Source: `crates/catalog-unity/src/models.rs:369`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1548549d7a4a5afa0aaa647"></a>
## clone

`function` · `deltalake_catalog_unity::models::ColumnInfo::clone` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ColumnInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 10], "end": [368, 15], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddf186712c8f2129adbbb944"></a>
## comment

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::comment` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
comment: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L393).

Source: `crates/catalog-unity/src/models.rs:393`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

User-provided free-form text description.

<a id="op-06c5bcb4ae048789630437bc"></a>
## default

`function` · `deltalake_catalog_unity::models::ColumnInfo::default` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ColumnInfo
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 17], "end": [368, 24], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3d903f1078d253496daeeeb2"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::ColumnInfo::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 55], "end": [368, 66], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5830406e1f3879d3cc0c3543"></a>
## eq

`function` · `deltalake_catalog_unity::models::ColumnInfo::eq` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnInfo) -> bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 33], "end": [368, 42], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b466134c742e4bfe585d2149"></a>
## fmt

`function` · `deltalake_catalog_unity::models::ColumnInfo::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 26], "end": [368, 31], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b790b21f705e2c45315e3cc"></a>
## name

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L371).

Source: `crates/catalog-unity/src/models.rs:371`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Name of Column.

<a id="op-2946ac2c71385e79608391eb"></a>
## nullable

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::nullable` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
nullable: bool
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L395).

Source: `crates/catalog-unity/src/models.rs:395`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Whether field may be Null.

<a id="op-7ceba111ac4025e58d5bf10a"></a>
## partition_index

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::partition_index` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
partition_index: Option<i32>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L398).

Source: `crates/catalog-unity/src/models.rs:398`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Partition index for column.

<a id="op-5e0c79d2142337a2c2a4ff3b"></a>
## position

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::position` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
position: u32
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L390).

Source: `crates/catalog-unity/src/models.rs:390`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Ordinal position of column (starting at position 0).

<a id="op-c1d6190e34c4a5c705576032"></a>
## serialize

`function` · `deltalake_catalog_unity::models::ColumnInfo::serialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L368).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::ColumnInfo", "path": "ColumnInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [368, 44], "end": [368, 53], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `crates/catalog-unity/src/models.rs:368`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bd3967178097eb78357ae4a"></a>
## type_interval_type

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_interval_type` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_interval_type: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L388).

Source: `crates/catalog-unity/src/models.rs:388`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Format of IntervalType.

<a id="op-bf4e8b1ebb59c23204b65665"></a>
## type_json

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_json` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_json: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L377).

Source: `crates/catalog-unity/src/models.rs:377`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Full data type specification, JSON-serialized.

<a id="op-3b70479a4af859c1b77b97f8"></a>
## type_name

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_name: Option<ColumnTypeName>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L379).

Source: `crates/catalog-unity/src/models.rs:379`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37abadbfda8c307867fc9201"></a>
## type_precision

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_precision` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_precision: Option<i32>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L382).

Source: `crates/catalog-unity/src/models.rs:382`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Digits of precision; required for DecimalTypes.

<a id="op-cd2498d169cc14731a407bf1"></a>
## type_scale

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_scale` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_scale: Option<i32>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L385).

Source: `crates/catalog-unity/src/models.rs:385`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Digits to right of decimal; Required for DecimalTypes.

<a id="op-349a0aad004c854025e7b29c"></a>
## type_text

`struct_field` · `deltalake_catalog_unity::models::ColumnInfo::type_text` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type_text: Option<String>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L374).

Source: `crates/catalog-unity/src/models.rs:374`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Full data type specification as SQL/catalogString text.
