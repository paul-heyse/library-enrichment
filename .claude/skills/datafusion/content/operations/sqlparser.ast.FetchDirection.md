# `sqlparser::ast::FetchDirection`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FetchDirection.json).

<a id="op-d553066265217d83f661c2fe"></a>
## FetchDirection

`enum` · `sqlparser::ast::FetchDirection` · sqlparser 0.62.0

```rust
enum FetchDirection
```

Source: `src/ast/mod.rs:6857`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specific direction for FETCH statement

<a id="op-b65a0e4ea4ba424d787f3f8a"></a>
## Absolute

`variant` · `sqlparser::ast::FetchDirection::Absolute` · sqlparser 0.62.0

```rust
Absolute
```

Source: `src/ast/mod.rs:6872`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch an absolute row by index.

<a id="op-937e3b5857f31f4d349fd9c3"></a>
## All

`variant` · `sqlparser::ast::FetchDirection::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/mod.rs:6882`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch all rows.

<a id="op-eab02a8fc2b7dcf64e6e7a20"></a>
## Backward

`variant` · `sqlparser::ast::FetchDirection::Backward` · sqlparser 0.62.0

```rust
Backward
```

Source: `src/ast/mod.rs:6895`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch backward by an optional limit.

<a id="op-642b14bfddbc7eff26d054ee"></a>
## BackwardAll

`variant` · `sqlparser::ast::FetchDirection::BackwardAll` · sqlparser 0.62.0

```rust
BackwardAll
```

Source: `src/ast/mod.rs:6900`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch all backward rows.

<a id="op-15db7ef9ef371696edeab1ce"></a>
## Count

`variant` · `sqlparser::ast::FetchDirection::Count` · sqlparser 0.62.0

```rust
Count
```

Source: `src/ast/mod.rs:6859`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch a specific count of rows.

<a id="op-b9a5f7de928e93e0d88ea381"></a>
## First

`variant` · `sqlparser::ast::FetchDirection::First` · sqlparser 0.62.0

```rust
First
```

Source: `src/ast/mod.rs:6868`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch the first row.

<a id="op-5d8594a903085d546cf463c6"></a>
## Forward

`variant` · `sqlparser::ast::FetchDirection::Forward` · sqlparser 0.62.0

```rust
Forward
```

Source: `src/ast/mod.rs:6886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch forward by an optional limit.

<a id="op-e2b7aa8aa11d39fd886d38cd"></a>
## ForwardAll

`variant` · `sqlparser::ast::FetchDirection::ForwardAll` · sqlparser 0.62.0

```rust
ForwardAll
```

Source: `src/ast/mod.rs:6891`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch all forward rows.

<a id="op-8bea33b849be8b7a5eca3677"></a>
## Last

`variant` · `sqlparser::ast::FetchDirection::Last` · sqlparser 0.62.0

```rust
Last
```

Source: `src/ast/mod.rs:6870`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch the last row.

<a id="op-782bc8eb6489a65d935db977"></a>
## Next

`variant` · `sqlparser::ast::FetchDirection::Next` · sqlparser 0.62.0

```rust
Next
```

Source: `src/ast/mod.rs:6864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch the next row.

<a id="op-8a0e4083a9fe68029ba06acd"></a>
## Prior

`variant` · `sqlparser::ast::FetchDirection::Prior` · sqlparser 0.62.0

```rust
Prior
```

Source: `src/ast/mod.rs:6866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch the prior row.

<a id="op-ee27c649c164b13ed120250d"></a>
## Relative

`variant` · `sqlparser::ast::FetchDirection::Relative` · sqlparser 0.62.0

```rust
Relative
```

Source: `src/ast/mod.rs:6877`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Fetch a row relative to the current position.

<a id="op-62c7acb0cf9252d0d7f0d619"></a>
## clone

`function` · `sqlparser::ast::FetchDirection::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FetchDirection
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 17], "end": [6854, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6b5b281a1afb3a7e992ae65"></a>
## cmp

`function` · `sqlparser::ast::FetchDirection::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FetchDirection) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 51], "end": [6854, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abfff94ad4a0abb258bd2429"></a>
## deserialize

`function` · `sqlparser::ast::FetchDirection::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6855, 49], "end": [6855, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a915c37fc3b4beded1845e1"></a>
## eq

`function` · `sqlparser::ast::FetchDirection::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FetchDirection) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 24], "end": [6854, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae3729a988e363f626f25f84"></a>
## fmt

`function` · `sqlparser::ast::FetchDirection::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 10], "end": [6854, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7e406d8aaaac3296a29ce9a"></a>
## fmt

`function` · `sqlparser::ast::FetchDirection::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6903, 1], "end": [6942, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:6904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-778baeb09f0f643cfc062173"></a>
## hash

`function` · `sqlparser::ast::FetchDirection::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 56], "end": [6854, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ff842f65432a292d70be11a"></a>
## partial_cmp

`function` · `sqlparser::ast::FetchDirection::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FetchDirection) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6854, 35], "end": [6854, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a60f2c34dd9022f3f6282602"></a>
## serialize

`function` · `sqlparser::ast::FetchDirection::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6855, 38], "end": [6855, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6855`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34ac0b116b8018f1896f5475"></a>
## visit

`function` · `sqlparser::ast::FetchDirection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6856, 47], "end": [6856, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc61494b2aedfe88f8c6d2d2"></a>
## visit

`function` · `sqlparser::ast::FetchDirection::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FetchDirection", "path": "FetchDirection"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6856, 40], "end": [6856, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6856`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
