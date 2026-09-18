# `sqlparser::ast::table_constraints::TableConstraint`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.table_constraints.TableConstraint.json).

<a id="op-6cf7ad3c767f63a7e0b2a59a"></a>
## TableConstraint

`enum` · `sqlparser::ast::table_constraints::TableConstraint` · sqlparser 0.62.0

```rust
enum TableConstraint
```

Source: `src/ast/table_constraints.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A table-level constraint, specified in a `CREATE TABLE` or an
`ALTER TABLE ADD <constraint>` statement.

<a id="op-8e9addc4a37a0cb999206a2e"></a>
## Check

`variant` · `sqlparser::ast::table_constraints::TableConstraint::Check` · sqlparser 0.62.0

```rust
Check
```

Source: `src/ast/table_constraints.rs:82`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`[ CONSTRAINT <name> ] CHECK (<expr>) [[NOT] ENFORCED]`

<a id="op-8237f52bd1c471fd3b1be957"></a>
## ForeignKey

`variant` · `sqlparser::ast::table_constraints::TableConstraint::ForeignKey` · sqlparser 0.62.0

```rust
ForeignKey
```

Source: `src/ast/table_constraints.rs:80`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A referential integrity constraint (`[ CONSTRAINT <name> ] FOREIGN KEY (<columns>)
REFERENCES <foreign_table> (<referred_columns>)
{ [ON DELETE <referential_action>] [ON UPDATE <referential_action>] |
  [ON UPDATE <referential_action>] [ON DELETE <referential_action>]
}`).

<a id="op-de8a7dcbb4eff01da37cc90a"></a>
## FulltextOrSpatial

`variant` · `sqlparser::ast::table_constraints::TableConstraint::FulltextOrSpatial` · sqlparser 0.62.0

```rust
FulltextOrSpatial
```

Source: `src/ast/table_constraints.rs:103`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQLs [fulltext][1] definition. Since the [`SPATIAL`][2] definition is exactly the same,
and MySQL displays both the same way, it is part of this definition as well.

Supported syntax:

```markdown
{FULLTEXT | SPATIAL} [INDEX | KEY] [index_name] (key_part,...)

