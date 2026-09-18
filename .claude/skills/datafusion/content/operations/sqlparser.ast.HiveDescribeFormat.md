# `sqlparser::ast::HiveDescribeFormat`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.HiveDescribeFormat.json).

<a id="op-314529416ce71b800bfdc3da"></a>
## HiveDescribeFormat

`enum` · `sqlparser::ast::HiveDescribeFormat` · sqlparser 0.62.0

```rust
enum HiveDescribeFormat
```

Source: `src/ast/mod.rs:8641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Describe output format options for Hive `DESCRIBE`/`EXPLAIN`.

<a id="op-df4541fb92f2232726baae8b"></a>
## Extended

`variant` · `sqlparser::ast::HiveDescribeFormat::Extended` · sqlparser 0.62.0

```rust
Extended
```

Source: `src/ast/mod.rs:8643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Extended describe output.

<a id="op-08753533a8cf887c5013fb6a"></a>
## Formatted

`variant` · `sqlparser::ast::HiveDescribeFormat::Formatted` · sqlparser 0.62.0

```rust
Formatted
```

Source: `src/ast/mod.rs:8645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Formatted describe output.

<a id="op-01c1ae4477caf38ab3cc81f7"></a>
## clone

`function` · `sqlparser::ast::HiveDescribeFormat::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> HiveDescribeFormat
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 23], "end": [8637, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-841074ddf18ecaad54cf611e"></a>
## cmp

`function` · `sqlparser::ast::HiveDescribeFormat::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &HiveDescribeFormat) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 57], "end": [8637, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2db920086fffa3d997917a1"></a>
## deserialize

`function` · `sqlparser::ast::HiveDescribeFormat::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8638, 49], "end": [8638, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-814e215e279c2b772480fe6d"></a>
## eq

`function` · `sqlparser::ast::HiveDescribeFormat::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &HiveDescribeFormat) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 30], "end": [8637, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-564d6d092f612dea932748fd"></a>
## fmt

`function` · `sqlparser::ast::HiveDescribeFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8648, 1], "end": [8656, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60a02f4230f433b7e69f7ffa"></a>
## fmt

`function` · `sqlparser::ast::HiveDescribeFormat::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 10], "end": [8637, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfa73528b0773a451bf5eb7b"></a>
## hash

`function` · `sqlparser::ast::HiveDescribeFormat::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 62], "end": [8637, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8009f95f40897f06b9995cf5"></a>
## partial_cmp

`function` · `sqlparser::ast::HiveDescribeFormat::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &HiveDescribeFormat) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8637, 41], "end": [8637, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8637`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-259bd612f7ea50dec4e398cb"></a>
## serialize

`function` · `sqlparser::ast::HiveDescribeFormat::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8638, 38], "end": [8638, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8aac311de76bd43ede1f0c22"></a>
## visit

`function` · `sqlparser::ast::HiveDescribeFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8639, 40], "end": [8639, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8639`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7a1b7d49e432fa443ed6c03"></a>
## visit

`function` · `sqlparser::ast::HiveDescribeFormat::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::HiveDescribeFormat", "path": "HiveDescribeFormat"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8639, 47], "end": [8639, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8639`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
