# `buoyant_kernel::table_features::column_mapping::ColumnMappingMode`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.table_features.column_mapping.ColumnMappingMode.json).

<a id="op-757f64913cb951ae565079da"></a>
## ColumnMappingMode

`enum` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum ColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L26).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:26`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Modes of column mapping a table can be in

<a id="op-21b834597e386252f066a315"></a>
## Err

`assoc_type` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::Err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6240a5d110bd8608ce7890b2"></a>
## Error

`assoc_type` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::Error` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Error = ParseError
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-daa16a7e06e167fc1441630c"></a>
## Id

`variant` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::Id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Id
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L30).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:30`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Columns are mapped by their field_id in parquet

<a id="op-9c15dcff4d43480fcd397e96"></a>
## Name

`variant` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::Name` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
Name
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L32).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:32`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Columns are mapped to a physical name

<a id="op-86c2a30a801346d0bff7b13e"></a>
## None

`variant` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::None` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
None
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L28).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:28`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No column mapping is applied

<a id="op-e1f232a69d7f03c438807203"></a>
## clone

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ColumnMappingMode
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 59], "end": [23, 64], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b8540b5194b575208da37ce"></a>
## deserialize

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::deserialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 40], "end": [23, 51], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bbf38c3ea00efb1c41f30580"></a>
## eq

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &ColumnMappingMode) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 66], "end": [23, 75], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0acfe6dc94621fff6e339ff4"></a>
## fmt

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a9d40e962ac25df7981bf70"></a>
## from_str

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::from_str` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> ::core::result::Result<ColumnMappingMode, <Self as ::core::str::FromStr>::Err>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fdeb0eaa4fd1b4733e1da4b"></a>
## serialize

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::serialize` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 29], "end": [23, 38], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b13b3a708445ac13c8e81144"></a>
## try_from

`function` · `buoyant_kernel::table_features::column_mapping::ColumnMappingMode::try_from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn try_from(s: &str) -> ::core::result::Result<ColumnMappingMode, <Self as ::core::convert::TryFrom>::Error>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/table_features/column_mapping.rs#L23).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::table_features::column_mapping::ColumnMappingMode", "path": "ColumnMappingMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 27], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::TryFrom", "path": "TryFrom"}, "trait_path": "core::convert::TryFrom"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/table_features/column_mapping.rs:23`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
