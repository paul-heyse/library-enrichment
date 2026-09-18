# `sqlparser::ast::FunctionArgumentList`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FunctionArgumentList.json).

<a id="op-8212006388c9c442a14377bb"></a>
## FunctionArgumentList

`struct` · `sqlparser::ast::FunctionArgumentList` · sqlparser 0.62.0

```rust
struct FunctionArgumentList
```

Source: `src/ast/mod.rs:8164`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This represents everything inside the parentheses when calling a function.

<a id="op-c3031ee812ff2f4ced2f96c9"></a>
## args

`struct_field` · `sqlparser::ast::FunctionArgumentList::args` · sqlparser 0.62.0

```rust
args: Vec<FunctionArg>
```

Source: `src/ast/mod.rs:8168`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The function arguments.

<a id="op-78bd85f8496ee1f1cc3139fd"></a>
## clauses

`struct_field` · `sqlparser::ast::FunctionArgumentList::clauses` · sqlparser 0.62.0

```rust
clauses: Vec<FunctionArgumentClause>
```

Source: `src/ast/mod.rs:8170`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional clauses specified within the argument list.

<a id="op-0dba516d9ccbbede2d437d0d"></a>
## clone

`function` · `sqlparser::ast::FunctionArgumentList::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FunctionArgumentList
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 17], "end": [8161, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4443e12d58a288628d9120c2"></a>
## cmp

`function` · `sqlparser::ast::FunctionArgumentList::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FunctionArgumentList) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 51], "end": [8161, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c003386fd504b7761b8e16b6"></a>
## deserialize

`function` · `sqlparser::ast::FunctionArgumentList::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8162, 49], "end": [8162, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-279150d144a1374c8777fac5"></a>
## duplicate_treatment

`struct_field` · `sqlparser::ast::FunctionArgumentList::duplicate_treatment` · sqlparser 0.62.0

```rust
duplicate_treatment: Option<DuplicateTreatment>
```

Source: `src/ast/mod.rs:8166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ ALL | DISTINCT ]`

<a id="op-7ae0eaa95ea8046126d83a2e"></a>
## eq

`function` · `sqlparser::ast::FunctionArgumentList::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FunctionArgumentList) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 24], "end": [8161, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fb928faf4c7c32157faa13"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgumentList::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 10], "end": [8161, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2f8eb67e7da07fbd73a4816"></a>
## fmt

`function` · `sqlparser::ast::FunctionArgumentList::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8173, 1], "end": [8187, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6f2f502e3f37cef685361aa"></a>
## hash

`function` · `sqlparser::ast::FunctionArgumentList::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 56], "end": [8161, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-150305e0c104b76e9233dd2a"></a>
## partial_cmp

`function` · `sqlparser::ast::FunctionArgumentList::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FunctionArgumentList) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8161, 35], "end": [8161, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8161`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80c128165dde2a9f0a3c322e"></a>
## serialize

`function` · `sqlparser::ast::FunctionArgumentList::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8162, 38], "end": [8162, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8162`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-115ae8ebbdbae0da67c18664"></a>
## span

`function` · `sqlparser::ast::FunctionArgumentList::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "super::FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1769, 1], "end": [1784, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1770`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e58804846dad9520a77135a"></a>
## visit

`function` · `sqlparser::ast::FunctionArgumentList::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8163, 40], "end": [8163, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69bf7903d88540f49c67eb5f"></a>
## visit

`function` · `sqlparser::ast::FunctionArgumentList::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FunctionArgumentList", "path": "FunctionArgumentList"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8163, 47], "end": [8163, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
