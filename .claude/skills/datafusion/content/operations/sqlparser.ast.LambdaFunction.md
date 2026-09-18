# `sqlparser::ast::LambdaFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LambdaFunction.json).

<a id="op-d1d85e18f16550a033c9695c"></a>
## LambdaFunction

`struct` · `sqlparser::ast::LambdaFunction` · sqlparser 0.62.0

```rust
struct LambdaFunction
```

Source: `src/ast/mod.rs:1476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A lambda function.

<a id="op-0322fdd87490ea6cf5b1e375"></a>
## body

`struct_field` · `sqlparser::ast::LambdaFunction::body` · sqlparser 0.62.0

```rust
body: Box<Expr>
```

Source: `src/ast/mod.rs:1480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The body of the lambda function.

<a id="op-a36002148fd1f3e6d21da148"></a>
## clone

`function` · `sqlparser::ast::LambdaFunction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LambdaFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 17], "end": [1473, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e20b3477103636b937890e20"></a>
## cmp

`function` · `sqlparser::ast::LambdaFunction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LambdaFunction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 51], "end": [1473, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2315fd09af41532d4c25e025"></a>
## deserialize

`function` · `sqlparser::ast::LambdaFunction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1474, 49], "end": [1474, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc814583454d3f44cb17496f"></a>
## eq

`function` · `sqlparser::ast::LambdaFunction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LambdaFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 24], "end": [1473, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2d3c094f3b8367c1c3168373"></a>
## fmt

`function` · `sqlparser::ast::LambdaFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1485, 1], "end": [1501, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:1486`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-687543878a4ed3c325bcec2d"></a>
## fmt

`function` · `sqlparser::ast::LambdaFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 10], "end": [1473, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eeaf8d271a8c4241b437818"></a>
## hash

`function` · `sqlparser::ast::LambdaFunction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 56], "end": [1473, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5483afcb52a8c88183ea7d0b"></a>
## params

`struct_field` · `sqlparser::ast::LambdaFunction::params` · sqlparser 0.62.0

```rust
params: OneOrManyWithParens<LambdaFunctionParameter>
```

Source: `src/ast/mod.rs:1478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The parameters to the lambda function.

<a id="op-b466b3cfd905510adeec337e"></a>
## partial_cmp

`function` · `sqlparser::ast::LambdaFunction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LambdaFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1473, 35], "end": [1473, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1473`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f723d1c02064bed3f2385a8"></a>
## serialize

`function` · `sqlparser::ast::LambdaFunction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1474, 38], "end": [1474, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67808af785b400873e7eb2a8"></a>
## syntax

`struct_field` · `sqlparser::ast::LambdaFunction::syntax` · sqlparser 0.62.0

```rust
syntax: LambdaSyntax
```

Source: `src/ast/mod.rs:1482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The syntax style used to write the lambda function.

<a id="op-46c00fbeff4ecf09ac350fd9"></a>
## visit

`function` · `sqlparser::ast::LambdaFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1475, 40], "end": [1475, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cb7070547689e117611e6c3"></a>
## visit

`function` · `sqlparser::ast::LambdaFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaFunction", "path": "LambdaFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1475, 47], "end": [1475, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
