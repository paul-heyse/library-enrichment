# `datafusion_physical_expr::analysis::AnalysisContext`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.analysis.AnalysisContext.json).

<a id="op-d721540697f220e5550a2df6"></a>
## AnalysisContext

`struct` · `datafusion_physical_expr::analysis::AnalysisContext` · datafusion-physical-expr 55.1.0

```rust
struct AnalysisContext
```

Source: `src/analysis.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The shared context used during the analysis of an expression. Includes
the boundaries for all known columns.

<a id="op-6c20c1d65bda6faf4441aae7"></a>
## boundaries

`struct_field` · `datafusion_physical_expr::analysis::AnalysisContext::boundaries` · datafusion-physical-expr 55.1.0

```rust
boundaries: Vec<ExprBoundaries>
```

Source: `src/analysis.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c6236cce04c3d678e6e016"></a>
## clone

`function` · `datafusion_physical_expr::analysis::AnalysisContext::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> AnalysisContext
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 10], "end": [38, 15], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/analysis.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-087091fe3cac0bc32dfb9eaa"></a>
## eq

`function` · `datafusion_physical_expr::analysis::AnalysisContext::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &AnalysisContext) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 24], "end": [38, 33], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/analysis.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cadc099c23d72c82c72c7a0b"></a>
## fmt

`function` · `datafusion_physical_expr::analysis::AnalysisContext::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 17], "end": [38, 22], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analysis.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6395131770399a529ccb14fc"></a>
## new

`function` · `datafusion_physical_expr::analysis::AnalysisContext::new` · datafusion-physical-expr 55.1.0

```rust
fn new(boundaries: Vec<ExprBoundaries>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [74, 2], "filename": "src/analysis.rs"}, "trait": null, "trait_path": null}`

Source: `src/analysis.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9adda94ecf76fce7de4dbffc"></a>
## selectivity

`struct_field` · `datafusion_physical_expr::analysis::AnalysisContext::selectivity` · datafusion-physical-expr 55.1.0

```rust
selectivity: Option<f64>
```

Source: `src/analysis.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The estimated percentage of rows that this expression would select, if
it were to be used as a boolean predicate on a filter. The value will be
between 0.0 (selects nothing) and 1.0 (selects everything).

<a id="op-419fdd3278e87519c6226280"></a>
## try_from_statistics

`function` · `datafusion_physical_expr::analysis::AnalysisContext::try_from_statistics` · datafusion-physical-expr 55.1.0

```rust
fn try_from_statistics(input_schema: &Schema, statistics: &[ColumnStatistics]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [74, 2], "filename": "src/analysis.rs"}, "trait": null, "trait_path": null}`

Source: `src/analysis.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new analysis context from column statistics.

<a id="op-fd347b2755774dd246221265"></a>
## with_selectivity

`function` · `datafusion_physical_expr::analysis::AnalysisContext::with_selectivity` · datafusion-physical-expr 55.1.0

```rust
fn with_selectivity(self, selectivity: f64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::AnalysisContext", "path": "AnalysisContext"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [74, 2], "filename": "src/analysis.rs"}, "trait": null, "trait_path": null}`

Source: `src/analysis.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
