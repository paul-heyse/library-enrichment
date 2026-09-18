# `sqlparser::ast::ddl::OperatorOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.OperatorOption.json).

<a id="op-b04076c64fce630a64fcab6c"></a>
## OperatorOption

`enum` · `sqlparser::ast::ddl::OperatorOption` · sqlparser 0.62.0

```rust
enum OperatorOption
```

Source: `src/ast/ddl.rs:1203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Option for `ALTER OPERATOR SET` operation

<a id="op-787e8d3e898eea9c510ab351"></a>
## Commutator

`variant` · `sqlparser::ast::ddl::OperatorOption::Commutator` · sqlparser 0.62.0

```rust
Commutator
```

Source: `src/ast/ddl.rs:1209`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`COMMUTATOR = com_op`

<a id="op-e848a3fb6db555b214eea758"></a>
## Hashes

`variant` · `sqlparser::ast::ddl::OperatorOption::Hashes` · sqlparser 0.62.0

```rust
Hashes
```

Source: `src/ast/ddl.rs:1213`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`HASHES`

<a id="op-fdd47242aafdf4f8b27213de"></a>
## Join

`variant` · `sqlparser::ast::ddl::OperatorOption::Join` · sqlparser 0.62.0

```rust
Join
```

Source: `src/ast/ddl.rs:1207`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`JOIN = { join_proc | NONE }`

<a id="op-9b4c1cf3e682c4caeb4457cc"></a>
## Merges

`variant` · `sqlparser::ast::ddl::OperatorOption::Merges` · sqlparser 0.62.0

```rust
Merges
```

Source: `src/ast/ddl.rs:1215`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MERGES`

<a id="op-727f3715f3027aac51176743"></a>
## Negator

`variant` · `sqlparser::ast::ddl::OperatorOption::Negator` · sqlparser 0.62.0

```rust
Negator
```

Source: `src/ast/ddl.rs:1211`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`NEGATOR = neg_op`

<a id="op-9561a720b9843fbc1f4d8f43"></a>
## Restrict

`variant` · `sqlparser::ast::ddl::OperatorOption::Restrict` · sqlparser 0.62.0

```rust
Restrict
```

Source: `src/ast/ddl.rs:1205`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`RESTRICT = { res_proc | NONE }`

<a id="op-8300af0ef01e691866a6fa9a"></a>
## clone

`function` · `sqlparser::ast::ddl::OperatorOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OperatorOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 17], "end": [1200, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d53864bb515e03fa4703071c"></a>
## cmp

`function` · `sqlparser::ast::ddl::OperatorOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OperatorOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 51], "end": [1200, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2003aedfc70ca52970b784de"></a>
## deserialize

`function` · `sqlparser::ast::ddl::OperatorOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 49], "end": [1201, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d42951dd47afb133a21eecfd"></a>
## eq

`function` · `sqlparser::ast::ddl::OperatorOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OperatorOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 24], "end": [1200, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60f42ebfba88e9800e83d4b5"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1249, 1], "end": [1262, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1250`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a94a3ff7fdea14073f19f9aa"></a>
## fmt

`function` · `sqlparser::ast::ddl::OperatorOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 10], "end": [1200, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc4d5c2f45fa774052e8fd6b"></a>
## hash

`function` · `sqlparser::ast::ddl::OperatorOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 56], "end": [1200, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cea3b9243154960d4ff917c"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::OperatorOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OperatorOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1200, 35], "end": [1200, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92d22bb16d61ffad4e7fd066"></a>
## serialize

`function` · `sqlparser::ast::ddl::OperatorOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1201, 38], "end": [1201, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1201`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d61cbc644fabe29254aaeef"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1202, 40], "end": [1202, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2972674fd020bb92a01d361"></a>
## visit

`function` · `sqlparser::ast::ddl::OperatorOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::OperatorOption", "path": "OperatorOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1202, 47], "end": [1202, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
