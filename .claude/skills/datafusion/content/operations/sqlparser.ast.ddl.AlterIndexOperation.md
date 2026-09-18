# `sqlparser::ast::ddl::AlterIndexOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterIndexOperation.json).

<a id="op-9e6104e643ca420b0e43a3eb"></a>
## AlterIndexOperation

`enum` · `sqlparser::ast::ddl::AlterIndexOperation` · sqlparser 0.62.0

```rust
enum AlterIndexOperation
```

Source: `src/ast/ddl.rs:696`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Alterations that can be applied to an index.

<a id="op-225e0b80010dabc4de54d890"></a>
## RenameIndex

`variant` · `sqlparser::ast::ddl::AlterIndexOperation::RenameIndex` · sqlparser 0.62.0

```rust
RenameIndex
```

Source: `src/ast/ddl.rs:698`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename the index to `index_name`.

<a id="op-f3256bb545fb131dd45ce7b1"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterIndexOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterIndexOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 17], "end": [692, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e6e4b0110bb65dc2ccdfb66"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterIndexOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterIndexOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 51], "end": [692, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdfb1d207d47e701e45c9ed5"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterIndexOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [693, 49], "end": [693, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e0595c1ce6152af7b4fffd"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterIndexOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterIndexOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 24], "end": [692, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dda53f4a25b7bf1dc4d403f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterIndexOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 10], "end": [692, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5d1f3de84ea09ab65d7dc7f"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterIndexOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1051, 1], "end": [1059, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6fe1abed018e84de9eb85de8"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterIndexOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 56], "end": [692, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc21becf125e4ab3b152c002"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterIndexOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterIndexOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [692, 35], "end": [692, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c978adbae47ae3094ef5710f"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterIndexOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [693, 38], "end": [693, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:693`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3246cd9562f8a3d6a3cb799d"></a>
## span

`function` · `sqlparser::ast::ddl::AlterIndexOperation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "super::AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1314, 1], "end": [1320, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b9f3f7ffb5dbaa25ec62426"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterIndexOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [694, 47], "end": [694, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b2a71234c4b93a426747ff3"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterIndexOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterIndexOperation", "path": "AlterIndexOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [694, 40], "end": [694, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:694`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
