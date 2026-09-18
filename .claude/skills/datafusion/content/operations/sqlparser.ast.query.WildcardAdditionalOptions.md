# `sqlparser::ast::query::WildcardAdditionalOptions`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.WildcardAdditionalOptions.json).

<a id="op-35c8794d2c31ce63bada0f67"></a>
## WildcardAdditionalOptions

`struct` · `sqlparser::ast::query::WildcardAdditionalOptions` · sqlparser 0.62.0

```rust
struct WildcardAdditionalOptions
```

Source: `src/ast/query.rs:928`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

<a id="op-d0e2abed3b001997036bd6e4"></a>
## clone

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WildcardAdditionalOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 17], "end": [925, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0263f87a794439ecea4886c2"></a>
## cmp

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WildcardAdditionalOptions) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 51], "end": [925, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb5542fffbd09d6166853e9e"></a>
## default

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::default` · sqlparser 0.62.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [951, 1], "end": [963, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/ast/query.rs:952`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27d70823ebfbc7e3ed0cd0aa"></a>
## deserialize

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 49], "end": [926, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79e0ddb493fd486963564e07"></a>
## eq

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WildcardAdditionalOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 24], "end": [925, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f946021cce6ac533821501e"></a>
## fmt

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 10], "end": [925, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb1e9a5fd1932515e1afa102"></a>
## fmt

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [965, 1], "end": [987, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45c3ac413836389fe4b4125c"></a>
## hash

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 56], "end": [925, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea196e0a8294ea5cd53bd26c"></a>
## opt_alias

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_alias` · sqlparser 0.62.0

```rust
opt_alias: Option<Ident>
```

Source: `src/ast/query.rs:948`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[AS <alias>]`.
 Redshift syntax: <https://docs.aws.amazon.com/redshift/latest/dg/r_SELECT_list.html>

<a id="op-fbdd91d6f58b58e2b564901d"></a>
## opt_except

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_except` · sqlparser 0.62.0

```rust
opt_except: Option<ExceptSelectItem>
```

Source: `src/ast/query.rs:938`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[EXCEPT...]`.
 Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#except>

<a id="op-6ef9aba4761407d82351c212"></a>
## opt_exclude

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_exclude` · sqlparser 0.62.0

```rust
opt_exclude: Option<ExcludeSelectItem>
```

Source: `src/ast/query.rs:935`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[EXCLUDE...]`.

<a id="op-a4e7e2989fe074f302415cad"></a>
## opt_ilike

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_ilike` · sqlparser 0.62.0

```rust
opt_ilike: Option<IlikeSelectItem>
```

Source: `src/ast/query.rs:933`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ILIKE...]`.
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-197cbbdae57e7fef8c99f180"></a>
## opt_rename

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_rename` · sqlparser 0.62.0

```rust
opt_rename: Option<RenameSelectItem>
```

Source: `src/ast/query.rs:945`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[RENAME ...]`.

<a id="op-ff04a3d9edfbb5fe7d7f6a5d"></a>
## opt_replace

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::opt_replace` · sqlparser 0.62.0

```rust
opt_replace: Option<ReplaceSelectItem>
```

Source: `src/ast/query.rs:943`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[REPLACE]`
 BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace>
 Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#replace>
 Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters>

<a id="op-7ccd0007e43a7ba43133efc6"></a>
## partial_cmp

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WildcardAdditionalOptions) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [925, 35], "end": [925, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:925`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bbb98c4d4fc2cceb5d77c0b"></a>
## serialize

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [926, 38], "end": [926, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:926`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57a22d7c0778aaee39f13b9a"></a>
## span

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "super::WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1853, 1], "end": [1875, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:1854`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dee63acbd649694bc569185a"></a>
## visit

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [927, 40], "end": [927, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eabf8745018511928fe3a81f"></a>
## visit

`function` · `sqlparser::ast::query::WildcardAdditionalOptions::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::WildcardAdditionalOptions", "path": "WildcardAdditionalOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [927, 47], "end": [927, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:927`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f107460e5b5de535a588b1cd"></a>
## wildcard_token

`struct_field` · `sqlparser::ast::query::WildcardAdditionalOptions::wildcard_token` · sqlparser 0.62.0

```rust
wildcard_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:930`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The wildcard token `*`
