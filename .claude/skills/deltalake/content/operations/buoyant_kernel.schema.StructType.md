# `buoyant_kernel::schema::StructType`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.StructType.json).

<a id="op-ac9d10e1b52f1ac2eacc4e98"></a>
## StructType

`struct` · `buoyant_kernel::schema::StructType` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L837).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:837`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A struct is used to represent both the top-level schema of the table
as well as struct columns that contain nested columns.

<a id="op-741488b90adb36e8a96b0ed1"></a>
## IntoIter

`assoc_type` · `buoyant_kernel::schema::StructType::IntoIter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type IntoIter = StructFieldIntoIter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1390).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1388, 1], "end": [1397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1390`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e905e719e5dba38a6547a15"></a>
## Item

`assoc_type` · `buoyant_kernel::schema::StructType::Item` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Item = StructField
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1389).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1388, 1], "end": [1397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1389`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efc4004b4ca97dc8735e0bf3"></a>
## add

`function` · `buoyant_kernel::schema::StructType::add` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add(&self, fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L998).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:998`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Adds fields to this [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98), returning a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98).

<a id="op-9fd5b23df199f5fa543ac5cc"></a>
## add_metadata_column

`function` · `buoyant_kernel::schema::StructType::add_metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn add_metadata_column(&self, name: impl Into<String>, spec: MetadataColumnSpec) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1003).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1003`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Adds a predefined metadata column to this [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98), returning a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98).

<a id="op-697eb8d66f79659841364b2f"></a>
## builder

`function` · `buoyant_kernel::schema::StructType::builder` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn builder() -> StructTypeBuilder
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L949).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:949`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-21626fd296049affee0e5126"></a>
## clone

`function` · `buoyant_kernel::schema::StructType::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> StructType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L836).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 28], "end": [836, 33], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:836`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36345e196439ccdd3fc203cc"></a>
## contains

`function` · `buoyant_kernel::schema::StructType::contains` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contains(&self, name: impl AsRef<str>) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1022).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1022`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Checks if the [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) contains a field with the specified name.

<a id="op-8b5e28caf896a25f85e4818a"></a>
## contains_metadata_column

`function` · `buoyant_kernel::schema::StructType::contains_metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn contains_metadata_column(&self, spec: &MetadataColumnSpec) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1027).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1027`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Checks if the [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) contains a metadata column with the given spec.

<a id="op-c4a839335de0d51b72e99054"></a>
## deserialize

`function` · `buoyant_kernel::schema::StructType::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de>, Self: Sized
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1688).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1687, 1], "end": [1696, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1688`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e17e438178661854bbe1694"></a>
## eq

`function` · `buoyant_kernel::schema::StructType::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &StructType) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L836).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 17], "end": [836, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:836`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a655c32ef6f44f3f8662749"></a>
## field

`function` · `buoyant_kernel::schema::StructType::field` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field(&self, name: impl AsRef<str>) -> Option<&StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1032).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1032`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the field with the given name.

<a id="op-8dc6f76a9a8c32de766bc684"></a>
## field_at

`function` · `buoyant_kernel::schema::StructType::field_at` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field_at<'a>(&'a self, col: &ColumnName) -> DeltaResult<&'a StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1040).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1040`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Retrieves the nested field named by the given column path.

Returns an error if the path is empty, a field is not found, or an intermediate field is not
a struct type.

<a id="op-009cd1f2f4aa36d00b2ac606"></a>
## field_at_index

`function` · `buoyant_kernel::schema::StructType::field_at_index` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field_at_index(&self, index: usize) -> Option<&StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1123).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1123`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the field at the given index.

<a id="op-b49f0e0d241fd740a355818d"></a>
## field_names

`function` · `buoyant_kernel::schema::StructType::field_names` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field_names(&self) -> impl ExactSizeIterator<Item = &String>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1192).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1192`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets all the field names in this struct type in the order they are defined.

<a id="op-4095da08e38f3294e75ff8b9"></a>
## field_with_index

