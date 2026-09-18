# `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.higher_order_function.HigherOrderFunctionExpr.json).

<a id="op-1925d2dfc3f5a8943cb4dfb9"></a>
## HigherOrderFunctionExpr

`struct` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr` · datafusion-physical-expr 55.1.0

```rust
struct HigherOrderFunctionExpr
```

Source: `src/higher_order_function.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical expression of a higher order function

<a id="op-9a39073b7bfaa2a28adb3004"></a>
## args

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::args` · datafusion-physical-expr 55.1.0

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:185`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Input arguments

<a id="op-ed3b33423123299105a534e3"></a>
## children

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:421`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f56e9e7f65d23f51f2f4ad32"></a>
## config_options

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::config_options` · datafusion-physical-expr 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:198`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e9a397864ae7113f5d363db"></a>
## eq

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, o: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [246, 1], "end": [270, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/higher_order_function.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00088e1fe85202116cf8f70a"></a>
## evaluate

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-820c8e04b1ff57745123c628"></a>
## fmt

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [240, 1], "end": [244, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/higher_order_function.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7702d5371e8032c13e81886"></a>
## fmt

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [120, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/higher_order_function.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5949aee480420f41002a6d91"></a>
## fmt_sql

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:474`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39af334f7d219747faf3254a"></a>
## fun

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::fun` · datafusion-physical-expr 55.1.0

```rust
fn fun(&self) -> &HigherOrderUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the higher order function implementation

<a id="op-64006c7b36e9bdd5e301cd81"></a>
## hash

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [272, 1], "end": [287, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/higher_order_function.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44af5034ad4b0d312b765b9b"></a>
## is_volatile_node

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::is_volatile_node` · datafusion-physical-expr 55.1.0

```rust
fn is_volatile_node(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:485`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50ef5f27948883862b6ffb2a"></a>
## name

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:180`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The name for this expression

<a id="op-4372416665399c41c64673cb"></a>
## nullable

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:194`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2842032b5d0b061fa10dabff"></a>
## return_field

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:417`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d596afc9571fe88900395e1"></a>
## return_type

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::return_type` · datafusion-physical-expr 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Data type produced by this expression

<a id="op-dc132e1a5198906feaa9703d"></a>
## try_new_with_schema

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::try_new_with_schema` · datafusion-physical-expr 55.1.0

```rust
fn try_new_with_schema(fun: Arc<HigherOrderUDF>, args: Vec<Arc<dyn PhysicalExpr>>, schema: &Schema, config_options: Arc<ConfigOptions>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [238, 2], "filename": "src/higher_order_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/higher_order_function.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new Higher Order function

Note that lambda arguments must be present directly in args as [LambdaExpr](../operations/datafusion_physical_expr.expressions.lambda.LambdaExpr.md#op-a16ba2aae0fe27d74fa09d31),
and not as a wrapped child of any arg

<a id="op-ca12838b775d7c435135f2b8"></a>
## with_new_children

`function` · `datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::higher_order_function::HigherOrderFunctionExpr", "path": "HigherOrderFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [488, 2], "filename": "src/higher_order_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/higher_order_function.rs:425`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
