# `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.helpers.stmt_data_loading.FileStagingCommand.json).

<a id="op-87d2704f1b6e37e4d460f753"></a>
## FileStagingCommand

`struct` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand` · sqlparser 0.62.0

```rust
struct FileStagingCommand
```

Source: `src/ast/helpers/stmt_data_loading.rs:133`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A command to stage files to a named stage.

<a id="op-b86634885fd39222cd68b2c3"></a>
## clone

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FileStagingCommand
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 17], "end": [129, 22], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d511d05473be9b565509d66a"></a>
## cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FileStagingCommand) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 51], "end": [129, 54], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-517cecf4e463590be4d12e77"></a>
## deserialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 49], "end": [130, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cacff40ca9afe67e4bed24df"></a>
## eq

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FileStagingCommand) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 24], "end": [129, 33], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ecdfc47f1e1f0d4d8333565c"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [141, 1], "end": [149, 2], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/helpers/stmt_data_loading.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ece3db7deae4659b0cd50f49"></a>
## fmt

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 10], "end": [129, 15], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24185935a7b50ae5c7c6ef52"></a>
## hash

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 56], "end": [129, 60], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2136ebeece7f60c7da6dad6d"></a>
## partial_cmp

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FileStagingCommand) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [129, 35], "end": [129, 45], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/helpers/stmt_data_loading.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29d47dbfb7c6d25c9dd83f6a"></a>
## pattern

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::pattern` · sqlparser 0.62.0

```rust
pattern: Option<String>
```

Source: `src/ast/helpers/stmt_data_loading.rs:138`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional file matching `PATTERN` expression.

<a id="op-2347a6ffc8ac09890a291bd9"></a>
## serialize

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [130, 38], "end": [130, 47], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/helpers/stmt_data_loading.rs:130`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57401eae9560b809ee4e8220"></a>
## stage

`struct_field` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::stage` · sqlparser 0.62.0

```rust
stage: ast::ObjectName
```

Source: `src/ast/helpers/stmt_data_loading.rs:136`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The stage to which files are being staged.

<a id="op-8b3eb979256e2ae31059d3b2"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 47], "end": [131, 55], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/helpers/stmt_data_loading.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e61dad7d81b11a92e8a41c9"></a>
## visit

`function` · `sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand", "path": "FileStagingCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 40], "end": [131, 45], "filename": "src/ast/helpers/stmt_data_loading.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/helpers/stmt_data_loading.rs:131`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
