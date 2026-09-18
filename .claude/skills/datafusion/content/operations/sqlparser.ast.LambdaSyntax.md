# `sqlparser::ast::LambdaSyntax`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.LambdaSyntax.json).

<a id="op-ca1605b8112a13f5fb25e121"></a>
## LambdaSyntax

`enum` · `sqlparser::ast::LambdaSyntax` · sqlparser 0.62.0

```rust
enum LambdaSyntax
```

Source: `src/ast/mod.rs:1528`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The syntax style for a lambda function.

<a id="op-f74ffbcdefeda483a7728cb5"></a>
## Arrow

`variant` · `sqlparser::ast::LambdaSyntax::Arrow` · sqlparser 0.62.0

```rust
Arrow
```

Source: `src/ast/mod.rs:1535`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arrow syntax: `param -> expr` or `(param1, param2) -> expr`

<https://docs.databricks.com/aws/en/sql/language-manual/sql-ref-lambda-functions>

Supported, but deprecated in DuckDB:
<https://duckdb.org/docs/stable/sql/functions/lambda>

<a id="op-63445111f01c5ce6b601cb34"></a>
## LambdaKeyword

`variant` · `sqlparser::ast::LambdaSyntax::LambdaKeyword` · sqlparser 0.62.0

```rust
LambdaKeyword
```

Source: `src/ast/mod.rs:1540`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Lambda keyword syntax: `lambda param : expr` or `lambda param1, param2 : expr`

Recommended in DuckDB:
<https://duckdb.org/docs/stable/sql/functions/lambda>

<a id="op-4ff278efe6a09c105e736cdf"></a>
## clone

`function` · `sqlparser::ast::LambdaSyntax::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> LambdaSyntax
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 17], "end": [1525, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3778e2b5125d6e5580d89ac5"></a>
## cmp

`function` · `sqlparser::ast::LambdaSyntax::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &LambdaSyntax) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 51], "end": [1525, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff267df579eba1b6b641c0f0"></a>
## deserialize

`function` · `sqlparser::ast::LambdaSyntax::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1526, 49], "end": [1526, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:1526`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a45d423245d6e2e94fd67525"></a>
## eq

`function` · `sqlparser::ast::LambdaSyntax::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &LambdaSyntax) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 24], "end": [1525, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e11f28e9bd027840c332c13b"></a>
## fmt

`function` · `sqlparser::ast::LambdaSyntax::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 10], "end": [1525, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7fbc6375443785ced7c5476"></a>
## hash

`function` · `sqlparser::ast::LambdaSyntax::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 56], "end": [1525, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16d7fdb7ad4e6efecca0d1bf"></a>
## partial_cmp

`function` · `sqlparser::ast::LambdaSyntax::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &LambdaSyntax) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1525, 35], "end": [1525, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:1525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52a444e5b2a41a0223cf5972"></a>
## serialize

`function` · `sqlparser::ast::LambdaSyntax::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1526, 38], "end": [1526, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:1526`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06a49dd2dc6f3b853fd1ca8b"></a>
## visit

`function` · `sqlparser::ast::LambdaSyntax::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1527, 40], "end": [1527, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:1527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-325208be18397ad7f61b9114"></a>
## visit

`function` · `sqlparser::ast::LambdaSyntax::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::LambdaSyntax", "path": "LambdaSyntax"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1527, 47], "end": [1527, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:1527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
