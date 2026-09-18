# `datafusion_physical_plan::joins::utils::ColumnIndex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.joins.utils.ColumnIndex.json).

<a id="op-f0ce6201e6c4d1246816afde"></a>
## ColumnIndex

`struct` · `datafusion_physical_plan::joins::utils::ColumnIndex` · datafusion-physical-plan 55.1.0

```rust
struct ColumnIndex
```

Source: `src/joins/utils.rs:234`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Information about the index and placement (left or right) of the columns

<a id="op-f5906da31fadbd96d8304782"></a>
## clone

`function` · `datafusion_physical_plan::joins::utils::ColumnIndex::clone` · datafusion-physical-plan 55.1.0

```rust
fn clone(&self) -> ColumnIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 17], "end": [233, 22], "filename": "src/joins/utils.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/joins/utils.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cd6b7782c34a5eedf979876"></a>
## eq

`function` · `datafusion_physical_plan::joins::utils::ColumnIndex::eq` · datafusion-physical-plan 55.1.0

```rust
fn eq(&self, other: &ColumnIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 24], "end": [233, 33], "filename": "src/joins/utils.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/joins/utils.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-804f747cf47e08bff5d2907d"></a>
## fmt

`function` · `datafusion_physical_plan::joins::utils::ColumnIndex::fmt` · datafusion-physical-plan 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_plan::joins::utils::ColumnIndex", "path": "ColumnIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 10], "end": [233, 15], "filename": "src/joins/utils.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/joins/utils.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e88df650c74ca217ca799444"></a>
## index

`struct_field` · `datafusion_physical_plan::joins::utils::ColumnIndex::index` · datafusion-physical-plan 55.1.0

```rust
index: usize
```

Source: `src/joins/utils.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Index of the column

<a id="op-616e6543e16b92e8483f7ee2"></a>
## side

`struct_field` · `datafusion_physical_plan::joins::utils::ColumnIndex::side` · datafusion-physical-plan 55.1.0

```rust
side: datafusion_common::JoinSide
```

Source: `src/joins/utils.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Whether the column is at the left or right side
