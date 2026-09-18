# `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.window.standard_window_function_expr.StandardWindowFunctionExpr.json).

<a id="op-5882bec2eb25cba3cfad5983"></a>
## StandardWindowFunctionExpr

`trait` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr` · datafusion-physical-expr 55.1.0

```rust
trait StandardWindowFunctionExpr: Send + Sync + std::fmt::Debug
```

Source: `src/window/standard_window_function_expr.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluates a window function by instantiating a
[`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) for calculating the function's output in
that partition.

Note that unlike aggregation based window functions, some window
functions such as `rank` ignore the values in the window frame,
but others such as `first_value`, `last_value`, and
`nth_value` need the value.

<a id="op-aba45c23f53b80a1f96a44a7"></a>
## as_any

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::as_any` · datafusion-physical-expr 55.1.0

```rust
fn as_any(&self) -> &dyn Any
```

Source: `src/window/standard_window_function_expr.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the aggregate expression as [`Any`] so that it can be
downcast to a specific implementation.

Unresolved upstream links (retained, not inferred): ``Any``.

<a id="op-bd7fbe661048b2a7cd48f27c"></a>
## create_evaluator

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::create_evaluator` · datafusion-physical-expr 55.1.0

```rust
fn create_evaluator(&self) -> Result<Box<dyn PartitionEvaluator>>
```

Source: `src/window/standard_window_function_expr.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a [`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) for evaluating the function on
a particular partition.

<a id="op-3ecb87d2c54d6e4726aaa64e"></a>
## evaluate_args

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::evaluate_args` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_args(&self, batch: &RecordBatch) -> Result<Vec<ArrayRef>>
```

Source: `src/window/standard_window_function_expr.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Evaluate window function's arguments against the input window
batch and return an [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1).

Typically, the resulting vector is a single element vector.

<a id="op-db755a623dada2fe6238c02c"></a>
## expressions

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::expressions` · datafusion-physical-expr 55.1.0

```rust
fn expressions(&self) -> Vec<Arc<dyn PhysicalExpr>>
```

Source: `src/window/standard_window_function_expr.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Expressions that are passed to the [`PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2).

<a id="op-319d1ee43f23427969220e50"></a>
## field

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::field` · datafusion-physical-expr 55.1.0

```rust
fn field(&self) -> Result<FieldRef>
```

Source: `src/window/standard_window_function_expr.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The field of the final result of evaluating this window function.

<a id="op-704a8ec7bb0da5a130bfd0ca"></a>
## get_result_ordering

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::get_result_ordering` · datafusion-physical-expr 55.1.0

```rust
fn get_result_ordering(&self, _schema: &SchemaRef) -> Option<PhysicalSortExpr>
```

Source: `src/window/standard_window_function_expr.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Returns the ordering introduced by the window function, if applicable.
Most window functions don't introduce an ordering, hence the default
value is `None`. Note that this information is used to update ordering
equivalences.

<a id="op-01630d4bd4a562f086082e28"></a>
## limit_effect

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::limit_effect` · datafusion-physical-expr 55.1.0

```rust
fn limit_effect(&self) -> LimitEffect
```

Source: `src/window/standard_window_function_expr.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a06dfe79440b848793fed737"></a>
## name

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Source: `src/window/standard_window_function_expr.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Human readable name such as `"MIN(c2)"` or `"RANK()"`. The default
implementation returns placeholder text.

<a id="op-8f36b3a2fe7b1cf7ed94f336"></a>
## reverse_expr

`function` · `datafusion_physical_expr::window::standard_window_function_expr::StandardWindowFunctionExpr::reverse_expr` · datafusion-physical-expr 55.1.0

```rust
fn reverse_expr(&self) -> Option<Arc<dyn StandardWindowFunctionExpr>>
```

Source: `src/window/standard_window_function_expr.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Construct a new [`StandardWindowFunctionExpr`](../operations/datafusion_physical_expr.window.standard_window_function_expr.StandardWindowFunctionExpr.md#op-5882bec2eb25cba3cfad5983) that produces
the same result as this function on a window with reverse
order. The return value of this function is used by the
DataFusion optimizer to avoid re-sorting the data when
possible.

Returns `None` (the default) if no reverse is known (or possible).

For example, the reverse of `lead(10)` is `lag(10)`.
