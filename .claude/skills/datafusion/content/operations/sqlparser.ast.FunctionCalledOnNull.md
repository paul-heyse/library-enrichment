# `sqlparser::ast::FunctionCalledOnNull`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionCalledOnNull.json).

<a id="op-44c2a86129b114a853280f70"></a>
## FunctionCalledOnNull

`enum` · `sqlparser::ast::FunctionCalledOnNull` · sqlparser 0.62.0

```rust
enum FunctionCalledOnNull
```

Source: `src/ast/mod.rs:10040`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

These attributes describe the behavior of the function when called with a null argument.

<a id="op-1436ff267e0e3921cc6b1a85"></a>
## CalledOnNullInput

`variant` · `sqlparser::ast::FunctionCalledOnNull::CalledOnNullInput` · sqlparser 0.62.0

```rust
CalledOnNullInput
```

Source: `src/ast/mod.rs:10042`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is called even when inputs are null.

<a id="op-2a7723c8cf9aea6b8c00cf9b"></a>
## ReturnsNullOnNullInput

`variant` · `sqlparser::ast::FunctionCalledOnNull::ReturnsNullOnNullInput` · sqlparser 0.62.0

```rust
ReturnsNullOnNullInput
```

Source: `src/ast/mod.rs:10044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function returns null when any input is null.

<a id="op-f475e0ac2315697542d55eb0"></a>
## Strict

`variant` · `sqlparser::ast::FunctionCalledOnNull::Strict` · sqlparser 0.62.0

```rust
Strict
```

Source: `src/ast/mod.rs:10046`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is strict about null inputs.

<a id="op-322cefcce26c811342b63008"></a>
## clone

`function` · `sqlparser::ast::FunctionCalledOnNull::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionCalledOnNull
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 17], "end": [10037, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-842a317a222147bc82c635c8"></a>
## cmp

`function` · `sqlparser::ast::FunctionCalledOnNull::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionCalledOnNull) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 51], "end": [10037, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5715105a15b57f37c8f3de"></a>
## deserialize

`function` · `sqlparser::ast::FunctionCalledOnNull::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10038, 49], "end": [10038, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad4b1b0c0d3129e531bcb158"></a>
## eq

`function` · `sqlparser::ast::FunctionCalledOnNull::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionCalledOnNull) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 24], "end": [10037, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bfb389e42da024a78fb3c02"></a>
## fmt

`function` · `sqlparser::ast::FunctionCalledOnNull::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 10], "end": [10037, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a670afd3ae55bd8f32794d8"></a>
## fmt

`function` · `sqlparser::ast::FunctionCalledOnNull::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10049, 1], "end": [10057, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10050`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b5ddf02e4b6075b88048b92"></a>
## hash

`function` · `sqlparser::ast::FunctionCalledOnNull::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 56], "end": [10037, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3de9bcea95ab8a7a618d473b"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionCalledOnNull::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionCalledOnNull) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10037, 35], "end": [10037, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd46788689f634e8170023c5"></a>
## serialize

`function` · `sqlparser::ast::FunctionCalledOnNull::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10038, 38], "end": [10038, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10038`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6114055c794132249ac0d8c"></a>
## visit

`function` · `sqlparser::ast::FunctionCalledOnNull::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10039, 47], "end": [10039, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efd04acb1c91a76108e14e08"></a>
## visit

`function` · `sqlparser::ast::FunctionCalledOnNull::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionCalledOnNull", "path": "FunctionCalledOnNull"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10039, 40], "end": [10039, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
