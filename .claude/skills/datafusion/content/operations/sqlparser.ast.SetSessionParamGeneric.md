# `sqlparser::ast::SetSessionParamGeneric`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SetSessionParamGeneric.json).

<a id="op-7d0f10c13b12e2420a6a86ed"></a>
## SetSessionParamGeneric

`struct` · `sqlparser::ast::SetSessionParamGeneric` · sqlparser 0.62.0

```rust
struct SetSessionParamGeneric
```

Source: `src/ast/mod.rs:11105`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic `SET SESSION` parameter represented as name(s) and value.

<a id="op-4160d795b7cefdaf2f1acfc3"></a>
## clone

`function` · `sqlparser::ast::SetSessionParamGeneric::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SetSessionParamGeneric
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 17], "end": [11101, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-102c3146fd800d217861f9df"></a>
## cmp

`function` · `sqlparser::ast::SetSessionParamGeneric::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SetSessionParamGeneric) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 51], "end": [11101, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8131804804a2ca806c9fb5b"></a>
## deserialize

`function` · `sqlparser::ast::SetSessionParamGeneric::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11102, 49], "end": [11102, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cc08061d8dad01202a4dfdb"></a>
## eq

`function` · `sqlparser::ast::SetSessionParamGeneric::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SetSessionParamGeneric) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 24], "end": [11101, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34a314e882ec8b6e33580685"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamGeneric::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 10], "end": [11101, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b67ca3f933203d56af4ff6dd"></a>
## fmt

`function` · `sqlparser::ast::SetSessionParamGeneric::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11112, 1], "end": [11116, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11113`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2df54d8536e45ded7a549ce"></a>
## hash

`function` · `sqlparser::ast::SetSessionParamGeneric::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 56], "end": [11101, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edf9037a6261d8be52512a93"></a>
## names

`struct_field` · `sqlparser::ast::SetSessionParamGeneric::names` · sqlparser 0.62.0

```rust
names: Vec<String>
```

Source: `src/ast/mod.rs:11107`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Names of the session parameters being set.

<a id="op-b1ee737f519fb4e826556279"></a>
## partial_cmp

`function` · `sqlparser::ast::SetSessionParamGeneric::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SetSessionParamGeneric) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11101, 35], "end": [11101, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11101`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7114664f9b963c73e1db4f6a"></a>
## serialize

`function` · `sqlparser::ast::SetSessionParamGeneric::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11102, 38], "end": [11102, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11102`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3f800281c25b95f3af03b70"></a>
## value

`struct_field` · `sqlparser::ast::SetSessionParamGeneric::value` · sqlparser 0.62.0

```rust
value: String
```

Source: `src/ast/mod.rs:11109`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The value to assign to the parameter(s).

<a id="op-4f87ce5793f6b5b1700c451d"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamGeneric::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11103, 47], "end": [11103, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9caaa084b7dc61c54dad059"></a>
## visit

`function` · `sqlparser::ast::SetSessionParamGeneric::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SetSessionParamGeneric", "path": "SetSessionParamGeneric"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11103, 40], "end": [11103, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
