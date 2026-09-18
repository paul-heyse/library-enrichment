# `sqlparser::ast::SessionParamValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.SessionParamValue.json).

<a id="op-39313a990126198a6523294b"></a>
## SessionParamValue

`enum` · `sqlparser::ast::SessionParamValue` · sqlparser 0.62.0

```rust
enum SessionParamValue
```

Source: `src/ast/mod.rs:11204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Value for a session boolean-like parameter (ON/OFF).

<a id="op-02fb6a22673005885f0f9a72"></a>
## Off

`variant` · `sqlparser::ast::SessionParamValue::Off` · sqlparser 0.62.0

```rust
Off
```

Source: `src/ast/mod.rs:11208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Session parameter disabled.

<a id="op-0c906871561f43f94605ffe3"></a>
## On

`variant` · `sqlparser::ast::SessionParamValue::On` · sqlparser 0.62.0

```rust
On
```

Source: `src/ast/mod.rs:11206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Session parameter enabled.

<a id="op-b7abb7f421f2c47b95f8e99b"></a>
## clone

`function` · `sqlparser::ast::SessionParamValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> SessionParamValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 17], "end": [11200, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18bd9ada4b9cb4aedb5190c4"></a>
## cmp

`function` · `sqlparser::ast::SessionParamValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &SessionParamValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 51], "end": [11200, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9137be7609bf359c404be20e"></a>
## deserialize

`function` · `sqlparser::ast::SessionParamValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11201, 49], "end": [11201, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f203b7077747033f6ed50ee"></a>
## eq

`function` · `sqlparser::ast::SessionParamValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &SessionParamValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 24], "end": [11200, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f9fb62a794ad5e3c8cf74ce"></a>
## fmt

`function` · `sqlparser::ast::SessionParamValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11211, 1], "end": [11218, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5478ce7da0601c26d795ed52"></a>
## fmt

`function` · `sqlparser::ast::SessionParamValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 10], "end": [11200, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2175e6c890ebf834349467bd"></a>
## hash

`function` · `sqlparser::ast::SessionParamValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 56], "end": [11200, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46a4f82e6462a68520ecc0a8"></a>
## partial_cmp

`function` · `sqlparser::ast::SessionParamValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &SessionParamValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11200, 35], "end": [11200, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97fae98aa7e29aa6e6bd9059"></a>
## serialize

`function` · `sqlparser::ast::SessionParamValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11201, 38], "end": [11201, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04957a4dd271f385d4f33cf1"></a>
## visit

`function` · `sqlparser::ast::SessionParamValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11202, 47], "end": [11202, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-528e7a873b0c670efe65a314"></a>
## visit

`function` · `sqlparser::ast::SessionParamValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::SessionParamValue", "path": "SessionParamValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11202, 40], "end": [11202, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
