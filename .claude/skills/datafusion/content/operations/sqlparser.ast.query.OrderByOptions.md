# `sqlparser::ast::query::OrderByOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.OrderByOptions.json).

<a id="op-6cafe8e5559dfe89de3c6696"></a>
## OrderByOptions

`struct` · `sqlparser::ast::query::OrderByOptions` · sqlparser 0.62.0

```rust
struct OrderByOptions
```

Source: `src/ast/query.rs:3028`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for an `ORDER BY` expression (ASC/DESC and NULLS FIRST/LAST).

<a id="op-d9b2d9e8170953a413f7d0da"></a>
## asc

`struct_field` · `sqlparser::ast::query::OrderByOptions::asc` · sqlparser 0.62.0

```rust
asc: Option<bool>
```

Source: `src/ast/query.rs:3030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `ASC` (`Some(true)`) or `DESC` (`Some(false)`).

<a id="op-319727d841cfe3d0f9e77efc"></a>
## clone

`function` · `sqlparser::ast::query::OrderByOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OrderByOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 32], "end": [3024, 37], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-138cfb430f7933ab1505f0a3"></a>
## cmp

`function` · `sqlparser::ast::query::OrderByOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OrderByOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 66], "end": [3024, 69], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3329f1007e954d145d8422f6"></a>
## default

`function` · `sqlparser::ast::query::OrderByOptions::default` · sqlparser 0.62.0

```rust
fn default() -> OrderByOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 10], "end": [3024, 17], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aada99dcf6fbc02274379dcd"></a>
## deserialize

`function` · `sqlparser::ast::query::OrderByOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3025, 49], "end": [3025, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3025`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5f10e5e2a266944ad3c08f0"></a>
## eq

`function` · `sqlparser::ast::query::OrderByOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OrderByOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 39], "end": [3024, 48], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a07809c6801a7b24e2b9d96"></a>
## fmt

`function` · `sqlparser::ast::query::OrderByOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 19], "end": [3024, 24], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2f027779a05b7f1cfe5a4cd"></a>
## fmt

`function` · `sqlparser::ast::query::OrderByOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3035, 1], "end": [3049, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3036`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03c19049e6aa65e0ef1ff619"></a>
## hash

`function` · `sqlparser::ast::query::OrderByOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 71], "end": [3024, 75], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e3ff5c4694073fea278e0ae"></a>
## nulls_first

`struct_field` · `sqlparser::ast::query::OrderByOptions::nulls_first` · sqlparser 0.62.0

```rust
nulls_first: Option<bool>
```

Source: `src/ast/query.rs:3032`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional `NULLS FIRST` (`Some(true)`) or `NULLS LAST` (`Some(false)`).

<a id="op-a12a0f6346d172255743f3fd"></a>
## partial_cmp

`function` · `sqlparser::ast::query::OrderByOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OrderByOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3024, 50], "end": [3024, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c98a9bf2fc0abd2d297a32df"></a>
## serialize

`function` · `sqlparser::ast::query::OrderByOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3025, 38], "end": [3025, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3025`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abd87c21b61d9c07c8663cb9"></a>
## visit

`function` · `sqlparser::ast::query::OrderByOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3026, 47], "end": [3026, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da6f6262d9e53d39da53ab54"></a>
## visit

`function` · `sqlparser::ast::query::OrderByOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::OrderByOptions", "path": "OrderByOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3026, 40], "end": [3026, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3026`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
