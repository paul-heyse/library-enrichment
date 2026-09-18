# `datafusion_spark::session_state::SessionStateBuilderSpark`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.session_state.SessionStateBuilderSpark.json).

<a id="op-6c4559096664cf9333cdd1c2"></a>
## SessionStateBuilderSpark

`trait` · `datafusion_spark::session_state::SessionStateBuilderSpark` · datafusion-spark 55.1.0

```rust
trait SessionStateBuilderSpark
```

Source: `src/session_state.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Extension trait for adding Apache Spark features to [`SessionStateBuilder`](../operations/datafusion.execution.session_state.SessionStateBuilder.md#op-99760a72dc31f296f2e30fdb).

This trait provides a convenient way to register all Apache Spark-compatible
functions and planners with a DataFusion session.

# Example

```rust
use datafusion::execution::SessionStateBuilder;
use datafusion_spark::SessionStateBuilderSpark;

// Create a SessionState with Apache Spark features enabled
// note: the order matters here, `with_spark_features` should be
// called after `with_default_features` to overwrite any existing functions
let state = SessionStateBuilder::new()
    .with_default_features()
    .with_spark_features()
    .build();
```

<a id="op-909d78a08c083d6981fc4f80"></a>
## with_spark_features

`function` · `datafusion_spark::session_state::SessionStateBuilderSpark::with_spark_features` · datafusion-spark 55.1.0

```rust
fn with_spark_features(self) -> Self
```

Source: `src/session_state.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Adds all expr_planners, scalar, aggregate, window and table functions
compatible with Apache Spark.

Note: This overwrites any previously registered items with the same name.
