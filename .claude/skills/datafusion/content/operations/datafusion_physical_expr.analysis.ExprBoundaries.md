# `datafusion_physical_expr::analysis::ExprBoundaries`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.analysis.ExprBoundaries.json).

<a id="op-a70b1c8762ff8cde1b1a94d3"></a>
## ExprBoundaries

`struct` · `datafusion_physical_expr::analysis::ExprBoundaries` · datafusion-physical-expr 55.1.0

```rust
struct ExprBoundaries
```

Source: `src/analysis.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents the boundaries (e.g. min and max values) of a particular column

This is used range analysis of expressions, to determine if the expression
limits the value of particular columns (e.g. analyzing an expression such as
`time < 50` would result in a boundary interval for `time` having a max
value of `50`).

<a id="op-711dcd3457c58e3e783b5a00"></a>
## clone

`function` · `datafusion_physical_expr::analysis::ExprBoundaries::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ExprBoundaries
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::ExprBoundaries", "path": "ExprBoundaries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 10], "end": [82, 15], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/analysis.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f2871ec1bbcc4eb46d1a0166"></a>
## column

`struct_field` · `datafusion_physical_expr::analysis::ExprBoundaries::column` · datafusion-physical-expr 55.1.0

```rust
column: expressions::Column
```

Source: `src/analysis.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6ec21bcaf9e26be751eb86e"></a>
## distinct_count

`struct_field` · `datafusion_physical_expr::analysis::ExprBoundaries::distinct_count` · datafusion-physical-expr 55.1.0

```rust
distinct_count: datafusion_common::stats::Precision<usize>
```

Source: `src/analysis.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Maximum number of distinct values this expression can produce, if known.

<a id="op-e9a2040f427cd10f54bd4c93"></a>
## eq

`function` · `datafusion_physical_expr::analysis::ExprBoundaries::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &ExprBoundaries) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::ExprBoundaries", "path": "ExprBoundaries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 24], "end": [82, 33], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/analysis.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b3f3322c16ef886c57fcbb6"></a>
## fmt

`function` · `datafusion_physical_expr::analysis::ExprBoundaries::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::ExprBoundaries", "path": "ExprBoundaries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [82, 17], "end": [82, 22], "filename": "src/analysis.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/analysis.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-855587a4cc83fef79489c689"></a>
## interval

`struct_field` · `datafusion_physical_expr::analysis::ExprBoundaries::interval` · datafusion-physical-expr 55.1.0

```rust
interval: Option<datafusion_expr::interval_arithmetic::Interval>
```

Source: `src/analysis.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Minimum and maximum values this expression can have. A `None` value
indicates that evaluating the given column results in an empty set.
For example, if the column `a` has values in the range [10, 20],
and there is a filter asserting that `a > 50`, then the resulting interval
range of `a` will be `None`.

<a id="op-1fc70ad220e60d7fb80e2951"></a>
## try_from_column

`function` · `datafusion_physical_expr::analysis::ExprBoundaries::try_from_column` · datafusion-physical-expr 55.1.0

```rust
fn try_from_column(schema: &Schema, col_stats: &ColumnStatistics, col_index: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::ExprBoundaries", "path": "ExprBoundaries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [147, 2], "filename": "src/analysis.rs"}, "trait": null, "trait_path": null}`

Source: `src/analysis.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new `ExprBoundaries` object from column level statistics.

<a id="op-f88f841fd33f8243ae317162"></a>
## try_new_unbounded

`function` · `datafusion_physical_expr::analysis::ExprBoundaries::try_new_unbounded` · datafusion-physical-expr 55.1.0

```rust
fn try_new_unbounded(schema: &Schema) -> Result<Vec<Self>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::analysis::ExprBoundaries", "path": "ExprBoundaries"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [147, 2], "filename": "src/analysis.rs"}, "trait": null, "trait_path": null}`

Source: `src/analysis.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create `ExprBoundaries` that represent no known bounds for all the
columns in `schema`