`function` · `buoyant_kernel::schema::StructType::field_with_index` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn field_with_index(&self, name: impl AsRef<str>) -> Option<(usize, &StructField)>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1116).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1116`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the field with the given name and its index.

<a id="op-5709a1a992bfde5252f24b81"></a>
## fields

`function` · `buoyant_kernel::schema::StructType::fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fields(&self) -> impl ExactSizeIterator<Item = &StructField> + DoubleEndedIterator + FusedIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1128).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1128`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets a reference to all the fields in this struct type.

<a id="op-913fe05c60397138807e2407"></a>
## fields_of_path

`function` · `buoyant_kernel::schema::StructType::fields_of_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fields_of_path<'a>(&'a self, col: &ColumnName) -> DeltaResult<Vec<&'a StructField>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1068).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1068`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Resolves a column path through nested structs, returning references to all
[`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s along the path. The last element is the leaf field.

Each element of the path must resolve to a field in the current struct. All intermediate
(non-leaf) fields must be struct types.

Returns an error if the path is empty, a field is not found, or an intermediate
field is not a struct type.

<a id="op-d4d5f66abca95e00a17af64d"></a>
## fmt

`function` · `buoyant_kernel::schema::StructType::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1381).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1380, 1], "end": [1386, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1381`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0d352510da321302b5c9436"></a>
## fmt

`function` · `buoyant_kernel::schema::StructType::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L836).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [836, 10], "end": [836, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:836`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50041b80de357714d5da64d2"></a>
## index_of

`function` · `buoyant_kernel::schema::StructType::index_of` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn index_of(&self, name: impl AsRef<str>) -> Option<usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1012).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1012`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the index of the field with the given name, or None if not found.

<a id="op-309ef2205dadcd0e25f1f4b8"></a>
## index_of_metadata_column

`function` · `buoyant_kernel::schema::StructType::index_of_metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn index_of_metadata_column(&self, spec: &MetadataColumnSpec) -> Option<&usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1017).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1017`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns the index of the metadata column with the given spec, or None if not found.

<a id="op-e4c353d1d13806a1aa21ebed"></a>
## into_fields

`function` · `buoyant_kernel::schema::StructType::into_fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_fields(self) -> impl ExactSizeIterator<Item = StructField> + DoubleEndedIterator + FusedIterator
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1135).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1135`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets an iterator over all the fields in this struct type.

<a id="op-c018cfb1a8ed02dcfaf99e50"></a>
## into_iter

`function` · `buoyant_kernel::schema::StructType::into_iter` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn into_iter(self) -> Self::IntoIter
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1392).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1388, 1], "end": [1397, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::iter::traits::collect::IntoIterator", "path": "IntoIterator"}, "trait_path": "core::iter::traits::collect::IntoIterator"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1392`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8af2610347c1ccf084875db"></a>
## leaves

`function` · `buoyant_kernel::schema::StructType::leaves` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn leaves<'s>(&self, own_name: impl Into<Option<&'s str>>) -> ColumnNamesAndTypes
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1248).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1248`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Extracts the name and type of all leaf columns, in schema order. Caller should pass Some
`own_name` if this schema is embedded in a larger struct (e.g. `add.*`) and None if the
schema is a top-level result (e.g. `*`).

NOTE: This method only traverses through `StructType` fields; `MapType` and `ArrayType`
fields are considered leaves even if they contain `StructType` entries/elements.

<a id="op-170d49e537dca68f8591d655"></a>
## make_physical

`function` · `buoyant_kernel::schema::StructType::make_physical` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn make_physical(&self, column_mapping_mode: ColumnMappingMode) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1261).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1261`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Applies physical name mappings to this field. If the `column_mapping_mode` is
[`ColumnMappingMode::Id`](../operations/buoyant_kernel.table_features.column_mapping.ColumnMappingMode.md#op-daa16a7e06e167fc1441630c), then each StructField will have its parquet field id in the
[`ColumnMetadataKey::ParquetFieldId`](../operations/buoyant_kernel.schema.ColumnMetadataKey.md#op-bb9afd4dbf92fc431ada321c) metadata field.

Uses a single transformer so duplicate column mapping IDs are detected across all
fields in this struct, not just within each field's subtree.

<a id="op-8c530ce09ea8a668f9621861"></a>
## metadata_column

`function` · `buoyant_kernel::schema::StructType::metadata_column` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata_column(&self, spec: &MetadataColumnSpec) -> Option<&StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1227).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1227`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets a reference to the metadata column with the given spec.

<a id="op-91ab2f5c70e8ff239de0ae43"></a>
## metadata_columns

`function` · `buoyant_kernel::schema::StructType::metadata_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn metadata_columns(&self) -> impl Iterator<Item = &StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1234).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1234`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets an iterator over all the metadata columns in this struct type.

<a id="op-4a358e7a62144c25e48670a6"></a>
## new_unchecked

`function` · `buoyant_kernel::schema::StructType::new_unchecked` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new_unchecked(fields: impl IntoIterator<Item = StructField>) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L958).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:958`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) from the given fields without validating them.