key_part: col_name
```

[1]: https://dev.mysql.com/doc/refman/8.0/en/fulltext-natural-language.html
[2]: https://dev.mysql.com/doc/refman/8.0/en/spatial-types.html

<a id="op-019df1b0a62c18847f79851b"></a>
## Index

`variant` · `sqlparser::ast::table_constraints::TableConstraint::Index` · sqlparser 0.62.0

```rust
Index
```

Source: `src/ast/table_constraints.rs:89`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQLs [index definition][1] for index creation. Not present on ANSI so, for now, the usage
is restricted to MySQL, as no other dialects that support this syntax were found.

`{INDEX | KEY} [index_name] [index_type] (key_part,...) [index_option]...`

[1]: https://dev.mysql.com/doc/refman/8.0/en/create-table.html

<a id="op-7583c467af92e3878542af47"></a>
## PrimaryKey

`variant` · `sqlparser::ast::table_constraints::TableConstraint::PrimaryKey` · sqlparser 0.62.0

```rust
PrimaryKey
```

Source: `src/ast/table_constraints.rs:74`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL [definition][1] for `PRIMARY KEY` constraints statements:\
* `[CONSTRAINT [<name>]] PRIMARY KEY [index_name] [index_type] (<columns>) <index_options>`

Actually the specification have no `[index_name]` but the next query will complete successfully:
```sql
CREATE TABLE unspec_table (
  xid INT NOT NULL,
  CONSTRAINT p_name PRIMARY KEY index_name USING BTREE (xid)
);
```

where:
* [index_type][2] is `USING {BTREE | HASH}`
* [index_options][3] is `{index_type | COMMENT 'string' | ... %currently unsupported stmts% } ...`

[1]: https://dev.mysql.com/doc/refman/8.3/en/create-table.html
[2]: IndexType
[3]: IndexOption

<a id="op-d4f23340a90bcbf603ee4ca5"></a>
## PrimaryKeyUsingIndex

`variant` · `sqlparser::ast::table_constraints::TableConstraint::PrimaryKeyUsingIndex` · sqlparser 0.62.0

```rust
PrimaryKeyUsingIndex
```

Source: `src/ast/table_constraints.rs:111`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL [definition][1] for promoting an existing unique index to a
`PRIMARY KEY` constraint:

`[ CONSTRAINT constraint_name ] PRIMARY KEY USING INDEX index_name
  [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`

[1]: https://www.postgresql.org/docs/current/sql-altertable.html

<a id="op-22483494bbc71e2a5ffa9d43"></a>
## Unique

`variant` · `sqlparser::ast::table_constraints::TableConstraint::Unique` · sqlparser 0.62.0

```rust
Unique
```

Source: `src/ast/table_constraints.rs:55`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL [definition][1] for `UNIQUE` constraints statements:\
* `[CONSTRAINT [<name>]] UNIQUE <index_type_display> [<index_name>] [index_type] (<columns>) <index_options>`

where:
* [index_type][2] is `USING {BTREE | HASH}`
* [index_options][3] is `{index_type | COMMENT 'string' | ... %currently unsupported stmts% } ...`
* [index_type_display][4] is `[INDEX | KEY]`

[1]: https://dev.mysql.com/doc/refman/8.3/en/create-table.html
[2]: IndexType
[3]: IndexOption
[4]: KeyOrIndexDisplay

<a id="op-f2de40270e55df5c431f5284"></a>
## UniqueUsingIndex

`variant` · `sqlparser::ast::table_constraints::TableConstraint::UniqueUsingIndex` · sqlparser 0.62.0

```rust
UniqueUsingIndex
```

Source: `src/ast/table_constraints.rs:119`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL [definition][1] for promoting an existing unique index to a
`UNIQUE` constraint:

`[ CONSTRAINT constraint_name ] UNIQUE USING INDEX index_name
  [ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`

[1]: https://www.postgresql.org/docs/current/sql-altertable.html

<a id="op-821245b85e8b49291b29a39a"></a>
## clone

`function` · `sqlparser::ast::table_constraints::TableConstraint::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> TableConstraint
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 17], "end": [39, 22], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4cef4964f5dab186facf676"></a>
## cmp

`function` · `sqlparser::ast::table_constraints::TableConstraint::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &TableConstraint) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 51], "end": [39, 54], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfc7c5a07a49ed28ef157c3a"></a>
## deserialize

`function` · `sqlparser::ast::table_constraints::TableConstraint::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 49], "end": [40, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/table_constraints.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3a028983068941c953b190c"></a>
## eq

`function` · `sqlparser::ast::table_constraints::TableConstraint::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &TableConstraint) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 24], "end": [39, 33], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95f754cc422ad03caa9916e7"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::TableConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 10], "end": [39, 15], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4c1e484faf96afa2e415100"></a>
## fmt

`function` · `sqlparser::ast::table_constraints::TableConstraint::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [158, 1], "end": [171, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/table_constraints.rs:159`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4befa42a47d4b2473137d10a"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: PrimaryKeyConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [132, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::PrimaryKeyConstraint", "path": "PrimaryKeyConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:129`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f37ee5257ffe1083b3801e8"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: CheckConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [140, 1], "end": [144, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::CheckConstraint", "path": "CheckConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:141`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-849077a02018417568e8b894"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: IndexConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [146, 1], "end": [150, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::IndexConstraint", "path": "IndexConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e385be664df5a40881e2b55b"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: ForeignKeyConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [134, 1], "end": [138, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::ForeignKeyConstraint", "path": "ForeignKeyConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:135`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3ef7e46d0d48d8d611f65e5"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: FullTextOrSpatialConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [152, 1], "end": [156, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::FullTextOrSpatialConstraint", "path": "FullTextOrSpatialConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f92d0acf24414b8f2f5151ea"></a>
## from

`function` · `sqlparser::ast::table_constraints::TableConstraint::from` · sqlparser 0.62.0

```rust
fn from(constraint: UniqueConstraint) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [126, 2], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::UniqueConstraint", "path": "UniqueConstraint"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/ast/table_constraints.rs:123`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb4bf8d476810bf0dd2045a4"></a>
## hash

`function` · `sqlparser::ast::table_constraints::TableConstraint::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 56], "end": [39, 60], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4febe7686d20891865cbbea5"></a>
## partial_cmp

`function` · `sqlparser::ast::table_constraints::TableConstraint::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &TableConstraint) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 35], "end": [39, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/table_constraints.rs:39`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-569218d892738dd1e62d2437"></a>
## serialize

`function` · `sqlparser::ast::table_constraints::TableConstraint::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 38], "end": [40, 47], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/table_constraints.rs:40`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-850543a5c3c74447db9ce378"></a>
## span

`function` · `sqlparser::ast::table_constraints::TableConstraint::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "super::TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [642, 1], "end": [655, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d37fd7fa12e80562d936c8e"></a>
## visit

`function` · `sqlparser::ast::table_constraints::TableConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 47], "end": [41, 55], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/table_constraints.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb30e654f2a210bbe9fd06b7"></a>
## visit

`function` · `sqlparser::ast::table_constraints::TableConstraint::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::table_constraints::TableConstraint", "path": "TableConstraint"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 40], "end": [41, 45], "filename": "src/ast/table_constraints.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/table_constraints.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
