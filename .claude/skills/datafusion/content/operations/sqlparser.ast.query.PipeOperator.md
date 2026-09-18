# `sqlparser::ast::query::PipeOperator`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.PipeOperator.json).

<a id="op-6772928293469f7683a00a4d"></a>
## PipeOperator

`enum` · `sqlparser::ast::query::PipeOperator` · sqlparser 0.62.0

```rust
enum PipeOperator
```

Source: `src/ast/query.rs:3157`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pipe syntax, first introduced in Google BigQuery.
Example:

```sql
FROM Produce
|> WHERE sales > 0
|> AGGREGATE SUM(sales) AS total_sales, COUNT(*) AS num_sales
   GROUP BY item;
```

See <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#pipe_syntax>

<a id="op-a53080fb518a3724e773171d"></a>
## Aggregate

`variant` · `sqlparser::ast::query::PipeOperator::Aggregate` · sqlparser 0.62.0

```rust
Aggregate
```

Source: `src/ast/query.rs:3239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Performs aggregation on data across grouped rows or an entire table.

Syntax: `|> AGGREGATE <agg_expr> [[AS] alias], ...`

Syntax:
```norust
|> AGGREGATE [<agg_expr> [[AS] alias], ...]
GROUP BY <grouping_expr> [AS alias], ...
```

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#aggregate_pipe_operator>

<a id="op-44be47a3cafd4506657221ff"></a>
## As

`variant` · `sqlparser::ast::query::PipeOperator::As` · sqlparser 0.62.0

```rust
As
```

Source: `src/ast/query.rs:3224`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Introduces a table alias for the input table, similar to applying the AS alias clause on a table subquery in standard syntax.

Syntax: `|> AS <alias>`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#as_pipe_operator>

<a id="op-e3136e0e29ed6e82d3864e0e"></a>
## Call

`variant` · `sqlparser::ast::query::PipeOperator::Call` · sqlparser 0.62.0

```rust
Call
```

Source: `src/ast/query.rs:3299`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Calls a table function or procedure that returns a table.

Syntax: `|> CALL function_name(args) [AS alias]`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#call_pipe_operator>

<a id="op-1294fce3be4b1696ba523ae8"></a>
## Drop

`variant` · `sqlparser::ast::query::PipeOperator::Drop` · sqlparser 0.62.0

```rust
Drop
```

Source: `src/ast/query.rs:3215`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Removes listed columns from the current table, similar to SELECT * EXCEPT (column) in standard syntax.

Syntax: `|> DROP <column>, ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#drop_pipe_operator>

<a id="op-b74acda303cb3216a8fcb776"></a>
## Except

`variant` · `sqlparser::ast::query::PipeOperator::Except` · sqlparser 0.62.0

```rust
Except
```

Source: `src/ast/query.rs:3288`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns only the rows that are present in the input table but not in the specified tables.

Syntax: `|> EXCEPT DISTINCT (<query>), (<query>), ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#except_pipe_operator>

<a id="op-0524be8d125b3b1634779ec6"></a>
## Extend

`variant` · `sqlparser::ast::query::PipeOperator::Extend` · sqlparser 0.62.0

```rust
Extend
```

Source: `src/ast/query.rs:3197`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Propagates the existing table and adds computed columns, similar to SELECT *, new_column in standard syntax.

Syntax: `|> EXTEND <expr> [[AS] alias], ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#extend_pipe_operator>

<a id="op-d1fa6daf01e3bdd9d29a2713"></a>
## Intersect

`variant` · `sqlparser::ast::query::PipeOperator::Intersect` · sqlparser 0.62.0

```rust
Intersect
```

Source: `src/ast/query.rs:3277`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Returns only the rows that are present in both the input table and the specified tables.

Syntax: `|> INTERSECT [DISTINCT] (<query>), (<query>), ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#intersect_pipe_operator>

<a id="op-0b9dbb1f67ab2c4eadca8475"></a>
## Join

`variant` · `sqlparser::ast::query::PipeOperator::Join` · sqlparser 0.62.0

```rust
Join
```

Source: `src/ast/query.rs:3343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Joins the input table with another table.

Syntax: `|> [JOIN_TYPE] JOIN <table> [alias] ON <condition>` or `|> [JOIN_TYPE] JOIN <table> [alias] USING (<columns>)`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#join_pipe_operator>

<a id="op-aea569788dfbd9c17e4c311a"></a>
## Limit

`variant` · `sqlparser::ast::query::PipeOperator::Limit` · sqlparser 0.62.0

```rust
Limit
```

Source: `src/ast/query.rs:3163`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Limits the number of rows to return in a query, with an optional OFFSET clause to skip over rows.

Syntax: `|> LIMIT <n> [OFFSET <m>]`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#limit_pipe_operator>

<a id="op-877661b9863eb02b2fcfd716"></a>
## OrderBy

`variant` · `sqlparser::ast::query::PipeOperator::OrderBy` · sqlparser 0.62.0

```rust
OrderBy
```

