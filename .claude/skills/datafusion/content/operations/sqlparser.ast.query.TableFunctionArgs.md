# `sqlparser::ast::query::TableFunctionArgs`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.TableFunctionArgs.json).

<a id="op-3ba5b8f4c635df8ebfec064f"></a>
## TableFunctionArgs

`struct` · `sqlparser::ast::query::TableFunctionArgs` · sqlparser 0.62.0

```rust
struct TableFunctionArgs
```

Source: `src/ast/query.rs:1357`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Arguments to a table-valued function

<a id="op-e39ae20a5505a6d311e21c05"></a>
## args

`struct_field` · `sqlparser::ast::query::TableFunctionArgs::args` · sqlparser 0.62.0

```rust
args: Vec<FunctionArg>
```

Source: `src/ast/query.rs:1359`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of arguments passed to the table-valued function.

<a id="op-34d16b46b21ce35473b9c399"></a>
## clone

`function` · `sqlparser::ast::query::TableFunctionArgs::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableFunctionArgs
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 17], "end": [1354, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f681e918752debe7495cc95"></a>
## cmp

`function` · `sqlparser::ast::query::TableFunctionArgs::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableFunctionArgs) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 51], "end": [1354, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2633570b7b2d35f8199f3c23"></a>
## deserialize

`function` · `sqlparser::ast::query::TableFunctionArgs::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1355, 49], "end": [1355, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:1355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e511ec6cb29562e5b6c94850"></a>
## eq

`function` · `sqlparser::ast::query::TableFunctionArgs::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableFunctionArgs) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 24], "end": [1354, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c14179558663a33c064df69"></a>
## fmt

`function` · `sqlparser::ast::query::TableFunctionArgs::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 10], "end": [1354, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-955a5cbbc2e38dacc382c308"></a>
## hash

`function` · `sqlparser::ast::query::TableFunctionArgs::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 56], "end": [1354, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8abcd552198256d380356de"></a>
## partial_cmp

`function` · `sqlparser::ast::query::TableFunctionArgs::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableFunctionArgs) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1354, 35], "end": [1354, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:1354`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3476b92d963912b27ed7fe1b"></a>
## serialize

`function` · `sqlparser::ast::query::TableFunctionArgs::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1355, 38], "end": [1355, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:1355`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45d142a01d344c82063aa29f"></a>
## settings

`struct_field` · `sqlparser::ast::query::TableFunctionArgs::settings` · sqlparser 0.62.0

```rust
settings: Option<Vec<Setting>>
```

Source: `src/ast/query.rs:1364`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse-specific `SETTINGS` clause.
For example,
`SELECT * FROM executable('generate_random.py', TabSeparated, 'id UInt32, random String', SETTINGS send_chunk_header = false, pool_size = 16)`
[`executable` table function](https://clickhouse.com/docs/en/engines/table-functions/executable)

<a id="op-7fdf337b87aa187b4f184f67"></a>
## visit

`function` · `sqlparser::ast::query::TableFunctionArgs::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1356, 47], "end": [1356, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:1356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a35e319e9902ae850dd74e7c"></a>
## visit

`function` · `sqlparser::ast::query::TableFunctionArgs::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::TableFunctionArgs", "path": "TableFunctionArgs"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1356, 40], "end": [1356, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:1356`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
