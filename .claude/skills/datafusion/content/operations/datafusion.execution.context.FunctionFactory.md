# `datafusion::execution::context::FunctionFactory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.FunctionFactory.json).

<a id="op-4f2bfbb3e4aa340eb4071d97"></a>
## FunctionFactory

`trait` · `datafusion::execution::context::FunctionFactory` · datafusion 55.1.0

```rust
trait FunctionFactory: Debug + Sync + Send
```

Source: `src/execution/context/mod.rs:2225`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Interface for handling `CREATE FUNCTION` statements and interacting with
[SessionState](../operations/datafusion.execution.session_state.SessionState.md#op-3ba80ad5c25fa63e8340a601) to create and register functions ([`ScalarUDF`](../operations/datafusion_expr.udf.ScalarUDF.md#op-12eb8b815a2294fbbbd085a0),
[`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379), [`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65), and [`TableFunctionImpl`](../operations/datafusion_session.table.TableFunctionImpl.md#op-7e3147f93fdf3640b07177ce)) dynamically.

Implement this trait to create user-defined functions in a custom way, such
as loading from external libraries or defining them programmatically.
DataFusion will parse `CREATE FUNCTION` statements into [`CreateFunction`](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md#op-3668c6565f5901a6967c489f)
structs and pass them to the [`create`](Self::create) method.

Note there is no default implementation of this trait provided in DataFusion,
because the implementation and requirements vary widely. Please see
[function_factory example] for a reference implementation.

[function_factory example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/builtin_functions/function_factory.rs

# Examples of syntax that can be supported

```sql
CREATE FUNCTION f1(BIGINT)
  RETURNS BIGINT
  RETURN $1 + 1;
```
or
```sql
CREATE FUNCTION to_miles(DOUBLE)
RETURNS DOUBLE
LANGUAGE PYTHON
AS '
import pyarrow.compute as pc

conversation_rate_multiplier = 0.62137119

def to_miles(km_data):
    return pc.multiply(km_data, conversation_rate_multiplier)
'
```

<a id="op-bd82706856a20ed1742b5a55"></a>
## create

`function` · `datafusion::execution::context::FunctionFactory::create` · datafusion 55.1.0

```rust
async fn create(&self, state: &SessionState, statement: CreateFunction) -> Result<RegisterFunction>
```

Source: `src/execution/context/mod.rs:2227`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a new dynamic function from the SQL in the [CreateFunction](../operations/datafusion_expr.logical_plan.ddl.CreateFunction.md#op-3668c6565f5901a6967c489f) statement
