# `sqlparser::ast::query::Select`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.Select.json).

<a id="op-10b575bc5d4cbdd9ae434f0d"></a>
## Select

`struct` · `sqlparser::ast::query::Select` · sqlparser 0.62.0

```rust
struct Select
```

Source: `src/ast/query.rs:446`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A restricted variant of `SELECT` (without CTEs/`ORDER BY`), which may
appear either as the only body item of a `Query`, or as an operand
to a set operation like `UNION`.

<a id="op-90b245d0dcf62a08cdee3bde"></a>
## clone

`function` · `sqlparser::ast::query::Select::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Select
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 17], "end": [442, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1054aae38dcd2c9a5d6c8d42"></a>
## cluster_by

`struct_field` · `sqlparser::ast::query::Select::cluster_by` · sqlparser 0.62.0

```rust
cluster_by: Vec<Expr>
```

Source: `src/ast/query.rs:489`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CLUSTER BY (Hive)

<a id="op-cf1f44405e7443d674d2eb37"></a>
## cmp

`function` · `sqlparser::ast::query::Select::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Select) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 51], "end": [442, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2bcdcff3286a76883f36901b"></a>
## connect_by

`struct_field` · `sqlparser::ast::query::Select::connect_by` · sqlparser 0.62.0

```rust
connect_by: Vec<ConnectByKind>
```

Source: `src/ast/query.rs:485`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

[START WITH ..] CONNECT BY ..

<a id="op-65168b5ade775878c11233c8"></a>
## deserialize

`function` · `sqlparser::ast::query::Select::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 49], "end": [443, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-140b2f0695740dcbaef26d28"></a>
## distinct

`struct_field` · `sqlparser::ast::query::Select::distinct` · sqlparser 0.62.0

```rust
distinct: Option<Distinct>
```

Source: `src/ast/query.rs:455`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`SELECT [DISTINCT] ...`

<a id="op-ec28e83d0f5f191eac1f3809"></a>
## distribute_by

`struct_field` · `sqlparser::ast::query::Select::distribute_by` · sqlparser 0.62.0

```rust
distribute_by: Vec<Expr>
```

Source: `src/ast/query.rs:491`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DISTRIBUTE BY (Hive)

<a id="op-4d1bdf10cbfddd62b23785f4"></a>
## eq

`function` · `sqlparser::ast::query::Select::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Select) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 24], "end": [442, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb94f1cfb1b88d652cfbfa7d"></a>
## exclude

`struct_field` · `sqlparser::ast::query::Select::exclude` · sqlparser 0.62.0

```rust
exclude: Option<ExcludeSelectItem>
```

Source: `src/ast/query.rs:470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Excluded columns from the projection expression which are not specified
directly after a wildcard.

[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_EXCLUDE_list.html)

<a id="op-b2c0722ef9c7ec352917f3db"></a>
## flavor

`struct_field` · `sqlparser::ast::query::Select::flavor` · sqlparser 0.62.0

```rust
flavor: SelectFlavor
```

Source: `src/ast/query.rs:508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Was this a FROM-first query?

<a id="op-be26923d5fd8301683043213"></a>
## fmt

`function` · `sqlparser::ast::query::Select::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 10], "end": [442, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e82ce3336aaa4d4708ef4899"></a>
## fmt

`function` · `sqlparser::ast::query::Select::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [511, 1], "end": [658, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4342919aef87d85682d882b8"></a>
## from

`struct_field` · `sqlparser::ast::query::Select::from` · sqlparser 0.62.0

```rust
from: Vec<TableWithJoins>
```

Source: `src/ast/query.rs:474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FROM

<a id="op-a643bb2be053f5970105e8d4"></a>
## group_by

`struct_field` · `sqlparser::ast::query::Select::group_by` · sqlparser 0.62.0

```rust
group_by: GroupByExpr
```

Source: `src/ast/query.rs:487`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

GROUP BY

<a id="op-802ef304f907c7a91c814a53"></a>
## hash

`function` · `sqlparser::ast::query::Select::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 56], "end": [442, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e4799592c29c7decc8485aa"></a>
## having

`struct_field` · `sqlparser::ast::query::Select::having` · sqlparser 0.62.0

```rust
having: Option<Expr>
```

Source: `src/ast/query.rs:495`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

HAVING

<a id="op-790bc9cd0feef25dd18de77e"></a>
## into

`struct_field` · `sqlparser::ast::query::Select::into` · sqlparser 0.62.0

```rust
into: Option<SelectInto>
```

Source: `src/ast/query.rs:472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

INTO

<a id="op-b02cc16d317f459420a4ae7f"></a>
## lateral_views

`struct_field` · `sqlparser::ast::query::Select::lateral_views` · sqlparser 0.62.0

```rust
lateral_views: Vec<LateralView>
```

Source: `src/ast/query.rs:476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LATERAL VIEWs

<a id="op-31b522771c6b319e67286fc1"></a>
## named_window

`struct_field` · `sqlparser::ast::query::Select::named_window` · sqlparser 0.62.0

```rust
named_window: Vec<NamedWindowDefinition>
```

Source: `src/ast/query.rs:497`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WINDOW AS

<a id="op-d62a955c614f585e49f79aa0"></a>
## optimizer_hints

`struct_field` · `sqlparser::ast::query::Select::optimizer_hints` · sqlparser 0.62.0

```rust
optimizer_hints: Vec<OptimizerHint>
```

Source: `src/ast/query.rs:453`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Query optimizer hints

[MySQL](https://dev.mysql.com/doc/refman/8.4/en/optimizer-hints.html)
[Oracle](https://docs.oracle.com/en/database/oracle/oracle-database/21/sqlrf/Comments.html#GUID-D316D545-89E2-4D54-977F-FC97815CD62E)

<a id="op-d61bfc1295b0fc6ae6024042"></a>
## partial_cmp

`function` · `sqlparser::ast::query::Select::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Select) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [442, 35], "end": [442, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:442`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f38c7b2e1b66f738b72a0776"></a>
## prewhere

`struct_field` · `sqlparser::ast::query::Select::prewhere` · sqlparser 0.62.0

```rust
prewhere: Option<Expr>
```

Source: `src/ast/query.rs:481`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse syntax: `PREWHERE a = 1 WHERE b = 2`,
and it can be used together with WHERE selection.

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/select/prewhere)

<a id="op-eddf3172cb30cbacc6d402e4"></a>
## projection

`struct_field` · `sqlparser::ast::query::Select::projection` · sqlparser 0.62.0

```rust
projection: Vec<SelectItem>
```

Source: `src/ast/query.rs:465`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

projection expressions

<a id="op-109a65436294bb063e927772"></a>
## qualify

`struct_field` · `sqlparser::ast::query::Select::qualify` · sqlparser 0.62.0

```rust
qualify: Option<Expr>
```

Source: `src/ast/query.rs:499`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

QUALIFY (Snowflake)

<a id="op-f584a60ebebbf1f22303a051"></a>
## select_modifiers

`struct_field` · `sqlparser::ast::query::Select::select_modifiers` · sqlparser 0.62.0

```rust
select_modifiers: Option<SelectModifiers>
```

Source: `src/ast/query.rs:459`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-specific SELECT modifiers.

See [MySQL SELECT](https://dev.mysql.com/doc/refman/8.4/en/select.html).

<a id="op-dc58e30d04951599cc497b91"></a>
## select_token

`struct_field` · `sqlparser::ast::query::Select::select_token` · sqlparser 0.62.0

```rust
select_token: helpers::attached_token::AttachedToken
```

Source: `src/ast/query.rs:448`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Token for the `SELECT` keyword

<a id="op-380f830a05d23057bac3b3d5"></a>
## selection

`struct_field` · `sqlparser::ast::query::Select::selection` · sqlparser 0.62.0

```rust
selection: Option<Expr>
```

Source: `src/ast/query.rs:483`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

WHERE

<a id="op-702f5fef898b057106e8e333"></a>
## serialize

`function` · `sqlparser::ast::query::Select::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 38], "end": [443, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:443`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb5096f150d376fbe5f110cb"></a>
## sort_by

`struct_field` · `sqlparser::ast::query::Select::sort_by` · sqlparser 0.62.0

```rust
sort_by: Vec<OrderByExpr>
```

Source: `src/ast/query.rs:493`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SORT BY (Hive)

<a id="op-846fc0bdfda2d1126148581e"></a>
## span

`function` · `sqlparser::ast::query::Select::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "super::Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2286, 1], "end": [2333, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c13f2c4d3c4e0eacf516c81e"></a>
## top

`struct_field` · `sqlparser::ast::query::Select::top` · sqlparser 0.62.0

```rust
top: Option<Top>
```

Source: `src/ast/query.rs:461`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MSSQL syntax: `TOP (<N>) [ PERCENT ] [ WITH TIES ]`

<a id="op-296316c43074b7e64186c2c4"></a>
## top_before_distinct

`struct_field` · `sqlparser::ast::query::Select::top_before_distinct` · sqlparser 0.62.0

```rust
top_before_distinct: bool
```

Source: `src/ast/query.rs:463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the top was located before `ALL`/`DISTINCT`

<a id="op-3b2ca43624c340098ca4ad7f"></a>
## value_table_mode

`struct_field` · `sqlparser::ast::query::Select::value_table_mode` · sqlparser 0.62.0

```rust
value_table_mode: Option<ValueTableMode>
```

Source: `src/ast/query.rs:506`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

BigQuery syntax: `SELECT AS VALUE | SELECT AS STRUCT`

<a id="op-3e9de72f64dbc3ed08b552c0"></a>
## visit

`function` · `sqlparser::ast::query::Select::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 40], "end": [444, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd3c6d5e909d55aac54dac43"></a>
## visit

`function` · `sqlparser::ast::query::Select::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::Select", "path": "Select"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [444, 47], "end": [444, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:444`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad94c955f9c7f47305e47b19"></a>
## window_before_qualify

`struct_field` · `sqlparser::ast::query::Select::window_before_qualify` · sqlparser 0.62.0

```rust
window_before_qualify: bool
```

Source: `src/ast/query.rs:504`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The positioning of QUALIFY and WINDOW clauses differ between dialects.
e.g. BigQuery requires that WINDOW comes after QUALIFY, while DUCKDB accepts
WINDOW before QUALIFY.
We accept either positioning and flag the accepted variant.
