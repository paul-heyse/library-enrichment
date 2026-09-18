# `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.intervals.cp_solver.ExprIntervalGraph.json).

<a id="op-a9fe51c501098e92f55a1510"></a>
## ExprIntervalGraph

`struct` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph` · datafusion-physical-expr 55.1.0

```rust
struct ExprIntervalGraph
```

Source: `src/intervals/cp_solver.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This object implements a directed acyclic expression graph (DAEG) that
is used to compute ranges for expressions through interval arithmetic.

<a id="op-19428b1bab67762c56ebbaa6"></a>
## assign_intervals

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::assign_intervals` · datafusion-physical-expr 55.1.0

```rust
fn assign_intervals(&mut self, assignments: &[(usize, Interval)])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:538`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function assigns given ranges to expressions in the DAEG.
The argument `assignments` associates indices of sought expressions
with their corresponding new ranges.

<a id="op-c75ec4fc7f502135c035e29c"></a>
## clone

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> ExprIntervalGraph
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 10], "end": [167, 15], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/intervals/cp_solver.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bc4a19e5cdd7ec7cf39f465"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::evaluate_bounds` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_bounds(&mut self) -> Result<&Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:595`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Computes bounds for an expression using interval arithmetic via a
bottom-up traversal.

# Examples

```
use arrow::datatypes::DataType;
use arrow::datatypes::Field;
use arrow::datatypes::Schema;
use datafusion_common::ScalarValue;
use datafusion_expr::interval_arithmetic::Interval;
use datafusion_expr::Operator;
use datafusion_physical_expr::expressions::{BinaryExpr, Column, Literal};
use datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph;
use datafusion_physical_expr::PhysicalExpr;
use std::sync::Arc;

let expr = Arc::new(BinaryExpr::new(
    Arc::new(Column::new("gnz", 0)),
    Operator::Plus,
    Arc::new(Literal::new(ScalarValue::Int32(Some(10)))),
));

let schema = Schema::new(vec![Field::new("gnz".to_string(), DataType::Int32, true)]);

let mut graph = ExprIntervalGraph::try_new(expr, &schema).unwrap();
// Do it once, while constructing.
let node_indices = graph.gather_node_indices(&[Arc::new(Column::new("gnz", 0))]);
let left_index = node_indices.get(0).unwrap().1;

// Provide intervals for leaf variables (here, there is only one).
let intervals = vec![(left_index, Interval::make(Some(10), Some(20)).unwrap())];

// Evaluate bounds for the composite expression:
graph.assign_intervals(&intervals);
assert_eq!(
    graph.evaluate_bounds().unwrap(),
    &Interval::make(Some(20), Some(30)).unwrap(),
)
```

<a id="op-a61dadfeec94bffe76f2404f"></a>
## fmt

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 17], "end": [167, 22], "filename": "src/intervals/cp_solver.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/intervals/cp_solver.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7a8e42f80c53686f8a3c39c"></a>
## gather_node_indices

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::gather_node_indices` · datafusion-physical-expr 55.1.0

```rust
fn gather_node_indices(&mut self, exprs: &[Arc<dyn PhysicalExpr>]) -> Vec<(Arc<dyn PhysicalExpr>, usize)>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:457`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function associates stable node indices with [`PhysicalExpr`](../operations/datafusion_physical_expr_common.physical_expr.PhysicalExpr.md#op-fe8284c43330456b0d4e6af7)s so
that we can match `Arc<dyn PhysicalExpr>` and NodeIndex objects during
membership tests.

<a id="op-371e40188b81615215762cd4"></a>
## get_interval

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::get_interval` · datafusion-physical-expr 55.1.0

```rust
fn get_interval(&self, index: usize) -> Interval
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:670`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the interval associated with the node at the given `index`.

<a id="op-1b04e7dc27efce947f24b037"></a>
## node_count

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::node_count` · datafusion-physical-expr 55.1.0

```rust
fn node_count(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3825d5ed358c460eee0ad00d"></a>
## size

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::size` · datafusion-physical-expr 55.1.0

```rust
fn size(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:398`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Estimate size of bytes including `Self`.

<a id="op-136ee9d7db97e15429185ee8"></a>
## try_new

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c2357724f0f1b2339490c70"></a>
## update_intervals

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::update_intervals` · datafusion-physical-expr 55.1.0

```rust
fn update_intervals(&self, assignments: &mut [(usize, Interval)])
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:548`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This function fetches ranges of expressions from the DAEG. The argument
`assignments` associates indices of sought expressions with their ranges,
which this function modifies to reflect the intervals in the DAEG.

<a id="op-778dcd67565e19779765a407"></a>
## update_ranges

`function` · `datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph::update_ranges` · datafusion-physical-expr 55.1.0

```rust
fn update_ranges(&mut self, leaf_bounds: &mut [(usize, Interval)], given_range: Interval) -> Result<PropagationResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::intervals::cp_solver::ExprIntervalGraph", "path": "ExprIntervalGraph"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [385, 1], "end": [673, 2], "filename": "src/intervals/cp_solver.rs"}, "trait": null, "trait_path": null}`

Source: `src/intervals/cp_solver.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Updates intervals for all expressions in the DAEG by successive
bottom-up and top-down traversals.
