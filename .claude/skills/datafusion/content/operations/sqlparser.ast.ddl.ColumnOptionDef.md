# `sqlparser::ast::ddl::ColumnOptionDef`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnOptionDef.json).

<a id="op-0952e690c40868eb83826a9e"></a>
## ColumnOptionDef

`struct` · `sqlparser::ast::ddl::ColumnOptionDef` · sqlparser 0.62.0

```rust
struct ColumnOptionDef
```

Source: `src/ast/ddl.rs:1644`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An optionally-named `ColumnOption`: `[ CONSTRAINT <name> ] <column-option>`.

Note that implementations are substantially more permissive than the ANSI
specification on what order column options can be presented in, and whether
they are allowed to be named. The specification distinguishes between
constraints (NOT NULL, UNIQUE, PRIMARY KEY, and CHECK), which can be named
and can appear in any order, and other options (DEFAULT, GENERATED), which
cannot be named and must appear in a fixed order. `PostgreSQL`, however,
allows preceding any option with `CONSTRAINT <name>`, even those that are
not really constraints, like NULL and DEFAULT. MSSQL is less permissive,
allowing DEFAULT, UNIQUE, PRIMARY KEY and CHECK to be named, but not NULL or
NOT NULL constraints (the last of which is in violation of the spec).

For maximum flexibility, we don't distinguish between constraint and
non-constraint options, lumping them all together under the umbrella of
"column options," and we allow any column option to be named.

<a id="op-2322c0c90b9ebfe789adbcaf"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnOptionDef::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnOptionDef
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 17], "end": [1641, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-339fcc2572157b239d09526f"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnOptionDef::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnOptionDef) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 51], "end": [1641, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0948bab02226744b4fb8ad9a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnOptionDef::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 49], "end": [1642, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1642`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77303058f89f7bdea6c1f2dc"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnOptionDef::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnOptionDef) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 24], "end": [1641, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a48da8b9d1e6694a8569154"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnOptionDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 10], "end": [1641, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbad0709dff26c59f975eb44"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnOptionDef::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1651, 1], "end": [1655, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2ca9d14fb465ead03b16eec"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnOptionDef::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 56], "end": [1641, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be10cc369b1591546f90d3b4"></a>
## name

`struct_field` · `sqlparser::ast::ddl::ColumnOptionDef::name` · sqlparser 0.62.0

```rust
name: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:1646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional name of the constraint.

<a id="op-f631bbd36765b77392d214cf"></a>
## option

`struct_field` · `sqlparser::ast::ddl::ColumnOptionDef::option` · sqlparser 0.62.0

```rust
option: ColumnOption
```

Source: `src/ast/ddl.rs:1648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The actual column option (e.g. `NOT NULL`, `DEFAULT`, `GENERATED`, ...).

<a id="op-8b53dd8ce2e2aabae56e1823"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnOptionDef::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnOptionDef) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1641, 35], "end": [1641, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2faf6d64254f9d89a8cde94"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnOptionDef::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1642, 38], "end": [1642, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1642`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02c1ae251064823bc679ecfd"></a>
## span

`function` · `sqlparser::ast::ddl::ColumnOptionDef::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "super::ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [634, 1], "end": [640, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:635`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a2ba329cac191278fbc3e5b"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOptionDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1643, 40], "end": [1643, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33e7d6849a035f783fcd024a"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnOptionDef::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnOptionDef", "path": "ColumnOptionDef"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1643, 47], "end": [1643, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
