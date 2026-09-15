# FunctionFactory

`datafusion::execution::context::FunctionFactory`

```rust
trait FunctionFactory: Debug + Sync + Send
```

Prose: [`api/datafusion.execution.context.md`](../api/datafusion.execution.context.md#functionfactory) · records: [`model/datafusion.execution.context.json`](../model/datafusion.execution.context.json)

## Required

Every implementation must supply these.

```rust
async fn create(&self, state: &SessionState, statement: CreateFunction) -> Result<RegisterFunction>
```

## Demonstrated by 1 upstream example(s)

- [`corpus/examples/builtin_functions/function_factory.rs`](../corpus/examples/builtin_functions/function_factory.rs)

## Documentation

Interface for handling `CREATE FUNCTION` statements and interacting with
[SessionState] to create and register functions ([`ScalarUDF`],
[`AggregateUDF`], [`WindowUDF`], and [`TableFunctionImpl`]) dynamically.

Implement this trait to create user-defined functions in a custom way, such
as loading from external libraries or defining them programmatically.
DataFusion will parse `CREATE FUNCTION` statements into [`CreateFunction`]
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
