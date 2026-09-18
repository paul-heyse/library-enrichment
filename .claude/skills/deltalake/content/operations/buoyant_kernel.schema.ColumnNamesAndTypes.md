# `buoyant_kernel::schema::ColumnNamesAndTypes`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.schema.ColumnNamesAndTypes.json).

<a id="op-c9d304be3542d196b4bc5814"></a>
## ColumnNamesAndTypes

`struct` · `buoyant_kernel::schema::ColumnNamesAndTypes` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ColumnNamesAndTypes
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1652).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1652`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

Helper for RowVisitor implementations

<a id="op-d40ec7fb9b114d4faecbb606"></a>
## as_ref

`function` · `buoyant_kernel::schema::ColumnNamesAndTypes::as_ref` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn as_ref(&self) -> (&[ColumnName], &[DataType])
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1655).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::ColumnNamesAndTypes", "path": "ColumnNamesAndTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1653, 1], "end": [1658, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": null, "trait_path": null}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1655`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-145035358fa11aae67db6c92"></a>
## clone

`function` · `buoyant_kernel::schema::ColumnNamesAndTypes::clone` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ColumnNamesAndTypes
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1651).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::ColumnNamesAndTypes", "path": "ColumnNamesAndTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1651, 10], "end": [1651, 15], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1651`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0f6ace1bdfa3da06f55fc11"></a>
## default

`function` · `buoyant_kernel::schema::ColumnNamesAndTypes::default` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ColumnNamesAndTypes
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1651).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::ColumnNamesAndTypes", "path": "ColumnNamesAndTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1651, 17], "end": [1651, 24], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1651`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a5a1cd8650356a496f03fd5c"></a>
## from

`function` · `buoyant_kernel::schema::ColumnNamesAndTypes::from` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from((names, fields): (Vec<ColumnName>, Vec<DataType>)) -> Self
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1661).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::ColumnNamesAndTypes", "path": "ColumnNamesAndTypes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1660, 1], "end": [1664, 2], "filename": "/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"tuple": [{"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::expressions::column_names::ColumnName", "path": "ColumnName"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}, {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "buoyant_kernel::schema::DataType", "path": "DataType"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}]}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1661`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd2e2bf43bd165ab9fda73d4"></a>
## 0

`struct_field` · `buoyant_kernel::schema::ColumnNamesAndTypes::0` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
0: Vec<expressions::ColumnName>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1652).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1652`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bb1fc7b5a71b3af0770a344"></a>
## 1

`struct_field` · `buoyant_kernel::schema::ColumnNamesAndTypes::1` · buoyant_kernel 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
1: Vec<DataType>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/schema/mod.rs#L1652).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/schema/mod.rs:1652`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

No upstream documentation on this item; consult its owner/trait contract.
