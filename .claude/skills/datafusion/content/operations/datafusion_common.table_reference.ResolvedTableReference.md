# `datafusion_common::table_reference::ResolvedTableReference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.table_reference.ResolvedTableReference.json).

<a id="op-8e17918e7e89b471267a2243"></a>
## ResolvedTableReference

`struct` · `datafusion_common::table_reference::ResolvedTableReference` · datafusion-common 55.1.0

```rust
struct ResolvedTableReference
```

Source: `src/table_reference.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A fully resolved path to a table of the form "catalog.schema.table"

<a id="op-48a36a6ac58cd64b69a957b0"></a>
## catalog

`struct_field` · `datafusion_common::table_reference::ResolvedTableReference::catalog` · datafusion-common 55.1.0

```rust
catalog: std::sync::Arc<str>
```

Source: `src/table_reference.rs:26`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The catalog (aka database) containing the table

<a id="op-48b3818127dccd2db41bf8fb"></a>
## clone

`function` · `datafusion_common::table_reference::ResolvedTableReference::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ResolvedTableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 17], "end": [23, 22], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c9032310ef89e75e0fd7c92"></a>
## cmp

`function` · `datafusion_common::table_reference::ResolvedTableReference::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &ResolvedTableReference) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 57], "end": [23, 60], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9eac81e68a72887ece8e3116"></a>
## eq

`function` · `datafusion_common::table_reference::ResolvedTableReference::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ResolvedTableReference) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 24], "end": [23, 33], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f2de1fdde743c9a1ebcef1a"></a>
## fmt

`function` · `datafusion_common::table_reference::ResolvedTableReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 10], "end": [23, 15], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88cbba15ddd1738bea44298c"></a>
## fmt

`function` · `datafusion_common::table_reference::ResolvedTableReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [33, 1], "end": [37, 2], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/table_reference.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e40ce91740d7f7841a5e68a"></a>
## hash

`function` · `datafusion_common::table_reference::ResolvedTableReference::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 39], "end": [23, 43], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6dad07911205f2f5c121b42"></a>
## partial_cmp

`function` · `datafusion_common::table_reference::ResolvedTableReference::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &ResolvedTableReference) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [23, 45], "end": [23, 55], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/table_reference.rs:23`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-630cb2abd6ca858b2d856fd0"></a>
## schema

`struct_field` · `datafusion_common::table_reference::ResolvedTableReference::schema` · datafusion-common 55.1.0

```rust
schema: std::sync::Arc<str>
```

Source: `src/table_reference.rs:28`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The schema containing the table

<a id="op-41e1f3ea0b72a2568f20da5e"></a>
## table

`struct_field` · `datafusion_common::table_reference::ResolvedTableReference::table` · datafusion-common 55.1.0

```rust
table: std::sync::Arc<str>
```

Source: `src/table_reference.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The table name
