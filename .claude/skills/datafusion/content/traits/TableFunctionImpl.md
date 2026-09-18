# TableFunctionImpl

`datafusion_session::table::TableFunctionImpl`

```rust
trait TableFunctionImpl: Debug + Sync + Send + Any
```

Also reachable as `datafusion_session::TableFunctionImpl`

Prose: [`api/datafusion_session.table.md`](../api/datafusion_session.table.md#tablefunctionimpl) · records: [`model/datafusion_session.table.json`](../model/datafusion_session.table.json)

## Provided

These methods have defaults. Read each full contract before overriding: some defaults reject unsupported operations, while others provide suitable general behavior. Required methods alone do not prove correctness or performance.

```rust
fn call(&self, _exprs: &[Expr]) -> Result<Arc<dyn TableProvider>>
fn call_with_args(&self, args: TableFunctionArgs<'_, '_>) -> Result<Arc<dyn TableProvider>>
```

## Implementors (3)

Read one before writing your own.

- `datafusion_ffi::udtf::ForeignTableFunction`
- `datafusion_functions_table::generate_series::GenerateSeriesFunc`
- `datafusion_functions_table::generate_series::RangeFunc`

## Demonstrated by 2 upstream example(s)

- [`corpus/examples/udf/simple_udtf.rs`](../corpus/examples/udf/simple_udtf.rs)
- [`corpus/examples/udf/table_list_udtf.rs`](../corpus/examples/udf/table_list_udtf.rs)

## Documentation

A trait for table function implementations
