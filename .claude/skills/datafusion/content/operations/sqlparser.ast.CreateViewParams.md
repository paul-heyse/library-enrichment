# `sqlparser::ast::CreateViewParams`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateViewParams.json).

<a id="op-abf2c23ab63327f3c7a568db"></a>
## CreateViewParams

`struct` · `sqlparser::ast::CreateViewParams` · sqlparser 0.62.0

```rust
struct CreateViewParams
```

Source: `src/ast/mod.rs:10519`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[MySQL] `CREATE VIEW` additional parameters

[MySQL]: https://dev.mysql.com/doc/refman/9.1/en/create-view.html

<a id="op-bbb45be1460b927f404ad888"></a>
## algorithm

`struct_field` · `sqlparser::ast::CreateViewParams::algorithm` · sqlparser 0.62.0

```rust
algorithm: Option<CreateViewAlgorithm>
```

Source: `src/ast/mod.rs:10521`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional view algorithm (e.g., MERGE, TEMPTABLE).

<a id="op-7fb5356fe1adb69bfb299aa2"></a>
## clone

`function` · `sqlparser::ast::CreateViewParams::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateViewParams
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 17], "end": [10516, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57d802350c5f570c44d28d90"></a>
## cmp

`function` · `sqlparser::ast::CreateViewParams::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateViewParams) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 51], "end": [10516, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72c2f93cd44c6a01f33ad842"></a>
## definer

`struct_field` · `sqlparser::ast::CreateViewParams::definer` · sqlparser 0.62.0

```rust
definer: Option<GranteeName>
```

Source: `src/ast/mod.rs:10523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional definer (the security principal that will own the view).

<a id="op-dbf4523b141d793f89e42641"></a>
## deserialize

`function` · `sqlparser::ast::CreateViewParams::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10517, 49], "end": [10517, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e77c789b5b239060b398318e"></a>
## eq

`function` · `sqlparser::ast::CreateViewParams::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateViewParams) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 24], "end": [10516, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ee2700a6cfbab3b9b6bf505"></a>
## fmt

`function` · `sqlparser::ast::CreateViewParams::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10528, 1], "end": [10546, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10529`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc7e3e3d79f6834fae99a561"></a>
## fmt

`function` · `sqlparser::ast::CreateViewParams::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 10], "end": [10516, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caea0e95cb72d076bf26dc51"></a>
## hash

`function` · `sqlparser::ast::CreateViewParams::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 56], "end": [10516, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56acf8e27f707155cd9454fe"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateViewParams::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateViewParams) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10516, 35], "end": [10516, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d29bdfdb6889292db33bf3b5"></a>
## security

`struct_field` · `sqlparser::ast::CreateViewParams::security` · sqlparser 0.62.0

```rust
security: Option<CreateViewSecurity>
```

Source: `src/ast/mod.rs:10525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional SQL SECURITY setting for the view.

<a id="op-762bb62d19fe6c008d78d478"></a>
## serialize

`function` · `sqlparser::ast::CreateViewParams::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10517, 38], "end": [10517, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22ef301627899a008dba7a10"></a>
## visit

`function` · `sqlparser::ast::CreateViewParams::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10518, 40], "end": [10518, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-618434066f32ef4e2579c481"></a>
## visit

`function` · `sqlparser::ast::CreateViewParams::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateViewParams", "path": "CreateViewParams"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10518, 47], "end": [10518, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
