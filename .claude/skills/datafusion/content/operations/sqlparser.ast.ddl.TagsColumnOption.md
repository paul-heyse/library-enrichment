# `sqlparser::ast::ddl::TagsColumnOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.TagsColumnOption.json).

<a id="op-c80e8c086d31f726c4dec2f3"></a>
## TagsColumnOption

`struct` · `sqlparser::ast::ddl::TagsColumnOption` · sqlparser 0.62.0

```rust
struct TagsColumnOption
```

Source: `src/ast/ddl.rs:1867`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tags option of column
Syntax
```sql
[ WITH ] TAG ( <tag_name> = '<tag_value>' [ , <tag_name> = '<tag_value>' , ... ] )
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-40255e3032666c906f6293f7"></a>
## clone

`function` · `sqlparser::ast::ddl::TagsColumnOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TagsColumnOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 17], "end": [1864, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-498aabc6f6bd03ba8c9b2121"></a>
## cmp

`function` · `sqlparser::ast::ddl::TagsColumnOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TagsColumnOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 51], "end": [1864, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7cc74768cdb2b6b9f2d334d0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::TagsColumnOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 49], "end": [1865, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31bc65908483f2b87bd6dc93"></a>
## eq

`function` · `sqlparser::ast::ddl::TagsColumnOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TagsColumnOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 24], "end": [1864, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d913d69f5cdfa4c8743cd85"></a>
## fmt

`function` · `sqlparser::ast::ddl::TagsColumnOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 10], "end": [1864, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-641d6e1bf6975ff51809c316"></a>
## fmt

`function` · `sqlparser::ast::ddl::TagsColumnOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1879, 1], "end": [1887, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fddd116a2c5f5a39b0af0b0b"></a>
## hash

`function` · `sqlparser::ast::ddl::TagsColumnOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 56], "end": [1864, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8f852a6a9b947c59ecfdbe2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::TagsColumnOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TagsColumnOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1864, 35], "end": [1864, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1864`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-271dd396b8f6a7ec9471cd83"></a>
## serialize

`function` · `sqlparser::ast::ddl::TagsColumnOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1865, 38], "end": [1865, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1865`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7410989fe9b68e0420dae0f0"></a>
## tags

`struct_field` · `sqlparser::ast::ddl::TagsColumnOption::tags` · sqlparser 0.62.0

```rust
tags: Vec<ast::Tag>
```

Source: `src/ast/ddl.rs:1876`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

List of tags to attach to the column.

<a id="op-59fdc3670bb5ab7ff9014d0e"></a>
## visit

`function` · `sqlparser::ast::ddl::TagsColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1866, 47], "end": [1866, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a707cb930eecd7c8ac9731b5"></a>
## visit

`function` · `sqlparser::ast::ddl::TagsColumnOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::TagsColumnOption", "path": "TagsColumnOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1866, 40], "end": [1866, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1866`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b3935763d335b4ef26821c2"></a>
## with

`struct_field` · `sqlparser::ast::ddl::TagsColumnOption::with` · sqlparser 0.62.0

```rust
with: bool
```

Source: `src/ast/ddl.rs:1874`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

This flag indicates that the tags option is declared using the `WITH` prefix.
Example:
```sql
WITH TAG (A = 'Tag A')
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table
