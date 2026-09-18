# `sqlparser::ast::FunctionArgOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArgOperator.json).

<a id="op-86d2f98c5ec5a673cc0f0d9d"></a>
## FunctionArgOperator

`enum` · `sqlparser::ast::FunctionArgOperator` · sqlparser 0.62.0

```rust
enum FunctionArgOperator
```

Source: `src/ast/mod.rs:7888`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operator used to separate function arguments

<a id="op-e7d5f2a746ea9d391466ca86"></a>
## Assignment

`variant` · `sqlparser::ast::FunctionArgOperator::Assignment` · sqlparser 0.62.0

```rust
Assignment
```

Source: `src/ast/mod.rs:7894`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

function(arg1 := value1)

<a id="op-7e3d6dcd81aebf852da8aa2f"></a>
## Colon

`variant` · `sqlparser::ast::FunctionArgOperator::Colon` · sqlparser 0.62.0

```rust
Colon
```

Source: `src/ast/mod.rs:7896`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

function(arg1 : value1)

<a id="op-8eb33b12119b2b03b762bf85"></a>
## Equals

`variant` · `sqlparser::ast::FunctionArgOperator::Equals` · sqlparser 0.62.0

```rust
Equals
```

Source: `src/ast/mod.rs:7890`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

function(arg1 = value1)

<a id="op-fae593f96b5d1a1ad21663d5"></a>
## RightArrow

`variant` · `sqlparser::ast::FunctionArgOperator::RightArrow` · sqlparser 0.62.0

```rust
RightArrow
```

Source: `src/ast/mod.rs:7892`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

function(arg1 => value1)

<a id="op-8ceefb54c2e15461140d4772"></a>
## Value

`variant` · `sqlparser::ast::FunctionArgOperator::Value` · sqlparser 0.62.0

```rust
Value
```

Source: `src/ast/mod.rs:7898`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

function(arg1 VALUE value1)

<a id="op-6a8d0c0dc1c0de446003d330"></a>
## clone

`function` · `sqlparser::ast::FunctionArgOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArgOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 17], "end": [7884, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e0969eb24cd8021da2b431b"></a>
## cmp

`function` · `sqlparser::ast::FunctionArgOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArgOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 51], "end": [7884, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f948684df635eff122925bf"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArgOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7885, 49], "end": [7885, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4f0166b909e291805bf1ae8"></a>
## eq

`function` · `sqlparser::ast::FunctionArgOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArgOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 24], "end": [7884, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2de2156740df452d796a8341"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 10], "end": [7884, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77a01e0a2ead228d6aa938e0"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7901, 1], "end": [7911, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7902`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-114c0e2f2af92df2277d2bb4"></a>
## hash

`function` · `sqlparser::ast::FunctionArgOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 56], "end": [7884, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e609b51896e98b67d11f1ea"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArgOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArgOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7884, 35], "end": [7884, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7884`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a87e0521851a3bec694c7a3"></a>
## serialize

`function` · `sqlparser::ast::FunctionArgOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7885, 38], "end": [7885, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-26e3417076420639019e6049"></a>
## visit

`function` · `sqlparser::ast::FunctionArgOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7886, 47], "end": [7886, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7764782382a8b124b69ee606"></a>
## visit

`function` · `sqlparser::ast::FunctionArgOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgOperator", "path": "FunctionArgOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7886, 40], "end": [7886, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7886`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
