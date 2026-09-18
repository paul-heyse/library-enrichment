# `datafusion_expr::async_udf::AsyncScalarUDFImpl`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.async_udf.AsyncScalarUDFImpl.json).

<a id="op-646732e18f39410b50c5c611"></a>
## AsyncScalarUDFImpl

`trait` · `datafusion_expr::async_udf::AsyncScalarUDFImpl` · datafusion-expr 55.1.0

```rust
trait AsyncScalarUDFImpl: ScalarUDFImpl
```

Source: `src/async_udf.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A scalar UDF that can invoke using async methods

Note this is less efficient than the ScalarUDFImpl, but it can be used
to register remote functions in the context.

The name is chosen to mirror ScalarUDFImpl

<a id="op-b8bde453a11f2aac25bef583"></a>
## ideal_batch_size

`function` · `datafusion_expr::async_udf::AsyncScalarUDFImpl::ideal_batch_size` · datafusion-expr 55.1.0

```rust
fn ideal_batch_size(&self) -> Option<usize>
```

Source: `src/async_udf.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The ideal batch size for this function.

This is used to determine what size of data to be evaluated at once.
If None, the whole batch will be evaluated at once.

<a id="op-6c87aa3ca1d0cb8df43f2422"></a>
## invoke_async_with_args

`function` · `datafusion_expr::async_udf::AsyncScalarUDFImpl::invoke_async_with_args` · datafusion-expr 55.1.0

```rust
async fn invoke_async_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Source: `src/async_udf.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Invoke the function asynchronously with the async arguments
