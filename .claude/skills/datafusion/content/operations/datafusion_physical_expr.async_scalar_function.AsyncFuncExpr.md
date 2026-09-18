# `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.async_scalar_function.AsyncFuncExpr.json).

<a id="op-14a803c4734de7e963d19e80"></a>
## AsyncFuncExpr

`struct` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr` · datafusion-physical-expr 55.1.0

```rust
struct AsyncFuncExpr
```

Source: `src/async_scalar_function.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Wrapper around a scalar function that can be evaluated asynchronously

<a id="op-433c1b6ac5d3deca0ac4958a"></a>
## children

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec848ac1b81c1c4318dc0b40"></a>
## clone

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> AsyncFuncExpr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 17], "end": [34, 22], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/async_scalar_function.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bfae82f3bb994ad8113c11a"></a>
## data_type

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:207`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d862baad2ce8d5ef4bd67bca"></a>
## eq

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 1], "end": [54, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/async_scalar_function.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b74235348577df845a837905"></a>
## evaluate

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, _batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:221`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6137ea6221adbb2d9dbc8c5c"></a>
## field

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self, _input_schema: &Schema) -> Result<Field>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [204, 2], "filename": "src/async_scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_scalar_function.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return the output field generated by evaluating this function

<a id="op-d84654c3a15d1d68fb38eb10"></a>
## fmt

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [34, 10], "end": [34, 15], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/async_scalar_function.rs:34`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dbe7c7f60fa1eead770c3fab"></a>
## fmt

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 1], "end": [48, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/async_scalar_function.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7c9b4293775a11f7affe088"></a>
## fmt_sql

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:242`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69f572f1234ede2a05472f9e"></a>
## func

`struct_field` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::func` · datafusion-physical-expr 55.1.0

```rust
func: std::sync::Arc<dyn PhysicalExpr>
```

Source: `src/async_scalar_function.rs:39`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The actual function (always `ScalarFunctionExpr`)

<a id="op-b6a9f2d553f694f23f7b1c22"></a>
## hash

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 1], "end": [61, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/async_scalar_function.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-878d39ff12338ba2689e4353"></a>
## ideal_batch_size

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::ideal_batch_size` · datafusion-physical-expr 55.1.0

```rust
fn ideal_batch_size(&self) -> Result<Option<usize>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [204, 2], "filename": "src/async_scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_scalar_function.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Return the ideal batch size for this function

<a id="op-83522e51a6c930d48737c8ac"></a>
## invoke_with_args

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::invoke_with_args` · datafusion-physical-expr 55.1.0

```rust
async fn invoke_with_args(&self, batch: &RecordBatch, config_options: Arc<ConfigOptions>) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [204, 2], "filename": "src/async_scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_scalar_function.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

This (async) function is called for each record batch to evaluate the LLM expressions

The output is the output of evaluating the async expression and the input record batch

<a id="op-a6042df88095eec4a0f688cd"></a>
## name

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [204, 2], "filename": "src/async_scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_scalar_function.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

return the name of the output column

<a id="op-bf023d931e9fd320d357493e"></a>
## name

`struct_field` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::name` · datafusion-physical-expr 55.1.0

```rust
name: String
```

Source: `src/async_scalar_function.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The name of the output column this function will generate

<a id="op-653eda8e61cf3357a5eb933e"></a>
## nullable

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f74b200042228c82e61c08e"></a>
## return_field

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e725ec9aa79e14713a7e769b"></a>
## try_new

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(name: impl Into<String>, func: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [204, 2], "filename": "src/async_scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_scalar_function.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

create a new AsyncFuncExpr

<a id="op-e7344d4e35c196780b3d2072"></a>
## with_new_children

`function` · `datafusion_physical_expr::async_scalar_function::AsyncFuncExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::async_scalar_function::AsyncFuncExpr", "path": "AsyncFuncExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [206, 1], "end": [245, 2], "filename": "src/async_scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/async_scalar_function.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
