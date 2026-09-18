# `datafusion_spark`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.json).

<a id="op-ae70b4b0dd9491e5b706fe87"></a>
## datafusion_spark

`module` · `datafusion_spark` · datafusion-spark 55.1.0

```rust
mod datafusion_spark
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

 Spark Expression packages for [DataFusion].

 This crate contains a collection of various Spark function packages for DataFusion,
 implemented using the extension API.

 [DataFusion]: https://crates.io/crates/datafusion


 # Available Function Packages
 See the list of [modules](#modules) in this crate for available packages.

 # Example: using all function packages

 You can register all the functions in all packages using the [`register_all`](../operations/datafusion_spark.register_all.md#op-fd7e242dfb35c47d60f716a0)
 function as shown below. Any existing functions will be overwritten, with these
 Spark functions taking priority.

 ```
 # use datafusion_execution::FunctionRegistry;
 # use datafusion_expr::{ScalarUDF, AggregateUDF, WindowUDF, HigherOrderUDF};
 # use datafusion_expr::planner::ExprPlanner;
 # use datafusion_common::Result;
 # use std::collections::HashSet;
 # use std::sync::Arc;
 # // Note: We can't use a real SessionContext here because the
 # // `datafusion_spark` crate has no dependence on the DataFusion crate
 # // thus use a dummy SessionContext that has enough of the implementation
 # struct SessionContext {}
 # impl FunctionRegistry for SessionContext {
 #    fn register_udf(&mut self, _udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>> { Ok (None) }
 #    fn udfs(&self) -> HashSet<String> { unimplemented!() }
 #    fn higher_order_function_names(&self) -> HashSet<String> { unimplemented!() }
 #    fn udafs(&self) -> HashSet<String> { unimplemented!() }
 #    fn udwfs(&self) -> HashSet<String> { unimplemented!() }
 #    fn udf(&self, _name: &str) -> Result<Arc<ScalarUDF>> { unimplemented!() }
 #    fn higher_order_function(&self, name: &str) -> Result<Arc<HigherOrderUDF>> { unimplemented!() }
 #    fn udaf(&self, name: &str) -> Result<Arc<AggregateUDF>> {unimplemented!() }
 #    fn udwf(&self, name: &str) -> Result<Arc<WindowUDF>> { unimplemented!() }
 #    fn expr_planners(&self) -> Vec<Arc<dyn ExprPlanner>> { unimplemented!() }
 # }
 # impl SessionContext {
 #   fn new() -> Self { SessionContext {} }
 #   async fn sql(&mut self, _query: &str) -> Result<()> { Ok(()) }
 #  }
 #
 # async fn stub() -> Result<()> {
 // Create a new session context
 let mut ctx = SessionContext::new();
 // Register all Spark functions with the context
 datafusion_spark::register_all(&mut ctx)?;
 // Run a query using the `sha2` function which is now available and has Spark semantics
 let df = ctx.sql("SELECT sha2('The input String', 256)").await?;
 # Ok(())
 # }
 ```

 # Example: calling a specific function in Rust

 Each package also exports an `expr_fn` submodule that create [`Expr`]s for
 invoking functions via rust using a fluent style. For example, to invoke the
 `sha2` function, you can use the following code:

 ```rust
 # use datafusion_expr::{col, lit};
 use datafusion_spark::expr_fn::sha2;
 // Create the expression `sha2(my_data, 256)`
 let expr = sha2(col("my_data"), lit(256));
 ```

 # Example: using the Spark expression planner

 The [`planner::SparkFunctionPlanner`](../operations/datafusion_spark.planner.SparkFunctionPlanner.md#op-e6b63867ad6e41fad9c0157a) provides Spark-compatible expression
 planning, such as mapping SQL `EXTRACT` expressions to Spark's `date_part`
 function. To use it, register it with your session context:

 ```ignore
 use std::sync::Arc;
 use datafusion::prelude::SessionContext;
 use datafusion_spark::planner::SparkFunctionPlanner;

 let mut ctx = SessionContext::new();
 // Register the Spark expression planner
 ctx.register_expr_planner(Arc::new(SparkFunctionPlanner))?;
 // Now EXTRACT expressions will use Spark semantics
 let df = ctx.sql("SELECT EXTRACT(YEAR FROM timestamp_col) FROM my_table").await?;
 ```

[`Expr`]: datafusion_expr::Expr

 # Example: enabling Apache Spark features with SessionStateBuilder

 The recommended way to enable Apache Spark compatibility is to use the
 `SessionStateBuilderSpark` extension trait. This registers all
 Apache Spark functions (scalar, aggregate, window, and table) as well as the Apache Spark
 expression planner.

 Enable the `core` feature in your `Cargo.toml`:
 ```toml
 datafusion-spark = { version = "X", features = ["core"] }
 ```

 Then use the extension trait - see [`SessionStateBuilderSpark::with_spark_features`](../operations/datafusion_spark.session_state.SessionStateBuilderSpark.md#op-909d78a08c083d6981fc4f80)
 for an example.
