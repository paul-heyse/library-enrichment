# `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_data_loading.StageLoadSelectItemKind.json).

<a id="op-619d938e7ac7050d0fc88f76"></a>
## StageLoadSelectItemKind

`enum` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind` · sqlparser 0.62.0

```rust
enum StageLoadSelectItemKind
```

Source: `src/ast/helpers/stmt_data_loading.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This enum enables support for both standard SQL select item expressions
and Snowflake-specific ones for data loading.

<a id="op-35d0307d0c33afb0b1e7eb5b"></a>
## SelectItem

`variant` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::SelectItem` · sqlparser 0.62.0

```rust
SelectItem
```

Source: `src/ast/helpers/stmt_data_loading.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A standard SQL select item expression.

<a id="op-4150bbf783560628fdacd145"></a>
## StageLoadSelectItem

`variant` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::StageLoadSelectItem` · sqlparser 0.62.0

```rust
StageLoadSelectItem
```

Source: `src/ast/helpers/stmt_data_loading.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A Snowflake-specific select item used for stage loading.

<a id="op-3065b9f42df2e864df46e29d"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StageLoadSelectItemKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 17], "end": [53, 22], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c451e44fa5f04b73aeacfe5f"></a>
## cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StageLoadSelectItemKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 51], "end": [53, 54], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a18773adbdaa41e094d268a0"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 49], "end": [54, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bff600930f90233be9a0c347"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StageLoadSelectItemKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 24], "end": [53, 33], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f62de96eab908d9f0a75d05"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 10], "end": [53, 15], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98390695e48d6544ad693668"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [70, 2], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/stmt_data_loading.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55bfd44c4ca5e5803988f488"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 56], "end": [53, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-558ac7b87de25484e73d060d"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StageLoadSelectItemKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 39], "end": [53, 49], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/stmt_data_loading.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26dd634b56b9b5a39c47fa69"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 38], "end": [54, 47], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ce9de6660ed810448605599"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 40], "end": [55, 45], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_data_loading.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb8a7f0b094eec6befd5cfc1"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind", "path": "StageLoadSelectItemKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 47], "end": [55, 55], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_data_loading.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
