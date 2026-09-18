# `sqlparser::ast::ddl::AlterFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterFunction.json).

<a id="op-9f78c8ab8258ff26473fff44"></a>
## AlterFunction

`struct` · `sqlparser::ast::ddl::AlterFunction` · sqlparser 0.62.0

```rust
struct AlterFunction
```

Source: `src/ast/ddl.rs:5366`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ALTER FUNCTION` / `ALTER AGGREGATE` statement.

<a id="op-8056fb42c882781d59f58134"></a>
## aggregate_order_by

`struct_field` · `sqlparser::ast::ddl::AlterFunction::aggregate_order_by` · sqlparser 0.62.0

```rust
aggregate_order_by: Option<Vec<ast::OperateFunctionArg>>
```

Source: `src/ast/ddl.rs:5374`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ORDER BY` argument list for aggregate signatures.

This is only used for `ALTER AGGREGATE`.

<a id="op-50f0c848a415c1ebd06ba023"></a>
## aggregate_star

`struct_field` · `sqlparser::ast::ddl::AlterFunction::aggregate_star` · sqlparser 0.62.0

```rust
aggregate_star: bool
```

Source: `src/ast/ddl.rs:5378`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the aggregate signature uses `*`.

This is only used for `ALTER AGGREGATE`.

<a id="op-7ec1106d0514f4aa14b48e8b"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterFunction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 17], "end": [5363, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2e2e1c7b0cff012adef464e"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterFunction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterFunction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 51], "end": [5363, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad58a7ff6a888536b7d210bb"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterFunction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5364, 49], "end": [5364, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38e1abcd254c7ac398f79d88"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterFunction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 24], "end": [5363, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2332af5428442bc9e6656ea6"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5469, 1], "end": [5501, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a47f6dd54ec7176ecbb1b305"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 10], "end": [5363, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf2331abcbb642759882ec9f"></a>
## function

`struct_field` · `sqlparser::ast::ddl::AlterFunction::function` · sqlparser 0.62.0

```rust
function: ast::FunctionDesc
```

Source: `src/ast/ddl.rs:5370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function or aggregate signature.

<a id="op-db2958fe8fa1b98faf957803"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterFunction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 56], "end": [5363, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a38686380a51c41c9e09b8c"></a>
## kind

`struct_field` · `sqlparser::ast::ddl::AlterFunction::kind` · sqlparser 0.62.0

```rust
kind: AlterFunctionKind
```

Source: `src/ast/ddl.rs:5368`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Object type being altered.

<a id="op-c0f645be1e5b9fe90da864b0"></a>
## operation

`struct_field` · `sqlparser::ast::ddl::AlterFunction::operation` · sqlparser 0.62.0

```rust
operation: AlterFunctionOperation
```

Source: `src/ast/ddl.rs:5380`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operation applied to the object.

<a id="op-1eec588ad8ac148a19b172fd"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterFunction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5363, 35], "end": [5363, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5363`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ae0fbd8a57eb4a270347414"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterFunction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5364, 38], "end": [5364, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc117288e38c705d4d65a469"></a>
## span

`function` · `sqlparser::ast::ddl::AlterFunction::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5564, 1], "end": [5568, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5565`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-199966b983fed03707a131c3"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5365, 40], "end": [5365, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26035711eedd48d8ea68e288"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterFunction", "path": "AlterFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5365, 47], "end": [5365, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5365`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
