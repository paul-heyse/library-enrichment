# `sqlparser::ast::NullInclusion`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.NullInclusion.json).

<a id="op-6ee21fd0d6756eb24ef99a18"></a>
## NullInclusion

`enum` · `sqlparser::ast::NullInclusion` · sqlparser 0.62.0

```rust
enum NullInclusion
```

Source: `src/ast/mod.rs:11390`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies Include / Exclude NULL within UNPIVOT command.
For example
`UNPIVOT (column1 FOR new_column IN (col3, col4, col5, col6))`

<a id="op-3be0e7f558a21171ab7adf59"></a>
## ExcludeNulls

`variant` · `sqlparser::ast::NullInclusion::ExcludeNulls` · sqlparser 0.62.0

```rust
ExcludeNulls
```

Source: `src/ast/mod.rs:11394`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exclude NULL values from the UNPIVOT output.

<a id="op-1a42ecc61fed0260a3124e48"></a>
## IncludeNulls

`variant` · `sqlparser::ast::NullInclusion::IncludeNulls` · sqlparser 0.62.0

```rust
IncludeNulls
```

Source: `src/ast/mod.rs:11392`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Include NULL values in the UNPIVOT output.

<a id="op-3aca3043de6614d1af26dadf"></a>
## clone

`function` · `sqlparser::ast::NullInclusion::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> NullInclusion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 17], "end": [11387, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-825a05081a6d36cf600e6afd"></a>
## cmp

`function` · `sqlparser::ast::NullInclusion::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &NullInclusion) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 51], "end": [11387, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc61eefe579a10d644f1f9dc"></a>
## deserialize

`function` · `sqlparser::ast::NullInclusion::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11388, 49], "end": [11388, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-999d06024da2670d00fd31ab"></a>
## eq

`function` · `sqlparser::ast::NullInclusion::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &NullInclusion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 24], "end": [11387, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b66a9c40e2308644df62705"></a>
## fmt

`function` · `sqlparser::ast::NullInclusion::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 10], "end": [11387, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afa3f4e3eaadaa55b8081a69"></a>
## fmt

`function` · `sqlparser::ast::NullInclusion::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11397, 1], "end": [11404, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11398`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-783eb0d29af5ef98f7aaeae6"></a>
## hash

`function` · `sqlparser::ast::NullInclusion::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 56], "end": [11387, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-acd716a6e90d8d398e99e7c3"></a>
## partial_cmp

`function` · `sqlparser::ast::NullInclusion::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &NullInclusion) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11387, 35], "end": [11387, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11387`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19f8fdd3b5425847c7a659e0"></a>
## serialize

`function` · `sqlparser::ast::NullInclusion::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11388, 38], "end": [11388, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11388`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f936bd783a01c8747658c1f"></a>
## visit

`function` · `sqlparser::ast::NullInclusion::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11389, 47], "end": [11389, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f720b01137d9a2a160c9e2ab"></a>
## visit

`function` · `sqlparser::ast::NullInclusion::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::NullInclusion", "path": "NullInclusion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11389, 40], "end": [11389, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11389`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
