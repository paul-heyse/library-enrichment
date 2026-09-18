# `sqlparser::ast::ddl::CreateView`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateView.json).

<a id="op-5e29f1e23293cb9a95f0de4a"></a>
## CreateView

`struct` · `sqlparser::ast::ddl::CreateView` · sqlparser 0.62.0

```rust
struct CreateView
```

Source: `src/ast/ddl.rs:4294`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE VIEW statement.

<a id="op-6b0b88cc43b1553429554e5e"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateView::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateView
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 17], "end": [4291, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d56fdfe1205b9246e149af98"></a>
## cluster_by

`struct_field` · `sqlparser::ast::ddl::CreateView::cluster_by` · sqlparser 0.62.0

```rust
cluster_by: Vec<ast::Ident>
```

Source: `src/ast/ddl.rs:4326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery: CLUSTER BY columns

<a id="op-9828c1f31baaf2ece839bee3"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateView::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateView) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 51], "end": [4291, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d814cc6a584c9564b377a6"></a>
## columns

`struct_field` · `sqlparser::ast::ddl::CreateView::columns` · sqlparser 0.62.0

```rust
columns: Vec<ViewColumnDef>
```

Source: `src/ast/ddl.rs:4320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional column definitions

<a id="op-164e872078eaa6cc2a537c44"></a>
## comment

`struct_field` · `sqlparser::ast::ddl::CreateView::comment` · sqlparser 0.62.0

```rust
comment: Option<String>
```

Source: `src/ast/ddl.rs:4329`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake: Views can have comments in Snowflake.
<https://docs.snowflake.com/en/sql-reference/sql/create-view#syntax>

<a id="op-baa6207f47644ae795c673d2"></a>
## copy_grants

`struct_field` · `sqlparser::ast::ddl::CreateView::copy_grants` · sqlparser 0.62.0

```rust
copy_grants: bool
```

Source: `src/ast/ddl.rs:4338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake: `COPY GRANTS` clause
<https://docs.snowflake.com/en/sql-reference/sql/create-view>

<a id="op-a979ec3155f51d8946c2f60f"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateView::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4292, 49], "end": [4292, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fdc537c9eca13b18538ca26"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateView::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateView) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 24], "end": [4291, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6577ac14e021d8a2dd5e0fbe"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateView::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 10], "end": [4291, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91eacd19e07c7448004d94bb"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateView::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4346, 1], "end": [4412, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4347`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cda6eea9ad31c0917e176fa"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateView::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 56], "end": [4291, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9908af280e63d83571575ceb"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateView::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:4333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

if true, has SQLite `IF NOT EXISTS` clause <https://www.sqlite.org/lang_createview.html>

<a id="op-e578a387c728a6fba3316b17"></a>
## materialized

`struct_field` · `sqlparser::ast::ddl::CreateView::materialized` · sqlparser 0.62.0

```rust
materialized: bool
```

Source: `src/ast/ddl.rs:4302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

if true, has MATERIALIZED view modifier

<a id="op-dfee07b4239cb9a88a11d2d5"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateView::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4307`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

View name

<a id="op-0881cff3a1ad962a765a3a20"></a>
## name_before_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateView::name_before_not_exists` · sqlparser 0.62.0

```rust
name_before_not_exists: bool
```

Source: `src/ast/ddl.rs:4318`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

If `if_not_exists` is true, this flag is set to true if the view name comes before the `IF NOT EXISTS` clause.
Example:
```sql
CREATE VIEW myview IF NOT EXISTS AS SELECT 1`
 ```
Otherwise, the flag is set to false if the view name comes after the clause
Example:
```sql
CREATE VIEW IF NOT EXISTS myview AS SELECT 1`
 ```

<a id="op-6ef86a2374d978de6ff4a36d"></a>
## options

`struct_field` · `sqlparser::ast::ddl::CreateView::options` · sqlparser 0.62.0

```rust
options: ast::CreateTableOptions
```

Source: `src/ast/ddl.rs:4324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table options (e.g., WITH (..), OPTIONS (...))

<a id="op-611f89827240b1018e490c24"></a>
## or_alter

`struct_field` · `sqlparser::ast::ddl::CreateView::or_alter` · sqlparser 0.62.0

```rust
or_alter: bool
```

Source: `src/ast/ddl.rs:4298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

True if this is a `CREATE OR ALTER VIEW` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-view-transact-sql)

<a id="op-4ae6656107806d01c51cb74f"></a>
## or_replace

`struct_field` · `sqlparser::ast::ddl::CreateView::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/ddl.rs:4300`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The `OR REPLACE` clause is used to re-create the view if it already exists.

<a id="op-7d3619d40c155d186bf90dc4"></a>
## params

`struct_field` · `sqlparser::ast::ddl::CreateView::params` · sqlparser 0.62.0

```rust
params: Option<ast::CreateViewParams>
```

Source: `src/ast/ddl.rs:4343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL: Optional parameters for the view algorithm, definer, and security context

<a id="op-d55374a8649e880a3b9200b2"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateView::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateView) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4291, 35], "end": [4291, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4291`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b0c1492b5c21c8fb86d1557"></a>
## query

`struct_field` · `sqlparser::ast::ddl::CreateView::query` · sqlparser 0.62.0

```rust
query: Box<ast::Query>
```

Source: `src/ast/ddl.rs:4322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The query that defines the view.

<a id="op-9ea0018d370e17dda2cf0743"></a>
## secure

`struct_field` · `sqlparser::ast::ddl::CreateView::secure` · sqlparser 0.62.0

```rust
secure: bool
```

Source: `src/ast/ddl.rs:4305`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake: SECURE view modifier
<https://docs.snowflake.com/en/sql-reference/sql/create-view#syntax>

<a id="op-970132c03b2df109f3eae335"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateView::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4292, 38], "end": [4292, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4292`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6978be12f0f0f30b87080b9f"></a>
## span

`function` · `sqlparser::ast::ddl::CreateView::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "crate::ast::CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2471, 1], "end": [2482, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9387924be356acf0a331efb"></a>
## temporary

`struct_field` · `sqlparser::ast::ddl::CreateView::temporary` · sqlparser 0.62.0

```rust
temporary: bool
```

Source: `src/ast/ddl.rs:4335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

if true, has SQLite `TEMP` or `TEMPORARY` clause <https://www.sqlite.org/lang_createview.html>

<a id="op-060e125c0e4250caf178ef32"></a>
## to

`struct_field` · `sqlparser::ast::ddl::CreateView::to` · sqlparser 0.62.0

```rust
to: Option<ast::ObjectName>
```

Source: `src/ast/ddl.rs:4341`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

if not None, has Clickhouse `TO` clause, specify the table into which to insert results
<https://clickhouse.com/docs/en/sql-reference/statements/create/view#materialized-view>

<a id="op-5880bd27a13ca33d0b0f805b"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateView::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 40], "end": [4293, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8cfdb50b6d930c0e8e9acab"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateView::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateView", "path": "CreateView"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4293, 47], "end": [4293, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4293`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e5d8d44a4e942c963ab6f51"></a>
## with_no_schema_binding

`struct_field` · `sqlparser::ast::ddl::CreateView::with_no_schema_binding` · sqlparser 0.62.0

```rust
with_no_schema_binding: bool
```

Source: `src/ast/ddl.rs:4331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

if true, has RedShift [`WITH NO SCHEMA BINDING`] clause <https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_VIEW.html>