Source: `src/ast/query.rs:3179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`ORDER BY <expr> [ASC|DESC], ...`

<a id="op-3f5c6144ce26d8e4fa93d599"></a>
## Pivot

`variant` · `sqlparser::ast::query::PipeOperator::Pivot` · sqlparser 0.62.0

```rust
Pivot
```

Source: `src/ast/query.rs:3310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Pivots data from rows to columns.

Syntax: `|> PIVOT(aggregate_function(column) FOR pivot_column IN (value1, value2, ...)) [AS alias]`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#pivot_pipe_operator>

<a id="op-615ff486276332fe4db240ac"></a>
## Rename

`variant` · `sqlparser::ast::query::PipeOperator::Rename` · sqlparser 0.62.0

```rust
Rename
```

Source: `src/ast/query.rs:3257`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Renames columns in the input table.

Syntax: `|> RENAME old_name AS new_name, ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#rename_pipe_operator>

<a id="op-449b05efb59fbfc369b8dbe7"></a>
## Select

`variant` · `sqlparser::ast::query::PipeOperator::Select` · sqlparser 0.62.0

```rust
Select
```

Source: `src/ast/query.rs:3188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Produces a new table with the listed columns, similar to the outermost SELECT clause in a table subquery in standard syntax.

Syntax `|> SELECT <expr> [[AS] alias], ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#select_pipe_operator>

<a id="op-c6cf09c22b5ec24782ffaa58"></a>
## Set

`variant` · `sqlparser::ast::query::PipeOperator::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/query.rs:3206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Replaces the value of a column in the current table, similar to SELECT * REPLACE (expression AS column) in standard syntax.

Syntax: `|> SET <column> = <expression>, ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#set_pipe_operator>

<a id="op-ba8166d3a20db4fa80fee7f1"></a>
## TableSample

`variant` · `sqlparser::ast::query::PipeOperator::TableSample` · sqlparser 0.62.0

```rust
TableSample
```

Source: `src/ast/query.rs:3248`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Selects a random sample of rows from the input table.
Syntax: `|> TABLESAMPLE SYSTEM (10 PERCENT)
See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#tablesample_pipe_operator>

<a id="op-43ed1f10967a135b9ba0a932"></a>
## Union

`variant` · `sqlparser::ast::query::PipeOperator::Union` · sqlparser 0.62.0

```rust
Union
```

Source: `src/ast/query.rs:3266`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Combines the input table with one or more tables using UNION.

Syntax: `|> UNION [ALL|DISTINCT] (<query>), (<query>), ...`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#union_pipe_operator>

<a id="op-9f0752a7d9f55c90083cff55"></a>
## Unpivot

`variant` · `sqlparser::ast::query::PipeOperator::Unpivot` · sqlparser 0.62.0

```rust
Unpivot
```

Source: `src/ast/query.rs:3328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `UNPIVOT` pipe operator transforms columns into rows.

Syntax:
```sql
|> UNPIVOT(value_column FOR name_column IN (column1, column2, ...)) [alias]
```

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#unpivot_pipe_operator>

<a id="op-8851e77c8f7f61385159c225"></a>
## Where

`variant` · `sqlparser::ast::query::PipeOperator::Where` · sqlparser 0.62.0

```rust
Where
```

Source: `src/ast/query.rs:3174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Filters the results of the input table.

Syntax: `|> WHERE <condition>`

See more at <https://cloud.google.com/bigquery/docs/reference/standard-sql/pipe-syntax#where_pipe_operator>

<a id="op-c81bb017277a769d12190ead"></a>
## clone

`function` · `sqlparser::ast::query::PipeOperator::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> PipeOperator
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 17], "end": [3154, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-754c288d213b466cd3da38e5"></a>
## cmp

`function` · `sqlparser::ast::query::PipeOperator::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &PipeOperator) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 51], "end": [3154, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a32a8ea9c5cd9a8b8de7d04b"></a>
## deserialize

`function` · `sqlparser::ast::query::PipeOperator::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3155, 49], "end": [3155, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-214d480693cb9bdd3fa42f1a"></a>
## eq

`function` · `sqlparser::ast::query::PipeOperator::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &PipeOperator) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 24], "end": [3154, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-498ea9ab34d1b8915b6de712"></a>
## fmt

`function` · `sqlparser::ast::query::PipeOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 10], "end": [3154, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d338b5b7f8efeae668270bd4"></a>
## fmt

`function` · `sqlparser::ast::query::PipeOperator::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3346, 1], "end": [3451, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e208ca57a0c95671e76a6fd"></a>
## hash

`function` · `sqlparser::ast::query::PipeOperator::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 56], "end": [3154, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-708927be3a1294a61a29f6b4"></a>
## partial_cmp

`function` · `sqlparser::ast::query::PipeOperator::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &PipeOperator) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3154, 35], "end": [3154, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3154`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e24705b45c3db432d461afd"></a>
## serialize

`function` · `sqlparser::ast::query::PipeOperator::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3155, 38], "end": [3155, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3155`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-847eee6d29a62a37d61576c4"></a>
## visit

`function` · `sqlparser::ast::query::PipeOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3156, 40], "end": [3156, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9faefac4df028cf0665517b6"></a>
## visit

`function` · `sqlparser::ast::query::PipeOperator::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::PipeOperator", "path": "PipeOperator"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3156, 47], "end": [3156, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3156`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
