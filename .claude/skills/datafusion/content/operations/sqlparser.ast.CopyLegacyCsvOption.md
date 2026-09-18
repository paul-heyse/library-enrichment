# `sqlparser::ast::CopyLegacyCsvOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CopyLegacyCsvOption.json).

<a id="op-febe1276553294cc4befdddf"></a>
## CopyLegacyCsvOption

`enum` · `sqlparser::ast::CopyLegacyCsvOption` · sqlparser 0.62.0

```rust
enum CopyLegacyCsvOption
```

Source: `src/ast/mod.rs:9675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `CSV` option in `COPY` statement before PostgreSQL version 9.0.

<https://www.postgresql.org/docs/8.4/sql-copy.html>

<a id="op-6f77aa75d0d5387918cdd1ec"></a>
## Escape

`variant` · `sqlparser::ast::CopyLegacyCsvOption::Escape` · sqlparser 0.62.0

```rust
Escape
```

Source: `src/ast/mod.rs:9681`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ESCAPE \[ AS \] 'escape_character'

<a id="op-9c42bed85e9fff66bf94b096"></a>
## ForceNotNull

`variant` · `sqlparser::ast::CopyLegacyCsvOption::ForceNotNull` · sqlparser 0.62.0

```rust
ForceNotNull
```

Source: `src/ast/mod.rs:9685`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORCE NOT NULL column_name [, ...]

<a id="op-e2fb8f206e8e689c172fa580"></a>
## ForceQuote

`variant` · `sqlparser::ast::CopyLegacyCsvOption::ForceQuote` · sqlparser 0.62.0

```rust
ForceQuote
```

Source: `src/ast/mod.rs:9683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORCE QUOTE { column_name [, ...] | * }

<a id="op-817bca03095765c89b124f92"></a>
## Header

`variant` · `sqlparser::ast::CopyLegacyCsvOption::Header` · sqlparser 0.62.0

```rust
Header
```

Source: `src/ast/mod.rs:9677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

HEADER

<a id="op-a6e1472ffa370e64ecea7039"></a>
## Quote

`variant` · `sqlparser::ast::CopyLegacyCsvOption::Quote` · sqlparser 0.62.0

```rust
Quote
```

Source: `src/ast/mod.rs:9679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

QUOTE \[ AS \] 'quote_character'

<a id="op-418d50651296b6dccf37ac4f"></a>
## clone

`function` · `sqlparser::ast::CopyLegacyCsvOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CopyLegacyCsvOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 17], "end": [9672, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4bde36a663f66d2f8dc98bc"></a>
## cmp

`function` · `sqlparser::ast::CopyLegacyCsvOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CopyLegacyCsvOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 51], "end": [9672, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1d394cd910c0e435ff8e612"></a>
## deserialize

`function` · `sqlparser::ast::CopyLegacyCsvOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9673, 49], "end": [9673, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef542d6ee002c8ea3de83173"></a>
## eq

`function` · `sqlparser::ast::CopyLegacyCsvOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CopyLegacyCsvOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 24], "end": [9672, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d101448585bc7be60a7453e"></a>
## fmt

`function` · `sqlparser::ast::CopyLegacyCsvOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 10], "end": [9672, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dda9fce20a1be6e837ec638e"></a>
## fmt

`function` · `sqlparser::ast::CopyLegacyCsvOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9688, 1], "end": [9701, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9689`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59706d0d7c6aa524724f1deb"></a>
## hash

`function` · `sqlparser::ast::CopyLegacyCsvOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 56], "end": [9672, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddbb070cfd22b744ed67d59b"></a>
## partial_cmp

`function` · `sqlparser::ast::CopyLegacyCsvOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CopyLegacyCsvOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9672, 35], "end": [9672, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9672`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5aa971880401f6f6acbf0d84"></a>
## serialize

`function` · `sqlparser::ast::CopyLegacyCsvOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9673, 38], "end": [9673, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3456f1727a6233a765c10458"></a>
## visit

`function` · `sqlparser::ast::CopyLegacyCsvOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9674, 40], "end": [9674, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fe1104d2b3d5de70b955a89"></a>
## visit

`function` · `sqlparser::ast::CopyLegacyCsvOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CopyLegacyCsvOption", "path": "CopyLegacyCsvOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9674, 47], "end": [9674, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9674`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
