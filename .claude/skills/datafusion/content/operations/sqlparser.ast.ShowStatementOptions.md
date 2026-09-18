# `sqlparser::ast::ShowStatementOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ShowStatementOptions.json).

<a id="op-efffe37642350fb5127ca78a"></a>
## ShowStatementOptions

`struct` · `sqlparser::ast::ShowStatementOptions` · sqlparser 0.62.0

```rust
struct ShowStatementOptions
```

Source: `src/ast/mod.rs:10777`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the different options available for `SHOW`
statements to filter the results. Example from Snowflake:
<https://docs.snowflake.com/en/sql-reference/sql/show-tables>

<a id="op-5b0070690699edd4a901c22b"></a>
## clone

`function` · `sqlparser::ast::ShowStatementOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ShowStatementOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 17], "end": [10774, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18ccdce1d062df668d57fd7f"></a>
## cmp

`function` · `sqlparser::ast::ShowStatementOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ShowStatementOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 51], "end": [10774, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5ba27222ed7d0c3967d90b8"></a>
## deserialize

`function` · `sqlparser::ast::ShowStatementOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10775, 49], "end": [10775, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0b24e38572d96dee5b3fb9e"></a>
## eq

`function` · `sqlparser::ast::ShowStatementOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ShowStatementOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 24], "end": [10774, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a209d4fccbdbd29811cd77d4"></a>
## filter_position

`struct_field` · `sqlparser::ast::ShowStatementOptions::filter_position` · sqlparser 0.62.0

```rust
filter_position: Option<ShowStatementFilterPosition>
```

Source: `src/ast/mod.rs:10787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional filter position (infix or suffix) for `LIKE`/`FILTER`.

<a id="op-6640b59a422559344c68d13a"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10790, 1], "end": [10823, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10791`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66918d9a435d502685e71de8"></a>
## fmt

`function` · `sqlparser::ast::ShowStatementOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 10], "end": [10774, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca4fefed2b4d2ecb31ba5926"></a>
## hash

`function` · `sqlparser::ast::ShowStatementOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 56], "end": [10774, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c33e0f9d5f96ccde55ddd91b"></a>
## limit

`struct_field` · `sqlparser::ast::ShowStatementOptions::limit` · sqlparser 0.62.0

```rust
limit: Option<Expr>
```

Source: `src/ast/mod.rs:10783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `LIMIT` expression.

<a id="op-9f538f68dd7dce13b650ae7f"></a>
## limit_from

`struct_field` · `sqlparser::ast::ShowStatementOptions::limit_from` · sqlparser 0.62.0

```rust
limit_from: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:10785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `FROM` value used with `LIMIT`.

<a id="op-0f4c09e5c44c6743359418ae"></a>
## partial_cmp

`function` · `sqlparser::ast::ShowStatementOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ShowStatementOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10774, 35], "end": [10774, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10774`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45943c5c3aa51e0177bf1ed6"></a>
## serialize

`function` · `sqlparser::ast::ShowStatementOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10775, 38], "end": [10775, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10775`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06093ae27717015ac2ffcac0"></a>
## show_in

`struct_field` · `sqlparser::ast::ShowStatementOptions::show_in` · sqlparser 0.62.0

```rust
show_in: Option<ShowStatementIn>
```

Source: `src/ast/mod.rs:10779`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional scope to show in (for example: TABLE, SCHEMA).

<a id="op-ab439023890919609139fb4e"></a>
## starts_with

`struct_field` · `sqlparser::ast::ShowStatementOptions::starts_with` · sqlparser 0.62.0

```rust
starts_with: Option<ValueWithSpan>
```

Source: `src/ast/mod.rs:10781`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `STARTS WITH` filter value.

<a id="op-3e8d61e4e70606d1564b798d"></a>
## visit

`function` · `sqlparser::ast::ShowStatementOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10776, 40], "end": [10776, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e55373dfbac3596c4504a17"></a>
## visit

`function` · `sqlparser::ast::ShowStatementOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ShowStatementOptions", "path": "ShowStatementOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10776, 47], "end": [10776, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10776`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
