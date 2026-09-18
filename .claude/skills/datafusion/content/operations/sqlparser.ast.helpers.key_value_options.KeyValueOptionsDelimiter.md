# `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.key_value_options.KeyValueOptionsDelimiter.json).

<a id="op-05515e3118020503bb08a40c"></a>
## KeyValueOptionsDelimiter

`enum` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter` · sqlparser 0.62.0

```rust
enum KeyValueOptionsDelimiter
```

Source: `src/ast/helpers/key_value_options.rs:49`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The delimiter used between key-value options.

<a id="op-dcb4e1eae5eee2e13e424f8f"></a>
## Comma

`variant` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::Comma` · sqlparser 0.62.0

```rust
Comma
```

Source: `src/ast/helpers/key_value_options.rs:53`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options are separated by commas.

<a id="op-baad598809decea6b41b8ba9"></a>
## Space

`variant` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::Space` · sqlparser 0.62.0

```rust
Space
```

Source: `src/ast/helpers/key_value_options.rs:51`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options are separated by spaces.

<a id="op-28dabab6833a9b533dacbe56"></a>
## clone

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KeyValueOptionsDelimiter
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 17], "end": [45, 22], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-888f5cdfd333cb643ebf6fd6"></a>
## cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KeyValueOptionsDelimiter) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 51], "end": [45, 54], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0af7e4ba2ca8fd78d0d35574"></a>
## deserialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 49], "end": [46, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/key_value_options.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6784cb93b4851384ae29863c"></a>
## eq

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KeyValueOptionsDelimiter) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 24], "end": [45, 33], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79aab150dcab702629da8761"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 10], "end": [45, 15], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ca8afc6d66f73bf57d5c605"></a>
## hash

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 56], "end": [45, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc0c85eede00b8f840297a25"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KeyValueOptionsDelimiter) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [45, 39], "end": [45, 49], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/key_value_options.rs:45`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f409919539956538499e9533"></a>
## serialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 38], "end": [46, 47], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/key_value_options.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b727f11a4cdc2726385e14"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 47], "end": [47, 55], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/key_value_options.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc369111612d6a102662679e"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionsDelimiter", "path": "KeyValueOptionsDelimiter"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 40], "end": [47, 45], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/key_value_options.rs:47`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
