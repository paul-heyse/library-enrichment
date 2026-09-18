# `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_data_loading.StageLoadSelectItem.json).

<a id="op-c454cc75ae8a2739b5d22045"></a>
## StageLoadSelectItem

`struct` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem` · sqlparser 0.62.0

```rust
struct StageLoadSelectItem
```

Source: `src/ast/helpers/stmt_data_loading.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single item in the `SELECT` list for data loading from staged files.

<a id="op-221eca4209759215e1e597f1"></a>
## alias

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::alias` · sqlparser 0.62.0

```rust
alias: Option<ast::Ident>
```

Source: `src/ast/helpers/stmt_data_loading.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the input source.

<a id="op-821839237eb0f5fe62327733"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> StageLoadSelectItem
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 22], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3409bf2eb22f172eaec40395"></a>
## cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &StageLoadSelectItem) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 51], "end": [72, 54], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-582b8c3170d064caf266066b"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 49], "end": [73, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-661fa933018a98ef90c464cc"></a>
## element

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::element` · sqlparser 0.62.0

```rust
element: Option<ast::Ident>
```

Source: `src/ast/helpers/stmt_data_loading.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional element identifier following the column reference.

<a id="op-e232f1af45f48253f817bda8"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &StageLoadSelectItem) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 24], "end": [72, 33], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0347e84116aa651a3e7a280"></a>
## file_col_num

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::file_col_num` · sqlparser 0.62.0

```rust
file_col_num: i32
```

Source: `src/ast/helpers/stmt_data_loading.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column number within the staged file (1-based).

<a id="op-26999b1539322ed6f6b05c72"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ba1067c24a10dcf6a99301f"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [127, 2], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/stmt_data_loading.rs:114`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-221d98807e8a545773c4e9cb"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 56], "end": [72, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2ae1865bddbca02d646987b"></a>
## item_as

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::item_as` · sqlparser 0.62.0

```rust
item_as: Option<ast::Ident>
```

Source: `src/ast/helpers/stmt_data_loading.rs:84`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional alias for the item (AS clause).

<a id="op-071841f701f26c6acb0bf706"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &StageLoadSelectItem) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 39], "end": [72, 49], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/stmt_data_loading.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac811f2bc25d3586a2123bda"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 38], "end": [73, 47], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70974e70b89688b2efc1646a"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 47], "end": [74, 55], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_data_loading.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6dc4bdb584ff3849f7681c6"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItem", "path": "StageLoadSelectItem"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 40], "end": [74, 45], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_data_loading.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
