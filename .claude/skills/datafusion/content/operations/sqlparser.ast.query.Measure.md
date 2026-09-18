# `sqlparser::ast::query::Measure`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Measure.json).

<a id="op-fbda8a840995cb1bfcfbf935"></a>
## Measure

`struct` · `sqlparser::ast::query::Measure` · sqlparser 0.62.0

```rust
struct Measure
```

Source: `src/ast/query.rs:1989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An item in the `MEASURES` subclause of a `MATCH_RECOGNIZE` operation.

See <https://docs.snowflake.com/en/sql-reference/constructs/match_recognize#measures-specifying-additional-output-columns>.
An item in the `MEASURES` clause of `MATCH_RECOGNIZE`.

<a id="op-656cc47b43492c9d87a5e6fb"></a>
## alias

`struct_field` · `sqlparser::ast::query::Measure::alias` · sqlparser 0.62.0

```rust
alias: Ident
```

Source: `src/ast/query.rs:1993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alias for the measure column.

<a id="op-bfb1eff90351312dbd2f6ae0"></a>
## clone

`function` · `sqlparser::ast::query::Measure::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Measure
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 17], "end": [1985, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d098a149ebe535a7dded77a"></a>
## cmp

`function` · `sqlparser::ast::query::Measure::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Measure) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 51], "end": [1985, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03e564e0cf4d61f849f345e9"></a>
## deserialize

`function` · `sqlparser::ast::query::Measure::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 49], "end": [1986, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3161ccfcb14f3b1d474d99c1"></a>
## eq

`function` · `sqlparser::ast::query::Measure::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Measure) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 24], "end": [1985, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09c4633c2ec2e2a39df4f958"></a>
## expr

`struct_field` · `sqlparser::ast::query::Measure::expr` · sqlparser 0.62.0

```rust
expr: Expr
```

Source: `src/ast/query.rs:1991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Expression producing the measure value.

<a id="op-02be15a37478d32b13401acc"></a>
## fmt

`function` · `sqlparser::ast::query::Measure::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 10], "end": [1985, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-934f53585d6eaa2c476312fb"></a>
## fmt

`function` · `sqlparser::ast::query::Measure::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1996, 1], "end": [2000, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:1997`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb888924c7630a352198d8c"></a>
## hash

`function` · `sqlparser::ast::query::Measure::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 56], "end": [1985, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-039ec8ee2e41827e0aeeb824"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Measure::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Measure) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1985, 35], "end": [1985, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ebc96bbab5308c9865867fb"></a>
## serialize

`function` · `sqlparser::ast::query::Measure::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1986, 38], "end": [1986, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1986`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f6de7ea7f540e540320ec34"></a>
## span

`function` · `sqlparser::ast::query::Measure::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "super::Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2114, 1], "end": [2120, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2115`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab1fb607c3f39b0009e2dca5"></a>
## visit

`function` · `sqlparser::ast::query::Measure::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1987, 40], "end": [1987, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8383b776d628fec47c4ecf2"></a>
## visit

`function` · `sqlparser::ast::query::Measure::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Measure", "path": "Measure"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1987, 47], "end": [1987, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
