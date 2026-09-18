# `sqlparser::ast::FunctionBehavior`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionBehavior.json).

<a id="op-540894a8f4dd4a0ed6c26f1d"></a>
## FunctionBehavior

`enum` · `sqlparser::ast::FunctionBehavior` · sqlparser 0.62.0

```rust
enum FunctionBehavior
```

Source: `src/ast/mod.rs:9954`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

These attributes inform the query optimizer about the behavior of the function.

<a id="op-42702884406129fe6ab6cb32"></a>
## Immutable

`variant` · `sqlparser::ast::FunctionBehavior::Immutable` · sqlparser 0.62.0

```rust
Immutable
```

Source: `src/ast/mod.rs:9956`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is immutable.

<a id="op-630613b914401d282b48b2dc"></a>
## Stable

`variant` · `sqlparser::ast::FunctionBehavior::Stable` · sqlparser 0.62.0

```rust
Stable
```

Source: `src/ast/mod.rs:9958`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is stable.

<a id="op-d942e71de2c11096dc483ae5"></a>
## Volatile

`variant` · `sqlparser::ast::FunctionBehavior::Volatile` · sqlparser 0.62.0

```rust
Volatile
```

Source: `src/ast/mod.rs:9960`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function is volatile.

<a id="op-95647050b9c43368f9282744"></a>
## clone

`function` · `sqlparser::ast::FunctionBehavior::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionBehavior
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 17], "end": [9951, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0901c0fda168411a5dd791e"></a>
## cmp

`function` · `sqlparser::ast::FunctionBehavior::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionBehavior) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 51], "end": [9951, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22572b31288953da05bbef6a"></a>
## deserialize

`function` · `sqlparser::ast::FunctionBehavior::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9952, 49], "end": [9952, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-897cfb0b29a3f68f83d59575"></a>
## eq

`function` · `sqlparser::ast::FunctionBehavior::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionBehavior) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 24], "end": [9951, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0daa6ffdcf4454391c03548a"></a>
## fmt

`function` · `sqlparser::ast::FunctionBehavior::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9963, 1], "end": [9971, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-427b52d9967f036563964bf8"></a>
## fmt

`function` · `sqlparser::ast::FunctionBehavior::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 10], "end": [9951, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8817754b37609709c41142a"></a>
## hash

`function` · `sqlparser::ast::FunctionBehavior::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 56], "end": [9951, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d4d7f4e3de469f07e1adc7e"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionBehavior::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionBehavior) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9951, 35], "end": [9951, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9951`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f04e3a60dd049c157afa1f2b"></a>
## serialize

`function` · `sqlparser::ast::FunctionBehavior::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9952, 38], "end": [9952, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a55d8472b74752772730e4c"></a>
## visit

`function` · `sqlparser::ast::FunctionBehavior::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9953, 47], "end": [9953, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c85ac049cbd7be7d2dff02f"></a>
## visit

`function` · `sqlparser::ast::FunctionBehavior::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionBehavior", "path": "FunctionBehavior"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9953, 40], "end": [9953, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9953`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
