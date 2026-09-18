# `sqlparser::ast::ddl::AlterCollationOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterCollationOperation.json).

<a id="op-6e8b39b062eb6bc84862e849"></a>
## AlterCollationOperation

`enum` · `sqlparser::ast::ddl::AlterCollationOperation` · sqlparser 0.62.0

```rust
enum AlterCollationOperation
```

Source: `src/ast/ddl.rs:4585`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operations supported by `ALTER COLLATION`.

<a id="op-4ab281b144598581599eda41"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterCollationOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:4600`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Change the collation owner.

```sql
ALTER COLLATION name OWNER TO role_name
```

<a id="op-9dc175dbd700c5252071ff1d"></a>
## RefreshVersion

`variant` · `sqlparser::ast::ddl::AlterCollationOperation::RefreshVersion` · sqlparser 0.62.0

```rust
RefreshVersion
```

Source: `src/ast/ddl.rs:4615`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Refresh collation version metadata.

```sql
ALTER COLLATION name REFRESH VERSION
```

<a id="op-08b108fd62f8409f7714cb5d"></a>
## RenameTo

`variant` · `sqlparser::ast::ddl::AlterCollationOperation::RenameTo` · sqlparser 0.62.0

```rust
RenameTo
```

Source: `src/ast/ddl.rs:4591`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename the collation.

```sql
ALTER COLLATION name RENAME TO new_name
```

<a id="op-003ba729195718aad3fac78e"></a>
## SetSchema

`variant` · `sqlparser::ast::ddl::AlterCollationOperation::SetSchema` · sqlparser 0.62.0

```rust
SetSchema
```

Source: `src/ast/ddl.rs:4606`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Move the collation to another schema.

```sql
ALTER COLLATION name SET SCHEMA new_schema
```

<a id="op-3dce38ed19f9a64af6f2401c"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterCollationOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterCollationOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 17], "end": [4582, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97f43a139aef908382ff4032"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterCollationOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterCollationOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 51], "end": [4582, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9433c90c2bc39475e32328d9"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterCollationOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4583, 49], "end": [4583, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a64cdc27131d13d771b9950"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterCollationOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterCollationOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 24], "end": [4582, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43a76d089914db9d57e6e5aa"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterCollationOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4618, 1], "end": [4629, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4619`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9547348c0f02dcc66eabe512"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterCollationOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 10], "end": [4582, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92dcee15e821551774f56a47"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterCollationOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 56], "end": [4582, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22debb0da2ed098d6eda4880"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterCollationOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterCollationOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4582, 35], "end": [4582, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99c55189d035b35ff914bc79"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterCollationOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4583, 38], "end": [4583, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4583`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6916750d4256935901d8efb7"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterCollationOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4584, 40], "end": [4584, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b568554301238041f7e4987c"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterCollationOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterCollationOperation", "path": "AlterCollationOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4584, 47], "end": [4584, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