This should only be used when you are sure that the fields are valid.
Refer to [`StructType::try_new`](../operations/buoyant_kernel.schema.StructType.md#op-0f88c83ea0fcc1fa23e07ea0) for more details on the validation checks.

<a id="op-0cab973e3365d6ea34c37f09"></a>
## num_fields

`function` · `buoyant_kernel::schema::StructType::num_fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn num_fields(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1197).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1197`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets the number of fields in this struct type.

<a id="op-80bacf4e6d0667d9004132e9"></a>
## project

`function` · `buoyant_kernel::schema::StructType::project` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn project(&self, names: &[impl AsRef<str>]) -> DeltaResult<SchemaRef>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L992).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:992`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets a [`SchemaRef`](../operations/buoyant_kernel.schema.SchemaRef.md#op-bcbc708676b5e88d4183ee59) containing [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s of the given names. The order of fields in
the returned schema will match the order passed to this function, which can be different
from this order in this schema. Returns an Err if a specified field doesn't exist.

<a id="op-281bcbc9aeb12f2230b0eaae"></a>
## project_as_struct

`function` · `buoyant_kernel::schema::StructType::project_as_struct` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn project_as_struct(&self, names: &[impl AsRef<str>]) -> DeltaResult<StructType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L979).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:979`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Gets a [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) containing [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d)s of the given names. The order of fields in
the returned schema will match the order passed to this function, which can be different
from this order in this schema. Returns an Err if a specified field doesn't exist.

<a id="op-3e8b3ffd8fbf16fed2aadb4c"></a>
## serialize

`function` · `buoyant_kernel::schema::StructType::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1675).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1674, 1], "end": [1685, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1675`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a206e988bb24fa6e3c29f844"></a>
## total_struct_fields

`function` · `buoyant_kernel::schema::StructType::total_struct_fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn total_struct_fields(&self) -> usize
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1210).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1210`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Recursively counts all [`StructField`](../operations/buoyant_kernel.schema.StructField.md#op-ebbc3bdce7a671f7a7d1f58d) nodes in this schema tree.

This includes nested struct fields (inside Struct, Array, and Map types) but does not
count Array/Map containers themselves. This matches the traversal pattern used by
`assign_column_mapping_metadata` when assigning column IDs, so the result equals the
expected `delta.columnMapping.maxColumnId` for a newly created table.

<a id="op-1ab182b91d071d035e92dd4e"></a>
## try_from_arrow

`function` · `buoyant_kernel::schema::StructType::try_from_arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_arrow(arrow_schema: ArrowSchemaRef) -> Result<Self, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L424).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "crate::schema::StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [423, 1], "end": [427, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "buoyant_kernel::engine::arrow_conversion::TryFromArrow", "path": "TryFromArrow"}, "trait_path": "buoyant_kernel::engine::arrow_conversion::TryFromArrow"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:424`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-455d1e786e92bf9f26696dc3"></a>
## try_from_arrow

`function` · `buoyant_kernel::schema::StructType::try_from_arrow` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_arrow(arrow_schema: &ArrowSchema) -> Result<Self, ArrowError>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_conversion/mod.rs#L412).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "crate::schema::StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [411, 1], "end": [421, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "arrow_schema::schema::Schema", "path": "Schema"}}}}}], "constraints": []}}, "id": "buoyant_kernel::engine::arrow_conversion::TryFromArrow", "path": "TryFromArrow"}, "trait_path": "buoyant_kernel::engine::arrow_conversion::TryFromArrow"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_conversion/mod.rs:412`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82e4e697bd002ec4c7741f7b"></a>
## try_from_results

`function` · `buoyant_kernel::schema::StructType::try_from_results` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from_results<E: Into<Error>>(fields: impl IntoIterator<Item = Result<StructField, E>>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L940).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:940`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) from a fallible iterator of fields.

This constructor collects all fields from the iterator, returning the first error
encountered, or a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) if all fields are successfully collected and validated.

<a id="op-0f88c83ea0fcc1fa23e07ea0"></a>
## try_new

`function` · `buoyant_kernel::schema::StructType::try_new` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_new(fields: impl IntoIterator<Item = StructField>) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L895).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:895`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Creates a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) from the given fields.

