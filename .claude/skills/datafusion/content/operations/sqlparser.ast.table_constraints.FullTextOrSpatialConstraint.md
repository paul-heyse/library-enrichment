# `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.FullTextOrSpatialConstraint.json).

<a id="op-0dac9d8c5b37b2ff6ce16576"></a>
## FullTextOrSpatialConstraint

`struct` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint` · sqlparser 0.62.0

```rust
struct FullTextOrSpatialConstraint
```

Source: `src/ast/table_constraints.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQLs [fulltext][1] definition. Since the [`SPATIAL`][2] definition is exactly the same,
and MySQL displays both the same way, it is part of this definition as well.

Supported syntax:

```markdown
{FULLTEXT | SPATIAL} [INDEX | KEY] [index_name] (key_part,...)

key_part: col_name
```

[1]: https://dev.mysql.com/doc/refman/8.0/en/fulltext-natural-language.html
[2]: https://dev.mysql.com/doc/refman/8.0/en/spatial-types.html

<a id="op-9236acbeacd7dd04070d43df"></a>
## clone

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FullTextOrSpatialConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 17], "end": [306, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-394ac1105c5c6e261f5b6423"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FullTextOrSpatialConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 51], "end": [306, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb7921e05e1448181d702b35"></a>
## columns

`struct_field` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::columns` · sqlparser 0.62.0

```rust
columns: Vec<ast::IndexColumn>
```

Source: `src/ast/table_constraints.rs:317`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Referred column identifier list.

<a id="op-fcac6409be5f93b0391ecba2"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 49], "end": [307, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-616fc72fcd8cdecb7b5ecc2b"></a>
## eq

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FullTextOrSpatialConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 24], "end": [306, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-84a0a3d9bd458e44ec17ef17"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 10], "end": [306, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2095f0db67d01b4f2f711d4"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [320, 1], "end": [338, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3eb49fbcc57b47187cadf38"></a>
## fulltext

`struct_field` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::fulltext` · sqlparser 0.62.0

```rust
fulltext: bool
```

Source: `src/ast/table_constraints.rs:311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this is a `FULLTEXT` (true) or `SPATIAL` (false) definition.

<a id="op-cbda6e28ccbb689ecde6a7cb"></a>
## hash

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 56], "end": [306, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce545e544bd4af45600fd847"></a>
## index_type_display

`struct_field` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::index_type_display` · sqlparser 0.62.0

```rust
index_type_display: ast::KeyOrIndexDisplay
```

Source: `src/ast/table_constraints.rs:313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the type is followed by the keyword `KEY`, `INDEX`, or no keyword at all.

<a id="op-830eb4f208c233f9dbbc1e02"></a>
## opt_index_name

`struct_field` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::opt_index_name` · sqlparser 0.62.0

```rust
opt_index_name: Option<ast::Ident>
```

Source: `src/ast/table_constraints.rs:315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional index name.

<a id="op-eb2e9c89374aeaa745289d78"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FullTextOrSpatialConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [306, 35], "end": [306, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9cbd2d4330d488149c69621"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [307, 38], "end": [307, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee164b0ece2ff596f5156416"></a>
## span

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 1], "end": [353, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/table_constraints.rs:341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69753782ac6863e06cfebea8"></a>
## visit

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 40], "end": [308, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5885c830497726c0738d77e"></a>
## visit

`function` · `sqlparser::ast::table_constraints::FullTextOrSpatialConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 47], "end": [308, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
