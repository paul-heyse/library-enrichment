# `sqlparser::ast::CreateTableOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateTableOptions.json).

<a id="op-3c85365733a2d1d7c8a8044e"></a>
## CreateTableOptions

`enum` · `sqlparser::ast::CreateTableOptions` · sqlparser 0.62.0

```rust
enum CreateTableOptions
```

Source: `src/ast/mod.rs:3178`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Sql options of a `CREATE TABLE` statement.
Options allowed within a `CREATE TABLE` statement.

<a id="op-0e6e297ba5e08cf23d1c563e"></a>
## None

`variant` · `sqlparser::ast::CreateTableOptions::None` · sqlparser 0.62.0

```rust
None
```

Source: `src/ast/mod.rs:3181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No options specified.

<a id="op-f88e5c4a373c1a79ffce3de2"></a>
## Options

`variant` · `sqlparser::ast::CreateTableOptions::Options` · sqlparser 0.62.0

```rust
Options
```

Source: `src/ast/mod.rs:3185`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options specified using the `OPTIONS(...)` clause.

<a id="op-e11ea1521058d0067763c144"></a>
## Plain

`variant` · `sqlparser::ast::CreateTableOptions::Plain` · sqlparser 0.62.0

```rust
Plain
```

Source: `src/ast/mod.rs:3187`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Plain space-separated options.

<a id="op-74e82413b7b1c2fc5bb33119"></a>
## TableProperties

`variant` · `sqlparser::ast::CreateTableOptions::TableProperties` · sqlparser 0.62.0

```rust
TableProperties
```

Source: `src/ast/mod.rs:3189`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table properties (e.g., TBLPROPERTIES / storage properties).

<a id="op-15f8d74a3d5aa6abb156279b"></a>
## With

`variant` · `sqlparser::ast::CreateTableOptions::With` · sqlparser 0.62.0

```rust
With
```

Source: `src/ast/mod.rs:3183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options specified using the `WITH` keyword, e.g. `WITH (k = v)`.

<a id="op-80f837c9a10ddc816291abf5"></a>
## clone

`function` · `sqlparser::ast::CreateTableOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 26], "end": [3174, 31], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd6bae6ba4fa6ba6d25c778e"></a>
## cmp

`function` · `sqlparser::ast::CreateTableOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTableOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 60], "end": [3174, 63], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-48e80c6387cc27645f660ce2"></a>
## default

`function` · `sqlparser::ast::CreateTableOptions::default` · sqlparser 0.62.0

```rust
fn default() -> CreateTableOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 17], "end": [3174, 24], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7d6ce624b6131dfd53dccd8"></a>
## deserialize

`function` · `sqlparser::ast::CreateTableOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 49], "end": [3175, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c65c746d702cc9d21f3bc065"></a>
## eq

`function` · `sqlparser::ast::CreateTableOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTableOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 33], "end": [3174, 42], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-980695ed2a99ca22cd957c46"></a>
## fmt

`function` · `sqlparser::ast::CreateTableOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 10], "end": [3174, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f4d103a094e5ccd099b2c8d"></a>
## fmt

`function` · `sqlparser::ast::CreateTableOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3192, 1], "end": [3210, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3193`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d48ef87ea4931625f5b43496"></a>
## hash

`function` · `sqlparser::ast::CreateTableOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 65], "end": [3174, 69], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf175807b3cb2b87c99a466b"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateTableOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTableOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3174, 44], "end": [3174, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c5d9a1051ac16e47016b73"></a>
## serialize

`function` · `sqlparser::ast::CreateTableOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 38], "end": [3175, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3175`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59ab54c9d0c7cabf03d4737d"></a>
## span

`function` · `sqlparser::ast::CreateTableOptions::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "super::CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1077, 1], "end": [1089, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1078`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f2bad0314683210bbdd2280"></a>
## visit

`function` · `sqlparser::ast::CreateTableOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3176, 40], "end": [3176, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db2ca5e1eb99ab4be52b83c3"></a>
## visit

`function` · `sqlparser::ast::CreateTableOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableOptions", "path": "CreateTableOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3176, 47], "end": [3176, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3176`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
