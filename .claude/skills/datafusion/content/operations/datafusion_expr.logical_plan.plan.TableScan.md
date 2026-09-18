# `datafusion_expr::logical_plan::plan::TableScan`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.TableScan.json).

<a id="op-7071222fb9e2f98877e7d141"></a>
## TableScan

`struct` · `datafusion_expr::logical_plan::plan::TableScan` · datafusion-expr 55.1.0

```rust
struct TableScan
```

Source: `src/logical_plan/plan.rs:2928`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Produces rows from a table provider by reference or from the context

<a id="op-a38256e07ea20280b4b7b0af"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::TableScan::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> TableScan
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2927, 10], "end": [2927, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:2927`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a919b153a8fc7b6a25bac9a2"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::TableScan::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2962, 1], "end": [2970, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:2963`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a99de0ea2a5c97b64efe4972"></a>
## fetch

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::fetch` · datafusion-expr 55.1.0

```rust
fetch: Option<usize>
```

Source: `src/logical_plan/plan.rs:2940`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional number of rows to read

<a id="op-a00e83a06cf00fd223cb0c36"></a>
## filters

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::filters` · datafusion-expr 55.1.0

```rust
filters: Vec<Expr>
```

Source: `src/logical_plan/plan.rs:2938`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional expressions to be used as filters by the table provider

<a id="op-4433c2f7608a22cef92ee1c0"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::TableScan::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2949, 1], "end": [2960, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:2950`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c86ff39ec9ec28269083104"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::TableScan::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3008, 1], "end": [3016, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3009`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-082ef3f5ceae7c0a0345361c"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::TableScan::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2976, 1], "end": [3006, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:2977`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b728186b8adf522ee6772c8f"></a>
## projected_schema

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::projected_schema` · datafusion-expr 55.1.0

```rust
projected_schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:2936`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The schema description of the output

<a id="op-20916799cd604dfa3fb2b0bb"></a>
## projection

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::projection` · datafusion-expr 55.1.0

```rust
projection: Option<Vec<usize>>
```

Source: `src/logical_plan/plan.rs:2934`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Optional column indices to use as a projection

<a id="op-71d7da617125028e09bba6ca"></a>
## source

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::source` · datafusion-expr 55.1.0

```rust
source: std::sync::Arc<dyn TableSource>
```

Source: `src/logical_plan/plan.rs:2932`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The source of the table

<a id="op-707a324031bea1f0c3719da5"></a>
## statistics_requests

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::statistics_requests` · datafusion-expr 55.1.0

```rust
statistics_requests: std::collections::BTreeSet<statistics::StatisticsRequest>
```

Source: `src/logical_plan/plan.rs:2946`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Statistics the planner would like the provider to answer for this
scan, typically attached by a custom optimizer rule from the
surrounding plan (e.g. Min/Max for sort keys).

A [`BTreeSet`], not a `Vec` to keep the resulting plan deterministic.

Unresolved upstream links (retained, not inferred): ``BTreeSet``.

<a id="op-5bb3da10360872c540176fd6"></a>
## table_name

`struct_field` · `datafusion_expr::logical_plan::plan::TableScan::table_name` · datafusion-expr 55.1.0

```rust
table_name: datafusion_common::TableReference
```

Source: `src/logical_plan/plan.rs:2930`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The name of the table

<a id="op-1b27be51caffbe73f551b012"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::TableScan::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3018, 1], "end": [3035, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3022`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Initialize TableScan with appropriate schema from the given
arguments.
