# `datafusion_physical_expr::utils::build_dag`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.utils.build_dag.json).

<a id="op-c0a386643ff21bea4436b195"></a>
## build_dag

`function` · `datafusion_physical_expr::utils::build_dag` · datafusion-physical-expr 55.1.0

```rust
fn build_dag<T, F>(expr: std::sync::Arc<dyn PhysicalExpr>, constructor: &F) -> datafusion_common::Result<(petgraph::graph::NodeIndex, petgraph::stable_graph::StableGraph<T, usize>)> where F: Fn(&ExprTreeNode<petgraph::graph::NodeIndex>) -> datafusion_common::Result<T>
```

Source: `src/utils/mod.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
