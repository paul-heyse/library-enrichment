# `sqlparser::ast::ddl::UserDefinedTypeStorage`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeStorage.json).

<a id="op-5498091593ecc79345ff7eff"></a>
## UserDefinedTypeStorage

`enum` · `sqlparser::ast::ddl::UserDefinedTypeStorage` · sqlparser 0.62.0

```rust
enum UserDefinedTypeStorage
```

Source: `src/ast/ddl.rs:2544`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Storage specification for PostgreSQL user-defined base types.

Specifies the storage strategy for values of the data type:
- `plain`: Prevents compression and out-of-line storage (for fixed-length types)
- `external`: Allows out-of-line storage but not compression
- `extended`: Allows both compression and out-of-line storage (default for most types)
- `main`: Allows compression but discourages out-of-line storage

# PostgreSQL Documentation
See: <https://www.postgresql.org/docs/current/sql-createtype.html>

# Examples
```sql
CREATE TYPE mytype (
    INPUT = in_func,
    OUTPUT = out_func,
    STORAGE = plain
);
```

<a id="op-573fc2a12d3cfcc4b197f8ea"></a>
## Extended

`variant` · `sqlparser::ast::ddl::UserDefinedTypeStorage::Extended` · sqlparser 0.62.0

```rust
Extended
```

Source: `src/ast/ddl.rs:2550`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Both compression and out-of-line storage allowed: `STORAGE = extended`

<a id="op-47728b9bdb9433589262ee81"></a>
## External

`variant` · `sqlparser::ast::ddl::UserDefinedTypeStorage::External` · sqlparser 0.62.0

```rust
External
```

Source: `src/ast/ddl.rs:2548`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Out-of-line storage allowed, no compression: `STORAGE = external`

<a id="op-4e54d4f94ed7638fa93d88f9"></a>
## Main

`variant` · `sqlparser::ast::ddl::UserDefinedTypeStorage::Main` · sqlparser 0.62.0

```rust
Main
```

Source: `src/ast/ddl.rs:2552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Compression allowed, out-of-line discouraged: `STORAGE = main`

<a id="op-f61c7c87e189337fe3ae744f"></a>
## Plain

`variant` · `sqlparser::ast::ddl::UserDefinedTypeStorage::Plain` · sqlparser 0.62.0

```rust
Plain
```

Source: `src/ast/ddl.rs:2546`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No compression or out-of-line storage: `STORAGE = plain`

<a id="op-5f29e23dd882ab32dd03e8c5"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeStorage
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 23], "end": [2541, 28], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d471350e3e52c757b97675d5"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeStorage) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 57], "end": [2541, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506aa0293d21a34233478c9a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2542, 49], "end": [2542, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4685eea5cc40f6a50814210"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeStorage) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 30], "end": [2541, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed21e3d1df85af64c1dd2081"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2555, 1], "end": [2564, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2556`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f168d49f80d5cce4f8b4b34f"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 10], "end": [2541, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dba10a94bc2ef1ead9f8d2b2"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 62], "end": [2541, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a21cd8102a31b29b108e98a5"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeStorage) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2541, 41], "end": [2541, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54b28c78bd2be6cc96e02b51"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2542, 38], "end": [2542, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0a7e62afba06964a9bc87516"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 47], "end": [2543, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-441e63e0dee548374ad4d6e8"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeStorage::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeStorage", "path": "UserDefinedTypeStorage"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2543, 40], "end": [2543, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2543`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
