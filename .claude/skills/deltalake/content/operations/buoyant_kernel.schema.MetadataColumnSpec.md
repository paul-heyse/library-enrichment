# `buoyant_kernel::schema::MetadataColumnSpec`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.MetadataColumnSpec.json).

<a id="op-2710452c41a584a4cd5350bb"></a>
## MetadataColumnSpec

`enum` · `buoyant_kernel::schema::MetadataColumnSpec` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
enum MetadataColumnSpec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L307).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:307`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Enumeration of metadata columns recognized by Delta Kernel.

Metadata columns provide additional information about rows in a Delta table.

<a id="op-39cc554c884c9e9c7e710765"></a>
## Err

`assoc_type` · `buoyant_kernel::schema::MetadataColumnSpec::Err` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
type Err = Error
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L355).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [366, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:355`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28ea87de44bb9ffbf8cf6f7b"></a>
## FilePath

`variant` · `buoyant_kernel::schema::MetadataColumnSpec::FilePath` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
FilePath
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L311).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:311`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d7f3e472376a5dc31180a05"></a>
## RowCommitVersion

`variant` · `buoyant_kernel::schema::MetadataColumnSpec::RowCommitVersion` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RowCommitVersion
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L310).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:310`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a017c624f8f38d9e05ecbb6"></a>
## RowId

`variant` · `buoyant_kernel::schema::MetadataColumnSpec::RowId` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RowId
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L309).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:309`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1427ded1f654fc5b4ab200ba"></a>
## RowIndex

`variant` · `buoyant_kernel::schema::MetadataColumnSpec::RowIndex` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
RowIndex
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L308).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:308`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adf243f2f02140f8c213a1ea"></a>
## clone

`function` · `buoyant_kernel::schema::MetadataColumnSpec::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> MetadataColumnSpec
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 38], "end": [306, 43], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-574c12316c29bb1d734cf6c8"></a>
## data_type

`function` · `buoyant_kernel::schema::MetadataColumnSpec::data_type` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn data_type(&self) -> DataType
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L326).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [352, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:326`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The data type of the specified metadata column.

<a id="op-7e572e37c190e3277217d453"></a>
## eq

`function` · `buoyant_kernel::schema::MetadataColumnSpec::eq` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn eq(&self, other: &MetadataColumnSpec) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 17], "end": [306, 26], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b3357edbd871ffe371ba5972"></a>
## fmt

`function` · `buoyant_kernel::schema::MetadataColumnSpec::fmt` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 10], "end": [306, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12383fb5c8b49543c90a7e68"></a>
## from_str

`function` · `buoyant_kernel::schema::MetadataColumnSpec::from_str` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L357).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [354, 1], "end": [366, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:357`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8205a6f8b6cfe298142ac169"></a>
## hash

`function` · `buoyant_kernel::schema::MetadataColumnSpec::hash` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L306).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 32], "end": [306, 36], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:306`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b984e3e246e49c7f30408a1"></a>
## nullable

`function` · `buoyant_kernel::schema::MetadataColumnSpec::nullable` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn nullable(&self) -> bool
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L336).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [352, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:336`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Whether the specified metadata column is nullable.

<a id="op-dc96290242e89842bce92a89"></a>
## reserved_field_id

`function` · `buoyant_kernel::schema::MetadataColumnSpec::reserved_field_id` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn reserved_field_id(&self) -> Option<i64>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L346).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [352, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:346`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

The reserved field ID for the specified metadata column, if any.

<a id="op-6dfe87ee238807698865c571"></a>
## text_value

`function` · `buoyant_kernel::schema::MetadataColumnSpec::text_value` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn text_value(&self) -> &'static str
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L316).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::MetadataColumnSpec", "path": "MetadataColumnSpec"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [314, 1], "end": [352, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:316`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

A human-readable name for the specified metadata column.
