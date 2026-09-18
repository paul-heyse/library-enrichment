# `datafusion_physical_expr::aggregate::AggregateExprBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.aggregate.AggregateExprBuilder.json).

<a id="op-65df368c42420e560d9b75ee"></a>
## AggregateExprBuilder

`struct` · `datafusion_physical_expr::aggregate::AggregateExprBuilder` · datafusion-physical-expr 55.1.0

```rust
struct AggregateExprBuilder
```

Source: `src/aggregate.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Builder for physical [`AggregateFunctionExpr`](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-a082484e03fcd21a48e2b705)

`AggregateFunctionExpr` contains the information necessary to call
an aggregate expression.

<a id="op-dc01e13d32c0e5625f12000b"></a>
## alias

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::alias` · datafusion-physical-expr 55.1.0

```rust
fn alias(self, alias: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:325`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf95378a8e849472381db4c5"></a>
## build

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::build` · datafusion-physical-expr 55.1.0

```rust
fn build(self) -> Result<AggregateFunctionExpr>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Constructs an `AggregateFunctionExpr` from the builder

Note that an [`Self::alias`](../operations/datafusion_physical_expr.aggregate.AggregateExprBuilder.md#op-dc01e13d32c0e5625f12000b) must be provided before calling this method.

# Example: Create an [`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379)

In the following example, [`AggregateFunctionExpr`](../operations/datafusion_physical_expr.aggregate.AggregateFunctionExpr.md#op-a082484e03fcd21a48e2b705) will be built using [`AggregateExprBuilder`](../operations/datafusion_physical_expr.aggregate.AggregateExprBuilder.md#op-65df368c42420e560d9b75ee)
which provides a build function. Full example could be accessed from the source file.

```
# use std::any::Any;
# use std::sync::Arc;
# use arrow::datatypes::{DataType, FieldRef};
# use datafusion_common::{Result, ScalarValue};
# use datafusion_expr::{col, ColumnarValue, Documentation, Signature, Volatility, Expr};
# use datafusion_expr::{AggregateUDFImpl, AggregateUDF, Accumulator, function::{AccumulatorArgs, StateFieldsArgs}};
# use arrow::datatypes::Field;
#
# #[derive(Debug, Clone, PartialEq, Eq, Hash)]
# struct FirstValueUdf {
#     signature: Signature,
# }
#
# impl FirstValueUdf {
#     fn new() -> Self {
#         Self {
#             signature: Signature::any(1, Volatility::Immutable),
#         }
#     }
# }
#
# impl AggregateUDFImpl for FirstValueUdf {
#     fn name(&self) -> &str {
#         unimplemented!()
#     }
#
#     fn signature(&self) -> &Signature {
#         unimplemented!()
#     }
#
#     fn return_type(&self, args: &[DataType]) -> Result<DataType> {
#         unimplemented!()
#     }
#
#     fn accumulator(&self, acc_args: AccumulatorArgs) -> Result<Box<dyn Accumulator>> {
#         unimplemented!()
#         }
#
#     fn state_fields(&self, args: StateFieldsArgs) -> Result<Vec<FieldRef>> {
#         unimplemented!()
#     }
#
#     fn documentation(&self) -> Option<&Documentation> {
#         unimplemented!()
#     }
# }
#
# let first_value = AggregateUDF::from(FirstValueUdf::new());
# let expr = first_value.call(vec![col("a")]);
#
# use datafusion_physical_expr::expressions::Column;
# use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
# use datafusion_physical_expr::aggregate::AggregateExprBuilder;
# use datafusion_physical_expr::expressions::PhysicalSortExpr;
# use datafusion_physical_expr::PhysicalSortRequirement;
#
fn build_aggregate_expr() -> Result<()> {
    let args = vec![Arc::new(Column::new("a", 0)) as Arc<dyn PhysicalExpr>];
    let order_by = vec![PhysicalSortExpr {
        expr: Arc::new(Column::new("x", 1)) as Arc<dyn PhysicalExpr>,
        options: Default::default(),
    }];

    let first_value = AggregateUDF::from(FirstValueUdf::new());

    let aggregate_expr = AggregateExprBuilder::new(
        Arc::new(first_value),
        args
    )
    .order_by(order_by)
    .alias("first_a_by_x")
    .ignore_nulls()
    .build()?;

    Ok(())
}
```

This creates a physical expression equivalent to SQL:
`first_value(a ORDER BY x) IGNORE NULLS AS first_a_by_x`

<a id="op-d5c111f742a45b875e87e8f0"></a>
## clone

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> AggregateExprBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 17], "end": [121, 22], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/aggregate.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1326c5e5d11dfabd3e1b1f2"></a>
## distinct

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::distinct` · datafusion-physical-expr 55.1.0

```rust
fn distinct(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e274b61b35cd8972bea350"></a>
## fmt

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 10], "end": [121, 15], "filename": "src/aggregate.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/aggregate.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2e7b153db74063245affbd3"></a>
## human_display

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::human_display` · datafusion-physical-expr 55.1.0

```rust
fn human_display(self, name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:335`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89d58146d9626999175d1ca7"></a>
## ignore_nulls

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::ignore_nulls` · datafusion-physical-expr 55.1.0

```rust
fn ignore_nulls(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:381`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd080aa36e28dfd9ba23fe3a"></a>
## new

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::new` · datafusion-physical-expr 55.1.0

```rust
fn new(fun: Arc<AggregateUDF>, args: Vec<Arc<dyn PhysicalExpr>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66ac5023d9bb9765e5c07f13"></a>
## order_by

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::order_by` · datafusion-physical-expr 55.1.0

```rust
fn order_by(self, order_bys: Vec<PhysicalSortExpr>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:356`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd92b7d68debf93ad3fc61e3"></a>
## reversed

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::reversed` · datafusion-physical-expr 55.1.0

```rust
fn reversed(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33a75f7fb466fc319f3e6335"></a>
## schema

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::schema` · datafusion-physical-expr 55.1.0

```rust
fn schema(self, schema: SchemaRef) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d233106a88d013d84870bbac"></a>
## with_distinct

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::with_distinct` · datafusion-physical-expr 55.1.0

```rust
fn with_distinct(self, is_distinct: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:376`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b432af8f526049f3b47e681d"></a>
## with_ignore_nulls

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::with_ignore_nulls` · datafusion-physical-expr 55.1.0

```rust
fn with_ignore_nulls(self, ignore_nulls: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-591d811256f9441abaacd92b"></a>
## with_reversed

`function` · `datafusion_physical_expr::aggregate::AggregateExprBuilder::with_reversed` · datafusion-physical-expr 55.1.0

```rust
fn with_reversed(self, is_reversed: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::aggregate::AggregateExprBuilder", "path": "AggregateExprBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 1], "end": [390, 2], "filename": "src/aggregate.rs"}, "trait": null, "trait_path": null}`

Source: `src/aggregate.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
