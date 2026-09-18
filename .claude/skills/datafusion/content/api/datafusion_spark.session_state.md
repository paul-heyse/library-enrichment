# `datafusion_spark::session_state`

Crate `datafusion-spark` · 1 public items · structured records in [`model/datafusion_spark.session_state.json`](../model/datafusion_spark.session_state.json)

## SessionStateBuilderSpark

`trait` · `datafusion_spark::session_state::SessionStateBuilderSpark`

Also reachable as `datafusion_spark::SessionStateBuilderSpark`

```rust
trait SessionStateBuilderSpark
```

**Implementors** (1)

- `datafusion::execution::session_state::SessionStateBuilder`

**Methods** (1)

```rust
fn with_spark_features(self) -> Self
```

[Full member, field, variant and typed contracts](../operations/datafusion_spark.session_state.SessionStateBuilderSpark.md).


Extension trait for adding Apache Spark features to [`SessionStateBuilder`].

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

---
