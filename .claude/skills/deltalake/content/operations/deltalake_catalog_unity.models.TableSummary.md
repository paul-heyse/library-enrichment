# `deltalake_catalog_unity::models::TableSummary`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_catalog_unity.models.TableSummary.json).

<a id="op-d44e25719353cba6abaea29c"></a>
## TableSummary

`struct` · `deltalake_catalog_unity::models::TableSummary` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct TableSummary
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L331).

Source: `crates/catalog-unity/src/models.rs:331`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Summary of the table

<a id="op-085de3032245d7e18a25f288"></a>
## deserialize

`function` · `deltalake_catalog_unity::models::TableSummary::deserialize` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L329).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableSummary", "path": "TableSummary"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 10], "end": [329, 21], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `crates/catalog-unity/src/models.rs:329`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3b6f2537ddcf3c4403d7c70"></a>
## fmt

`function` · `deltalake_catalog_unity::models::TableSummary::fmt` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L329).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_catalog_unity::models::TableSummary", "path": "TableSummary"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [329, 23], "end": [329, 28], "filename": "crates/catalog-unity/src/models.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/catalog-unity/src/models.rs:329`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3b607ee2da67520edbc5080"></a>
## full_name

`struct_field` · `deltalake_catalog_unity::models::TableSummary::full_name` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
full_name: String
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L333).

Source: `crates/catalog-unity/src/models.rs:333`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

The full name of the table.

<a id="op-84053e92b67dc14c7e0e53f1"></a>
## table_type

`struct_field` · `deltalake_catalog_unity::models::TableSummary::table_type` · deltalake-catalog-unity 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
table_type: TableType
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/catalog-unity/src/models.rs#L335).

Source: `crates/catalog-unity/src/models.rs:335`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Type of table
