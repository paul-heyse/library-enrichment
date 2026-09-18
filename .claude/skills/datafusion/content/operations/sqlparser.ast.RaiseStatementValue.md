# `sqlparser::ast::RaiseStatementValue`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.RaiseStatementValue.json).

<a id="op-a936037287f54208d28d9b90"></a>
## RaiseStatementValue

`enum` · `sqlparser::ast::RaiseStatementValue` · sqlparser 0.62.0

```rust
enum RaiseStatementValue
```

Source: `src/ast/mod.rs:2905`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the error value of a [RaiseStatement](../operations/sqlparser.ast.RaiseStatement.md#op-23ff2b4a827bdc3b655accea).

<a id="op-48d16a4680dc12967122e709"></a>
## Expr

`variant` · `sqlparser::ast::RaiseStatementValue::Expr` · sqlparser 0.62.0

```rust
Expr
```

Source: `src/ast/mod.rs:2909`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RAISE myerror`

<a id="op-a3420e013ea24e9c7b314f57"></a>
## UsingMessage

`variant` · `sqlparser::ast::RaiseStatementValue::UsingMessage` · sqlparser 0.62.0

```rust
UsingMessage
```

Source: `src/ast/mod.rs:2907`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RAISE USING MESSAGE = 'error'`

<a id="op-6cdd82a1a0b617fde5a16c13"></a>
## clone

`function` · `sqlparser::ast::RaiseStatementValue::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RaiseStatementValue
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 17], "end": [2902, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d01a5de67440b42b469dabf"></a>
## cmp

`function` · `sqlparser::ast::RaiseStatementValue::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RaiseStatementValue) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 51], "end": [2902, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66d8f44826f1ec4b2a07aee2"></a>
## deserialize

`function` · `sqlparser::ast::RaiseStatementValue::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2903, 49], "end": [2903, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:2903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-454402a739c9aaed257553e9"></a>
## eq

`function` · `sqlparser::ast::RaiseStatementValue::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RaiseStatementValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 24], "end": [2902, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc3209c3fbabacfc8e97abe2"></a>
## fmt

`function` · `sqlparser::ast::RaiseStatementValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 10], "end": [2902, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9a43050eaa5006fdd899397"></a>
## fmt

`function` · `sqlparser::ast::RaiseStatementValue::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2912, 1], "end": [2919, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:2913`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17e782aa377fa222ec070eb7"></a>
## hash

`function` · `sqlparser::ast::RaiseStatementValue::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 56], "end": [2902, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e76b958abd0441a6dadd5c89"></a>
## partial_cmp

`function` · `sqlparser::ast::RaiseStatementValue::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RaiseStatementValue) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2902, 35], "end": [2902, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:2902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-073aaab0e516dad43dfab890"></a>
## serialize

`function` · `sqlparser::ast::RaiseStatementValue::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2903, 38], "end": [2903, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:2903`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cc64393264d28d1ae9b932e"></a>
## span

`function` · `sqlparser::ast::RaiseStatementValue::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "super::RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [798, 1], "end": [805, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:799`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-714d062f28a88d7a4dd1de9a"></a>
## visit

`function` · `sqlparser::ast::RaiseStatementValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2904, 40], "end": [2904, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:2904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2e6f09b8c52cff0330322d5"></a>
## visit

`function` · `sqlparser::ast::RaiseStatementValue::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::RaiseStatementValue", "path": "RaiseStatementValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2904, 47], "end": [2904, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:2904`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
