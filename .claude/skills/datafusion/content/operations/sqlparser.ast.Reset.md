# `sqlparser::ast::Reset`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Reset.json).

<a id="op-a022635856c95330baa74183"></a>
## Reset

`enum` · `sqlparser::ast::Reset` · sqlparser 0.62.0

```rust
enum Reset
```

Source: `src/ast/mod.rs:11955`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variants of the RESET statement

<a id="op-6c58151652e5b996de418a6f"></a>
## ALL

`variant` · `sqlparser::ast::Reset::ALL` · sqlparser 0.62.0

```rust
ALL
```

Source: `src/ast/mod.rs:11957`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Resets all session parameters to their default values.

<a id="op-88087ac441106a354c95f78b"></a>
## ConfigurationParameter

`variant` · `sqlparser::ast::Reset::ConfigurationParameter` · sqlparser 0.62.0

```rust
ConfigurationParameter
```

Source: `src/ast/mod.rs:11960`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Resets a specific session parameter to its default value.

<a id="op-5851d76300520f0ccc9e1ce7"></a>
## clone

`function` · `sqlparser::ast::Reset::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Reset
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 17], "end": [11952, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01d2d991be307d14bb9cb61"></a>
## cmp

`function` · `sqlparser::ast::Reset::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Reset) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 51], "end": [11952, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c758ae6462bf1a1a825b3cf"></a>
## deserialize

`function` · `sqlparser::ast::Reset::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11953, 49], "end": [11953, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2b9d58e898a855791a68afa"></a>
## eq

`function` · `sqlparser::ast::Reset::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Reset) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 24], "end": [11952, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1af70810daa315acec04d2b6"></a>
## fmt

`function` · `sqlparser::ast::Reset::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 10], "end": [11952, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d820c9a1de7decbd33576ac0"></a>
## hash

`function` · `sqlparser::ast::Reset::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 56], "end": [11952, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-691f92864da870b84d985903"></a>
## partial_cmp

`function` · `sqlparser::ast::Reset::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Reset) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11952, 35], "end": [11952, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c305c2a147658ea4bdc15f4"></a>
## serialize

`function` · `sqlparser::ast::Reset::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11953, 38], "end": [11953, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d4f35996592d86ad4501da2"></a>
## visit

`function` · `sqlparser::ast::Reset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11954, 47], "end": [11954, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6f128c24bd7387694506837"></a>
## visit

`function` · `sqlparser::ast::Reset::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Reset", "path": "Reset"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11954, 40], "end": [11954, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
