# `sqlparser::ast::ddl::ConstraintCharacteristics`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ConstraintCharacteristics.json).

<a id="op-40c07c7dafae25a20655ef82"></a>
## ConstraintCharacteristics

`struct` · `sqlparser::ast::ddl::ConstraintCharacteristics` · sqlparser 0.62.0

```rust
struct ConstraintCharacteristics
```

Source: `src/ast/ddl.rs:2233`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`<constraint_characteristics> = [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ] [ ENFORCED | NOT ENFORCED ]`

Used in UNIQUE and foreign key constraints. The individual settings may occur in any order.

<a id="op-31f7e14478dfa22dc0c60378"></a>
## clone

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ConstraintCharacteristics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 23], "end": [2230, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c647a8bfabaf97645a6c9153"></a>
## cmp

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ConstraintCharacteristics) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 66], "end": [2230, 69], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f69bcc890df5bc99c8202b9"></a>
## default

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::default` · sqlparser 0.62.0

```rust
fn default() -> ConstraintCharacteristics
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 53], "end": [2230, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9c136bebd79bff4b6c46fc5"></a>
## deferrable

`struct_field` · `sqlparser::ast::ddl::ConstraintCharacteristics::deferrable` · sqlparser 0.62.0

```rust
deferrable: Option<bool>
```

Source: `src/ast/ddl.rs:2235`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ DEFERRABLE | NOT DEFERRABLE ]`

<a id="op-c17f8202541a11bab3f5e0fe"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2231, 49], "end": [2231, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4cd51e695f7ac010398461a"></a>
## enforced

`struct_field` · `sqlparser::ast::ddl::ConstraintCharacteristics::enforced` · sqlparser 0.62.0

```rust
enforced: Option<bool>
```

Source: `src/ast/ddl.rs:2239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ ENFORCED | NOT ENFORCED ]`

<a id="op-375aeeab4d4bd80cbeb26229"></a>
## eq

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ConstraintCharacteristics) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 30], "end": [2230, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-893d06547a3e28cc61e6d9bc"></a>
## fmt

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2285, 1], "end": [2304, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2286`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77ede8ad2227f4bfc218aa9"></a>
## fmt

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 10], "end": [2230, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc81a1e2ff2606eaeebe4a1"></a>
## hash

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 71], "end": [2230, 75], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-767b6c8ec189d5643eb9d648"></a>
## initially

`struct_field` · `sqlparser::ast::ddl::ConstraintCharacteristics::initially` · sqlparser 0.62.0

```rust
initially: Option<DeferrableInitial>
```

Source: `src/ast/ddl.rs:2237`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`

<a id="op-37ac3b76b4ef27e1aa87c399"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ConstraintCharacteristics) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2230, 41], "end": [2230, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2230`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-042f104366cddb338efce88d"></a>
## serialize

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2231, 38], "end": [2231, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2231`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c0aaec26da7e41d67370b0d"></a>
## span

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "super::ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [855, 1], "end": [865, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c395cd8b63324068ee02fad"></a>
## visit

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2232, 47], "end": [2232, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e167dbc0fc408c425e46fa80"></a>
## visit

`function` · `sqlparser::ast::ddl::ConstraintCharacteristics::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ConstraintCharacteristics", "path": "ConstraintCharacteristics"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2232, 40], "end": [2232, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2232`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