Returns an error if:
- the schema contains duplicate field names (case-insensitive; Delta column names are
  case-insensitive per the protocol)
- the schema contains duplicate metadata columns
- the schema contains nested metadata columns

<a id="op-f629fcd8ef47133a8621998f"></a>
## visit_fields_of_path

`function` · `buoyant_kernel::schema::StructType::visit_fields_of_path` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn visit_fields_of_path<'a>(&'a self, col: &ColumnName, visit_field: impl FnMut(&'a StructField)) -> DeltaResult<()>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1051).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1051`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Visits all fields along the given column path.

Returns an error if the path is empty, a field is not found, or an intermediate field is not
a struct type.

<a id="op-b5a4ebf168bbca6b18e579f3"></a>
## with_fields_filtered

`function` · `buoyant_kernel::schema::StructType::with_fields_filtered` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_fields_filtered(&self, predicate: impl Fn(&StructField) -> bool) -> DeltaResult<Self>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1317).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1317`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns a new [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) containing only the top-level fields for which `predicate`
returns `true`. This does not recurse into nested [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) fields.

<a id="op-2c322118eac5e4c9c636a52b"></a>
## with_fields_filtered_nonempty

`function` · `buoyant_kernel::schema::StructType::with_fields_filtered_nonempty` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn with_fields_filtered_nonempty(&self, predicate: impl Fn(&StructField) -> bool) -> DeltaResult<Option<Self>>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1329).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [887, 1], "end": [1340, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1329`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns an optional [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) containing only the top-level fields for which
`predicate` returns `true`.

This is a convenience wrapper around [`StructType::with_fields_filtered`](../operations/buoyant_kernel.schema.StructType.md#op-b5a4ebf168bbca6b18e579f3) for callers
that treat an empty top-level struct as "no schema".

<a id="op-d706bbdce4301ad59b47dbf7"></a>
## can_read_as

`function` · `buoyant_kernel::schema::StructType::can_read_as` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_trait**. Canonical source location is not automatically a valid import path.

```rust
fn can_read_as(&self, read_type: &Self) -> Result<(), Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/compare.rs#L99).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::StructType", "path": "super::StructType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 1], "end": [136, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/compare.rs"}, "trait": {"args": null, "id": "buoyant_kernel::schema::compare::SchemaComparison", "path": "SchemaComparison"}, "trait_path": "buoyant_kernel::schema::compare::SchemaComparison"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/compare.rs:99`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Returns `Ok` if this [`StructType`](../operations/buoyant_kernel.schema.StructType.md#op-ac9d10e1b52f1ac2eacc4e98) can be read as `read_type`. This is the case when:
    1. The set of fields in this struct type are a subset of the `read_type`.
    2. For each field in this struct, you can read it as the `read_type`'s field. See
       [`StructField::can_read_as`](../operations/buoyant_kernel.schema.StructField.md#op-28524f651cbb400876a69029).
    3. If a field in `read_type` is not present in this struct, then it must be nullable.
    4. Both [`StructTypes`] must be valid schemas. No two fields of a struct may share a
       name that only differs by case.

<a id="op-25ece67406a4fe7f6559432f"></a>
## fields

`struct_field` · `buoyant_kernel::schema::StructType::fields` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
fields: indexmap::IndexMap<String, StructField>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L843).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:843`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The fields stored in this struct

<a id="op-1d1c640a46fce44fea0f3492"></a>
## metadata_columns

`struct_field` · `buoyant_kernel::schema::StructType::metadata_columns` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
metadata_columns: std::collections::HashMap<MetadataColumnSpec, usize>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L847).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:847`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The metadata columns in this struct

<a id="op-7022078f1081a1196df6a970"></a>
## type_name

`struct_field` · `buoyant_kernel::schema::StructType::type_name` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
type_name: String
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L838).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:838`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
