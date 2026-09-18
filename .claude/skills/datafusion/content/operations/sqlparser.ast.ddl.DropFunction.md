# `sqlparser::ast::ddl::DropFunction`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropFunction.json).

<a id="op-3458a10fbab33f921f60ab5b"></a>
## DropFunction

`struct` · `sqlparser::ast::ddl::DropFunction` · sqlparser 0.62.0

```rust
struct DropFunction
```

Source: `src/ast/ddl.rs:4717`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DROP FUNCTION statement

<a id="op-558417f36939f1193e140abb"></a>
## clone

`function` · `sqlparser::ast::ddl::DropFunction::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropFunction
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 17], "end": [4714, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3030f246339f161f34d10da4"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropFunction::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropFunction) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 51], "end": [4714, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82976117981b1b49a2a73187"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropFunction::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4715, 49], "end": [4715, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adc115f59188777d5e417a58"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::DropFunction::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:4723`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` or `RESTRICT`

<a id="op-659f50453e000a362432e93c"></a>
## eq

`function` · `sqlparser::ast::ddl::DropFunction::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropFunction) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 24], "end": [4714, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78e716476f98f9edeedf29ed"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4726, 1], "end": [4739, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4727`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea7d6005eacafdb008f17c88"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropFunction::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 10], "end": [4714, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecd00ba5418dfefadbab53b5"></a>
## func_desc

`struct_field` · `sqlparser::ast::ddl::DropFunction::func_desc` · sqlparser 0.62.0

```rust
func_desc: Vec<ast::FunctionDesc>
```

Source: `src/ast/ddl.rs:4721`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more functions to drop

<a id="op-9863c64ef447e66b7d019f18"></a>
## hash

`function` · `sqlparser::ast::ddl::DropFunction::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 56], "end": [4714, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-515463467794006bba9fb21d"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropFunction::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4719`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether to include the `IF EXISTS` clause.

<a id="op-eff232ba8dd27a35cdfe585d"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropFunction::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropFunction) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4714, 35], "end": [4714, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4714`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eb6556dd99ef87932fffb29"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropFunction::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4715, 38], "end": [4715, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4715`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaa1fda80e0392253a068dcf"></a>
## span

`function` · `sqlparser::ast::ddl::DropFunction::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4741, 1], "end": [4745, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4742`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a06399420b942772ba71d9a"></a>
## visit

`function` · `sqlparser::ast::ddl::DropFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4716, 40], "end": [4716, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a52481907f017953de1ee7e9"></a>
## visit

`function` · `sqlparser::ast::ddl::DropFunction::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropFunction", "path": "DropFunction"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4716, 47], "end": [4716, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4716`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
