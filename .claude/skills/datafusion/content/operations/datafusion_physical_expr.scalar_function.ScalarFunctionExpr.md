# `datafusion_physical_expr::scalar_function::ScalarFunctionExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.scalar_function.ScalarFunctionExpr.json).

<a id="op-36c7242f16be408f959deb9f"></a>
## ScalarFunctionExpr

`struct` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr` · datafusion-physical-expr 55.1.0

```rust
struct ScalarFunctionExpr
```

Source: `src/scalar_function.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Physical expression of a scalar function

<a id="op-85f1ea582a62940440f35411"></a>
## args

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::args` · datafusion-physical-expr 55.1.0

```rust
fn args(&self) -> &[Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Input arguments

<a id="op-6215ad0a669d3d4013470de5"></a>
## children

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::children` · datafusion-physical-expr 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a0642f84ad6bf3a542a9357"></a>
## config_options

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::config_options` · datafusion-physical-expr 55.1.0

```rust
fn config_options(&self) -> &ConfigOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2595d519d9f8f4945387c646"></a>
## data_type

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::data_type` · datafusion-physical-expr 55.1.0

```rust
fn data_type(&self, _input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1de22cb7c330c8156d128e20"></a>
## eq

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, o: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [203, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/scalar_function.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba410ed33e5f8c32c250b2a9"></a>
## evaluate

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::evaluate` · datafusion-physical-expr 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:236`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3785c26360a7f1be2833b3da"></a>
## evaluate_bounds

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::evaluate_bounds` · datafusion-physical-expr 55.1.0

```rust
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b1ed5c81e8ea4893048b1ea"></a>
## fmt

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [176, 1], "end": [180, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/scalar_function.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9aeea34728e2fdb023d49384"></a>
## fmt

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 1], "end": [69, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/scalar_function.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-652cfd1da02e8c1514333021"></a>
## fmt_sql

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::fmt_sql` · datafusion-physical-expr 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00487085ffb52c0c62699d56"></a>
## fun

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::fun` · datafusion-physical-expr 55.1.0

```rust
fn fun(&self) -> &ScalarUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Get the scalar function implementation

<a id="op-3a449952626e59ea6e9a3eed"></a>
## get_properties

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::get_properties` · datafusion-physical-expr 55.1.0

```rust
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:316`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec0eccbb05354d7ee4134c5c"></a>
## hash

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::hash` · datafusion-physical-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [219, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/scalar_function.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e860945738ade676b72ea42"></a>
## is_volatile_node

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::is_volatile_node` · datafusion-physical-expr 55.1.0

```rust
fn is_volatile_node(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4439bee583b707756d1489d"></a>
## name

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::name` · datafusion-physical-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

The name for this expression

<a id="op-7293d85894f40dfb26d24624"></a>
## new

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::new` · datafusion-physical-expr 55.1.0

```rust
fn new(name: &str, fun: Arc<ScalarUDF>, args: Vec<Arc<dyn PhysicalExpr>>, return_field: FieldRef, config_options: Arc<ConfigOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new Scalar function

<a id="op-47ffd2939aabdd463e19569c"></a>
## nullable

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f15f785fd84df870ea3a5ef4"></a>
## nullable

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::nullable` · datafusion-physical-expr 55.1.0

```rust
fn nullable(&self, _input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b8899ca12a2f81ef33198af"></a>
## placement

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::placement` · datafusion-physical-expr 55.1.0

```rust
fn placement(&self) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f3b4ebbdf085f289ea7161c"></a>
## propagate_constraints

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::propagate_constraints` · datafusion-physical-expr 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e5acadba5ca79f8b7d41c7d"></a>
## return_field

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::return_field` · datafusion-physical-expr 55.1.0

```rust
fn return_field(&self, _input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3042c8df2abba9e8808fcd4e"></a>
## return_type

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::return_type` · datafusion-physical-expr 55.1.0

```rust
fn return_type(&self) -> &DataType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Data type produced by this expression

<a id="op-073ea87a898795f8b8e5c7c3"></a>
## try_downcast_func

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::try_downcast_func` · datafusion-physical-expr 55.1.0

```rust
fn try_downcast_func<T>(expr: &dyn PhysicalExpr) -> Option<&ScalarFunctionExpr> where T: ScalarUDFImpl
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Given an arbitrary PhysicalExpr attempt to downcast it to a ScalarFunctionExpr
and verify that its inner function is of type T.
If the downcast fails, or the function is not of type T, returns `None`.
Otherwise returns `Some(ScalarFunctionExpr)`.

<a id="op-c52420e4500cfda0b43ea9a3"></a>
## try_new

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::try_new` · datafusion-physical-expr 55.1.0

```rust
fn try_new(fun: Arc<ScalarUDF>, args: Vec<Arc<dyn PhysicalExpr>>, schema: &Schema, config_options: Arc<ConfigOptions>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Create a new Scalar function

<a id="op-55adc3fc57d4697e03daa4a4"></a>
## with_new_children

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::with_new_children` · datafusion-physical-expr 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [227, 1], "end": [354, 2], "filename": "src/scalar_function.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/scalar_function.rs:291`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09427aafe63728b28f9d737e"></a>
## with_nullable

`function` · `datafusion_physical_expr::scalar_function::ScalarFunctionExpr::with_nullable` · datafusion-physical-expr 55.1.0

```rust
fn with_nullable(self, nullable: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::scalar_function::ScalarFunctionExpr", "path": "ScalarFunctionExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [174, 2], "filename": "src/scalar_function.rs"}, "trait": null, "trait_path": null}`

Source: `src/scalar_function.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
