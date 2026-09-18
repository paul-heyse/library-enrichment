# `datafusion_expr::function::PartitionEvaluatorFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.function.PartitionEvaluatorFactory.json).

<a id="op-26682125ab111c44544ed977"></a>
## PartitionEvaluatorFactory

`type_alias` · `datafusion_expr::function::PartitionEvaluatorFactory` · datafusion-expr 55.1.0

```rust
type PartitionEvaluatorFactory = std::sync::Arc<dyn Fn() -> datafusion_common::Result<Box<dyn PartitionEvaluator>> + Send + Sync>
```

Source: `src/function.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Factory that creates a PartitionEvaluator for the given window
function
