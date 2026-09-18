# `sqlparser::ast::ddl::CreateCollationDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateCollationDefinition.json).

<a id="op-897449bb3b336f5fe2760d35"></a>
## CreateCollationDefinition

`enum` · `sqlparser::ast::ddl::CreateCollationDefinition` · sqlparser 0.62.0

```rust
enum CreateCollationDefinition
```

Source: `src/ast/ddl.rs:4525`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Definition forms supported by `CREATE COLLATION`.

<a id="op-539c77d2c6fe938db706a534"></a>
## From

`variant` · `sqlparser::ast::ddl::CreateCollationDefinition::From` · sqlparser 0.62.0

```rust
From
```

Source: `src/ast/ddl.rs:4531`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create from an existing collation.

```sql
CREATE COLLATION name FROM existing_collation
```

<a id="op-394a3da6f9ad8eb3da302171"></a>
## Options

`variant` · `sqlparser::ast::ddl::CreateCollationDefinition::Options` · sqlparser 0.62.0

```rust
Options
```

Source: `src/ast/ddl.rs:4537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create with an option list.

```sql
CREATE COLLATION name (key = value, ...)
```

<a id="op-d46afd2d6361582d101e451a"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateCollationDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 17], "end": [4522, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-585b361ae141a9cec0111af4"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateCollationDefinition) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 51], "end": [4522, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa0417d2baf11b8e8cff11f1"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4523, 49], "end": [4523, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e76943488b5e9483697cf39"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateCollationDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 24], "end": [4522, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bedeb6485b062c02d215d923"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 10], "end": [4522, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d966c7fc7c54bac7f9a5057"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 56], "end": [4522, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-053c86786743138aed02a855"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateCollationDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4522, 35], "end": [4522, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b7272b3231e2b5834f48a76"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4523, 38], "end": [4523, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4523`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32161e86085d3a67f66310f8"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4524, 40], "end": [4524, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a39a4772207568f31884733"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateCollationDefinition::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollationDefinition", "path": "CreateCollationDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4524, 47], "end": [4524, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
