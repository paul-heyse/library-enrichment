# `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.key_value_options.KeyValueOptionKind.json).

<a id="op-30b750549531a6574b92034b"></a>
## KeyValueOptionKind

`enum` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind` · sqlparser 0.62.0

```rust
enum KeyValueOptionKind
```

Source: `src/ast/helpers/key_value_options.rs:76`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An option can have a single value, multiple values or a nested list of values.

A value can be numeric, boolean, etc. Enum-style values are represented
as Value::Placeholder. For example: MFA_METHOD=SMS will be represented as
`Value::Placeholder("SMS".to_string)`.
The kind of value for a key-value option.

<a id="op-31afbac4f1222961a44d3802"></a>
## KeyValueOptions

`variant` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::KeyValueOptions` · sqlparser 0.62.0

```rust
KeyValueOptions
```

Source: `src/ast/helpers/key_value_options.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A nested list of key-value options.

<a id="op-26125e00d3a66c53714f26c0"></a>
## Multi

`variant` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::Multi` · sqlparser 0.62.0

```rust
Multi
```

Source: `src/ast/helpers/key_value_options.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Multiple values.

<a id="op-f772f773e4c28c188cdfa67b"></a>
## Single

`variant` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::Single` · sqlparser 0.62.0

```rust
Single
```

Source: `src/ast/helpers/key_value_options.rs:78`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single value.

<a id="op-4ab9b3e87e0806ddcd143248"></a>
## clone

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KeyValueOptionKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 22], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1663d8d4c81e3ac8568230f9"></a>
## cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KeyValueOptionKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 51], "end": [72, 54], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d6eb29c8ca11294af7571c"></a>
## deserialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 49], "end": [73, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/key_value_options.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6643b1333d7513805dde6fa8"></a>
## eq

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KeyValueOptionKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 24], "end": [72, 33], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1f9feca4091ab7fb9fbda99"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc6c508636441379fad4f3ca"></a>
## hash

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 56], "end": [72, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be552373c061b8f07747f49d"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KeyValueOptionKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 39], "end": [72, 49], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/key_value_options.rs:72`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-76e4288c786540a2ab48965d"></a>
## serialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 38], "end": [73, 47], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/key_value_options.rs:73`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7be2030db5967b01a32334c7"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 40], "end": [74, 45], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/key_value_options.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a92a4d8f586526de897cf7e7"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptionKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptionKind", "path": "KeyValueOptionKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 47], "end": [74, 55], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/key_value_options.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
