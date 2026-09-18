# `datafusion_expr::logical_plan::plan::TableScanBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.TableScanBuilder.json).

<a id="op-09a550b066bfc8792b33640d"></a>
## TableScanBuilder

`struct` · `datafusion_expr::logical_plan::plan::TableScanBuilder` · datafusion-expr 55.1.0

```rust
struct TableScanBuilder
```

Source: `src/logical_plan/plan.rs:3044`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Builder for [`TableScan`](../operations/datafusion_expr.logical_plan.plan.TableScan.md#op-7071222fb9e2f98877e7d141).

Prefer this over constructing a [`TableScan`](../operations/datafusion_expr.logical_plan.plan.TableScan.md#op-7071222fb9e2f98877e7d141) directly: it derives the
`projected_schema` from the source schema and projection, and is resilient
to new fields being added to [`TableScan`](../operations/datafusion_expr.logical_plan.plan.TableScan.md#op-7071222fb9e2f98877e7d141). An existing scan can be turned
back into a builder with `TableScanBuilder::from(scan)`, tweaked, and
rebuilt with [`TableScanBuilder::build`](../operations/datafusion_expr.logical_plan.plan.TableScanBuilder.md#op-5a64c4c94f1b6e4d1e3b86d7).

<a id="op-5a64c4c94f1b6e4d1e3b86d7"></a>
## build

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::build` · datafusion-expr 55.1.0

```rust
fn build(self) -> Result<TableScan>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3099`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Build the [`TableScan`](../operations/datafusion_expr.logical_plan.plan.TableScan.md#op-7071222fb9e2f98877e7d141), deriving its `projected_schema` from the
source schema and projection.

<a id="op-18d96387813ca63dcb46fd98"></a>
## from

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::from` · datafusion-expr 55.1.0

```rust
fn from(scan: TableScan) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3152, 1], "end": [3163, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScan", "path": "TableScan"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/logical_plan/plan.rs:3153`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c1365b900c31a81c371f88d"></a>
## new

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::new` · datafusion-expr 55.1.0

```rust
fn new(table_name: impl Into<TableReference>, source: Arc<dyn TableSource>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3055`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new builder for a scan of `source` named `table_name`.

<a id="op-d3a9f539c45bb1131a119adb"></a>
## with_fetch

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::with_fetch` · datafusion-expr 55.1.0

```rust
fn with_fetch(self, fetch: Option<usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3082`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the maximum number of rows to read.

<a id="op-0dad6769196e562e312b26dd"></a>
## with_filters

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::with_filters` · datafusion-expr 55.1.0

```rust
fn with_filters(self, filters: Vec<Expr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3076`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the filter expressions offered to the table provider.

<a id="op-7e784af5aeeb8daf46719c06"></a>
## with_projection

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::with_projection` · datafusion-expr 55.1.0

```rust
fn with_projection(self, projection: Option<Vec<usize>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3070`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the column projection (indices into the source schema).

<a id="op-9780be073508e50f0bb5ad25"></a>
## with_statistics_requests

`function` · `datafusion_expr::logical_plan::plan::TableScanBuilder::with_statistics_requests` · datafusion-expr 55.1.0

```rust
fn with_statistics_requests(self, statistics_requests: BTreeSet<StatisticsRequest>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::TableScanBuilder", "path": "TableScanBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3053, 1], "end": [3150, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3089`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Set the statistics requests for the scan. See
[`TableScan::statistics_requests`](../operations/datafusion_expr.logical_plan.plan.TableScan.md#op-707a324031bea1f0c3719da5).
