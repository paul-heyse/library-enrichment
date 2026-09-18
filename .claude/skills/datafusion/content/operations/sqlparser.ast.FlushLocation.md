# `sqlparser::ast::FlushLocation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FlushLocation.json).

<a id="op-22936ec7caf56e64ac251700"></a>
## FlushLocation

`enum` · `sqlparser::ast::FlushLocation` · sqlparser 0.62.0

```rust
enum FlushLocation
```

Source: `src/ast/mod.rs:9786`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Location modifier for flush commands.

<a id="op-a373ebdda4fbfc465be75b77"></a>
## Local

`variant` · `sqlparser::ast::FlushLocation::Local` · sqlparser 0.62.0

```rust
Local
```

Source: `src/ast/mod.rs:9790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply flush locally.

<a id="op-d534c494488964bb921ea93f"></a>
## NoWriteToBinlog

`variant` · `sqlparser::ast::FlushLocation::NoWriteToBinlog` · sqlparser 0.62.0

```rust
NoWriteToBinlog
```

Source: `src/ast/mod.rs:9788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Do not write changes to the binary log.

<a id="op-a331accd953480b7d1cc724b"></a>
## clone

`function` · `sqlparser::ast::FlushLocation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FlushLocation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 23], "end": [9783, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e75afaa3b6ddd7de1d7412eb"></a>
## cmp

`function` · `sqlparser::ast::FlushLocation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FlushLocation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 57], "end": [9783, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-330cee033c17b0a8b9bedab4"></a>
## deserialize

`function` · `sqlparser::ast::FlushLocation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9784, 49], "end": [9784, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64c96639158c0fbff690a138"></a>
## eq

`function` · `sqlparser::ast::FlushLocation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FlushLocation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 30], "end": [9783, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a468908a1ca34bca19d724f"></a>
## fmt

`function` · `sqlparser::ast::FlushLocation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9793, 1], "end": [9800, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e061ee832adca51eab9cbc6"></a>
## fmt

`function` · `sqlparser::ast::FlushLocation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 10], "end": [9783, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81935e6b68670f99386cceba"></a>
## hash

`function` · `sqlparser::ast::FlushLocation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 62], "end": [9783, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-726df6ba0a07b0a18c1dbb4a"></a>
## partial_cmp

`function` · `sqlparser::ast::FlushLocation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FlushLocation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9783, 41], "end": [9783, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9783`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc14a2fd924735744ca1cbae"></a>
## serialize

`function` · `sqlparser::ast::FlushLocation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9784, 38], "end": [9784, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9784`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f2e0edfa08f51a00331e5ba"></a>
## visit

`function` · `sqlparser::ast::FlushLocation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9785, 47], "end": [9785, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecf562e5110f73ab9bed12c2"></a>
## visit

`function` · `sqlparser::ast::FlushLocation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushLocation", "path": "FlushLocation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9785, 40], "end": [9785, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9785`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
