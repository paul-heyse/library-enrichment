# `sqlparser::ast::FunctionParallel`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionParallel.json).

<a id="op-40bd9488cce7ca8f7f639dfd"></a>
## FunctionParallel

`enum` · `sqlparser::ast::FunctionParallel` · sqlparser 0.62.0

```rust
enum FunctionParallel
```

Source: `src/ast/mod.rs:10063`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If it is safe for PostgreSQL to call the function from multiple threads at once

<a id="op-1105c7a0c53b849ebbef3821"></a>
## Restricted

`variant` · `sqlparser::ast::FunctionParallel::Restricted` · sqlparser 0.62.0

```rust
Restricted
```

Source: `src/ast/mod.rs:10067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function is restricted for parallel execution.

<a id="op-b09cd7ce19f31be124d4d1e6"></a>
## Safe

`variant` · `sqlparser::ast::FunctionParallel::Safe` · sqlparser 0.62.0

```rust
Safe
```

Source: `src/ast/mod.rs:10069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function is safe to run in parallel.

<a id="op-5c648fdfa2da4f707b5c778c"></a>
## Unsafe

`variant` · `sqlparser::ast::FunctionParallel::Unsafe` · sqlparser 0.62.0

```rust
Unsafe
```

Source: `src/ast/mod.rs:10065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function is not safe to run in parallel.

<a id="op-2e4f0443962be9a8c199500a"></a>
## clone

`function` · `sqlparser::ast::FunctionParallel::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionParallel
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 17], "end": [10060, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04889f184b1b3fcccd8c68b7"></a>
## cmp

`function` · `sqlparser::ast::FunctionParallel::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionParallel) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 51], "end": [10060, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d801f2f9ebfe5d4ed8e249e9"></a>
## deserialize

`function` · `sqlparser::ast::FunctionParallel::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10061, 49], "end": [10061, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d73a8e0aed2bc718abe35762"></a>
## eq

`function` · `sqlparser::ast::FunctionParallel::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionParallel) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 24], "end": [10060, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-739847a9766c1c1532fe0a95"></a>
## fmt

`function` · `sqlparser::ast::FunctionParallel::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10072, 1], "end": [10080, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10073`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e83a8123cf5289857ff79733"></a>
## fmt

`function` · `sqlparser::ast::FunctionParallel::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 10], "end": [10060, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9a991572be7429f6263d7f4"></a>
## hash

`function` · `sqlparser::ast::FunctionParallel::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 56], "end": [10060, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-588c101246e35118c4df9f3f"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionParallel::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionParallel) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10060, 35], "end": [10060, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-670b89ef64eed95b32c0c551"></a>
## serialize

`function` · `sqlparser::ast::FunctionParallel::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10061, 38], "end": [10061, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10061`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a81b4357ee0068fa191668ff"></a>
## visit

`function` · `sqlparser::ast::FunctionParallel::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10062, 47], "end": [10062, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da808d8101a8ccef8f32d116"></a>
## visit

`function` · `sqlparser::ast::FunctionParallel::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionParallel", "path": "FunctionParallel"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10062, 40], "end": [10062, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
