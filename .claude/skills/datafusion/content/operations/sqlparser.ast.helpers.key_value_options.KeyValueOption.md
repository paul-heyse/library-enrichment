# `sqlparser::ast::helpers::key_value_options::KeyValueOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.key_value_options.KeyValueOption.json).

<a id="op-2b60f6bc8d59332dd84e7a71"></a>
## KeyValueOption

`struct` · `sqlparser::ast::helpers::key_value_options::KeyValueOption` · sqlparser 0.62.0

```rust
struct KeyValueOption
```

Source: `src/ast/helpers/key_value_options.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A single key-value option.

<a id="op-256b0b5eddf4422963c68ee4"></a>
## clone

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KeyValueOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 22], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d7481849ca002ef181cfa0"></a>
## cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KeyValueOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 51], "end": [56, 54], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cfa81337527e057db1e6695"></a>
## deserialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 49], "end": [57, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/key_value_options.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac184d4e4e9061ac6fe188a4"></a>
## eq

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KeyValueOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 24], "end": [56, 33], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f9c34dd9009764d59ff0984"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [115, 2], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/key_value_options.rs:96`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a22011dacfa26c59869a8cec"></a>
## fmt

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3576f359fc923d6938dc9c4"></a>
## hash

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 56], "end": [56, 60], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-972b8f4e55121dcf6c1964c0"></a>
## option_name

`struct_field` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::option_name` · sqlparser 0.62.0

```rust
option_name: String
```

Source: `src/ast/helpers/key_value_options.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the option.

<a id="op-58f8f546949b7dc5c2e994d7"></a>
## option_value

`struct_field` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::option_value` · sqlparser 0.62.0

```rust
option_value: KeyValueOptionKind
```

Source: `src/ast/helpers/key_value_options.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value of the option.

<a id="op-676226228c00628306611166"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KeyValueOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 39], "end": [56, 49], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/key_value_options.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd3bf1744e504094227b182e"></a>
## serialize

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 38], "end": [57, 47], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/key_value_options.rs:57`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c790ad55ed9a0405ddb783e"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 47], "end": [58, 55], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/key_value_options.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f809bdee61c4b79fd89e1d1"></a>
## visit

`function` · `sqlparser::ast::helpers::key_value_options::KeyValueOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::key_value_options::KeyValueOption", "path": "KeyValueOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 40], "end": [58, 45], "filename": "src/ast/helpers/key_value_options.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/key_value_options.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
