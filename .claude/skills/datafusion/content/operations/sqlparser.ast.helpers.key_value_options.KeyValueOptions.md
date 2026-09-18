# `sqlparser::ast::helpers::key_value_options::KeyValueOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.key_value_options.KeyValueOptions.json).

<a id="op-b14b5e42be55e025979ba145"></a>
## KeyValueOptions

`struct` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions` · sqlparser 0.62.0

```rust
struct KeyValueOptions
```

Source: `src/ast/helpers/key_value_options.rs:38`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A collection of key-value options.

<a id="op-85c5b65caff457d3b78b5d28"></a>
## clone

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KeyValueOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aff0f40ec33d4010fff4a96"></a>
## cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KeyValueOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 51], "end": [34, 54], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98cd4866f9038b24fc7d28c1"></a>
## delimiter

`struct_field` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::delimiter` · sqlparser 0.62.0

```rust
delimiter: KeyValueOptionsDelimiter
```

Source: `src/ast/helpers/key_value_options.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The delimiter used between options.

<a id="op-71838041823f25eded61c5ad"></a>
## deserialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 49], "end": [35, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/key_value_options.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09632d378afa6747a3b2b7b7"></a>
## eq

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KeyValueOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 24], "end": [34, 33], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62759e8cb1d86d5fa6fbbcd3"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d27a3f9240c7609cd2c6b89c"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [93, 2], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/key_value_options.rs:86`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40d5482d3a86a57c86005f86"></a>
## hash

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 56], "end": [34, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-613064d87d76d0c1c43dbe1d"></a>
## options

`struct_field` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::options` · sqlparser 0.62.0

```rust
options: Vec<KeyValueOption>
```

Source: `src/ast/helpers/key_value_options.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of key-value options.

<a id="op-f205f23238035297f9b0ad0e"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KeyValueOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 39], "end": [34, 49], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/key_value_options.rs:34`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-058eb2dacf143be19acd8e70"></a>
## serialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [35, 38], "end": [35, 47], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/key_value_options.rs:35`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c86081ace4c9061c9495d0f2"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 47], "end": [36, 55], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/key_value_options.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5436b3a5341730b5e88d257"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOptions", "path": "KeyValueOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [36, 40], "end": [36, 45], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/key_value_options.rs:36`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
