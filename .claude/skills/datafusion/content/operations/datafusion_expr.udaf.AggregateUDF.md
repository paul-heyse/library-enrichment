# `datafusion_expr::udaf::AggregateUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.udaf.AggregateUDF.json).

<a id="op-d90e5a97479981a539718379"></a>
## AggregateUDF

`struct` · `datafusion_expr::udaf::AggregateUDF` · datafusion-expr 55.1.0

```rust
struct AggregateUDF
```

Source: `src/udaf.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Logical representation of a user-defined [aggregate function] (UDAF).

An aggregate function combines the values from multiple input rows
into a single output "aggregate" (summary) row. It is different
from a scalar function because it is stateful across batches. User
defined aggregate functions can be used as normal SQL aggregate
functions (`GROUP BY` clause) as well as window functions (`OVER`
clause).

`AggregateUDF` provides DataFusion the information needed to plan and call
aggregate functions, including name, type information, and a factory
function to create an [`Accumulator`] instance, to perform the actual
aggregation.

For more information, please see [the examples]:

1. For simple use cases, use [`create_udaf`] (examples in [`simple_udaf.rs`]).

2. For advanced use cases, use [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) which provides full API
   access (examples in [`advanced_udaf.rs`]).

# API Note
This is a separate struct from `AggregateUDFImpl` to maintain backwards
compatibility with the older API.

[the examples]: https://github.com/apache/datafusion/tree/main/datafusion-examples#single-process
[aggregate function]: https://en.wikipedia.org/wiki/Aggregate_function
[`Accumulator`]: Accumulator
[`create_udaf`]: crate::expr_fn::create_udaf
[`simple_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/simple_udaf.rs
[`advanced_udaf.rs`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/udf/advanced_udaf.rs

<a id="op-7705551e72842e6e0efbec66"></a>
## accumulator

`function` · `datafusion_expr::udaf::AggregateUDF::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return an accumulator the given aggregate, given its return datatype

<a id="op-3b1928ec6853177ba2e4ee7a"></a>
## aliases

`function` · `datafusion_expr::udaf::AggregateUDF::aliases` · datafusion-expr 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the aliases for this function.

<a id="op-57b20b11e75a0a2648e6877c"></a>
## call

`function` · `datafusion_expr::udaf::AggregateUDF::call` · datafusion-expr 55.1.0

```rust
fn call(&self, args: Vec<Expr>) -> Expr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Creates an [`Expr`](../operations/datafusion_expr.expr.Expr.md#op-230499d6f244cf7372db53bc) that calls the aggregate function.

This utility allows using the UDAF without requiring access to
the registry, such as with the DataFrame API.

<a id="op-320b1f7fa4140254a2fc4349"></a>
## clone

`function` · `datafusion_expr::udaf::AggregateUDF::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> AggregateUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 17], "end": [80, 22], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/udaf.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c38e79b264dce3df3e81ffb"></a>
## coerce_types

`function` · `datafusion_expr::udaf::AggregateUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "crate::AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [87, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec074b52a75a156da628c7ba"></a>
## coerce_types

`function` · `datafusion_expr::udaf::AggregateUDF::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:271`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce60bd953e339cf2751fb591"></a>
## create_groups_accumulator

`function` · `datafusion_expr::udaf::AggregateUDF::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::create_groups_accumulator`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-b55923cc32b806eb805e9ffb) for more details.

<a id="op-c62c0d977db155704554529f"></a>
## create_sliding_accumulator

`function` · `datafusion_expr::udaf::AggregateUDF::create_sliding_accumulator` · datafusion-expr 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:264`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4acfda07f65957e4e03b3c1b"></a>
## default_value

`function` · `datafusion_expr::udaf::AggregateUDF::default_value` · datafusion-expr 55.1.0

```rust
fn default_value(&self, data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:339`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::default_value`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-5ae1688167b50fd77316df64) for more details.

<a id="op-7d29c84d98fc93ee6626a868"></a>
## display_name

`function` · `datafusion_expr::udaf::AggregateUDF::display_name` · datafusion-expr 55.1.0

```rust
fn display_name(&self, params: &AggregateFunctionParams) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::display_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-1bb6dfbde26e11e415def938) for more details.

<a id="op-b7cc81836e6615fdab05c0ca"></a>
## documentation

`function` · `datafusion_expr::udaf::AggregateUDF::documentation` · datafusion-expr 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the documentation for this Aggregate UDF.

Documentation can be accessed programmatically as well as
generating publicly facing documentation.

<a id="op-f5889e1f577f22772bc15c90"></a>
## eq

`function` · `datafusion_expr::udaf::AggregateUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [89, 2], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udaf.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be3973251b40cbfb2322b426"></a>
## fmt

`function` · `datafusion_expr::udaf::AggregateUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [103, 2], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/udaf.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e514a6e553c1745ab4506a2b"></a>
## fmt

`function` · `datafusion_expr::udaf::AggregateUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 10], "end": [80, 15], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fcc21533295d5553f6caecd"></a>
## from

`function` · `datafusion_expr::udaf::AggregateUDF::from` · datafusion-expr 55.1.0

```rust
fn from(fun: F) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": [{"bound_predicate": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Send", "path": "Send"}}}, {"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": null, "id": "core::marker::Sync", "path": "Sync"}}}, {"outlives": "'static"}], "generic_params": [], "type": {"generic": "F"}}}]}, "is_negative": false, "span": {"begin": [362, 1], "end": [369, 2], "filename": "src/udaf.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/udaf.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a12d13554d4bbf824a5acbef"></a>
## groups_accumulator_supported

`function` · `datafusion_expr::udaf::AggregateUDF::groups_accumulator_supported` · datafusion-expr 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::groups_accumulator_supported`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-fbcc41eac02cc7099e2ddac5) for more details.

<a id="op-3fb037bd8558cb3182e9fe33"></a>
## hash

`function` · `datafusion_expr::udaf::AggregateUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [97, 2], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udaf.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65161d278b827d5714a0b1d2"></a>
## human_display

`function` · `datafusion_expr::udaf::AggregateUDF::human_display` · datafusion-expr 55.1.0

```rust
fn human_display(&self, params: &AggregateFunctionParams) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns a human readable expression.

See [`Expr::human_display`](../operations/datafusion_expr.expr.Expr.md#op-3d5b1a93f5851b5885d840ce) for details.

<a id="op-b9eb579534b339358233b0c9"></a>
## inner

`function` · `datafusion_expr::udaf::AggregateUDF::inner` · datafusion-expr 55.1.0

```rust
fn inner(&self) -> &Arc<dyn AggregateUDFImpl>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the underlying [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) trait object for this function

<a id="op-059345adb8b38eb2715abbe2"></a>
## is_descending

`function` · `datafusion_expr::udaf::AggregateUDF::is_descending` · datafusion-expr 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns true if the function is max, false if the function is min
None in all other cases, used in certain optimizations for
or aggregate

<a id="op-cb5c9132693c2f23fdc9a786"></a>
## is_nullable

`function` · `datafusion_expr::udaf::AggregateUDF::is_nullable` · datafusion-expr 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69893f01ad93eae277d8bf6f"></a>
## name

`function` · `datafusion_expr::udaf::AggregateUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "crate::AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [87, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-929a1ba07ae51fc17bcb0452"></a>
## name

`function` · `datafusion_expr::udaf::AggregateUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:172`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's name

See [`AggregateUDFImpl::name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-7844e1fa23821e46c7cabf21) for more details.

<a id="op-989937e262a3469060cc53fa"></a>
## new_from_impl

`function` · `datafusion_expr::udaf::AggregateUDF::new_from_impl` · datafusion-expr 55.1.0

```rust
fn new_from_impl<F>(fun: F) -> AggregateUDF where F: AggregateUDFImpl + 'static
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `AggregateUDF` from a `[AggregateUDFImpl]` trait object

Note this is the same as using the `From` impl (`AggregateUDF::from`)

<a id="op-349d213278064b1eb059eed3"></a>
## new_from_shared_impl

`function` · `datafusion_expr::udaf::AggregateUDF::new_from_shared_impl` · datafusion-expr 55.1.0

```rust
fn new_from_shared_impl(fun: Arc<dyn AggregateUDFImpl>) -> AggregateUDF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `AggregateUDF` from a `[AggregateUDFImpl]` trait object

<a id="op-973f26e995cca3a02b754557"></a>
## order_sensitivity

`function` · `datafusion_expr::udaf::AggregateUDF::order_sensitivity` · datafusion-expr 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Gets the order sensitivity of the UDF. See [`AggregateOrderSensitivity`](../operations/datafusion_functions_aggregate_common.order.AggregateOrderSensitivity.md#op-8b6a26410e0f5b4d59a68b96)
for possible options.

<a id="op-a50bf6ec25e02a88c0b023b2"></a>
## partial_cmp

`function` · `datafusion_expr::udaf::AggregateUDF::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &AggregateUDF) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [80, 24], "end": [80, 34], "filename": "src/udaf.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/udaf.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70767a00d8dae8458d3d330a"></a>
## return_field

`function` · `datafusion_expr::udaf::AggregateUDF::return_field` · datafusion-expr 55.1.0

```rust
fn return_field(&self, args: &[FieldRef]) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the field of the function given its input fields

See [`AggregateUDFImpl::return_field`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-bc99f6392eb81fae9a622a87) for more details.

<a id="op-39677e1123235753c3a3fc44"></a>
## return_type

`function` · `datafusion_expr::udaf::AggregateUDF::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, args: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:226`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the type of the function given its input types

See [`AggregateUDFImpl::return_type`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-02d75f6f8103226358991849) for more details.

<a id="op-917779edf46cc08ca893a1f8"></a>
## reverse_udf

`function` · `datafusion_expr::udaf::AggregateUDF::reverse_udf` · datafusion-expr 55.1.0

```rust
fn reverse_udf(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:294`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Reserves the `AggregateUDF` (e.g. returns the `AggregateUDF` that will
generate same result with this `AggregateUDF` when iterated in reverse
order, and `None` if there is no such `AggregateUDF`).

<a id="op-66fce6d1fc82006044ac57f5"></a>
## schema_name

`function` · `datafusion_expr::udaf::AggregateUDF::schema_name` · datafusion-expr 55.1.0

```rust
fn schema_name(&self, params: &AggregateFunctionParams) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::schema_name`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-2730cda6b3e779ac3cd35ede) for more details.

<a id="op-237a325d97e4745122f7b94d"></a>
## signature

`function` · `datafusion_expr::udaf::AggregateUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "crate::AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [87, 2], "filename": "src/type_coercion/functions.rs"}, "trait": {"args": null, "id": "datafusion_expr::type_coercion::functions::UDFCoercionExt", "path": "UDFCoercionExt"}, "trait_path": "datafusion_expr::type_coercion::functions::UDFCoercionExt"}`

Source: `src/type_coercion/functions.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64e9934ca38899b047046aa3"></a>
## signature

`function` · `datafusion_expr::udaf::AggregateUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this function's signature (what input types are accepted)

See [`AggregateUDFImpl::signature`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-130a8046da41201cb1a17c63) for more details.

<a id="op-ced58eeead0d857fa1f60b63"></a>
## simplify

`function` · `datafusion_expr::udaf::AggregateUDF::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<AggregateFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:301`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this aggregate function's simplification hook, if any.

See [`AggregateUDFImpl::simplify`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-97048a636c94a209ef572cb4) for more details.

<a id="op-6eb6c426f3db76a72bad50d8"></a>
## simplify_expr_op_literal

`function` · `datafusion_expr::udaf::AggregateUDF::simplify_expr_op_literal` · datafusion-expr 55.1.0

```rust
fn simplify_expr_op_literal(&self, agg_function: &AggregateFunction, arg: &Expr, op: Operator, lit: &Expr, arg_is_left: bool) -> Result<Option<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Rewrite aggregate to have simpler arguments

See  [`AggregateUDFImpl::simplify_expr_op_literal`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-491c7aa80585642342237af6) for more details

<a id="op-4db334fc26162f9f609bc469"></a>
## state_fields

`function` · `datafusion_expr::udaf::AggregateUDF::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the fields used to store the intermediate state for this aggregator, given
the name of the aggregate, value type and ordering fields. See [`AggregateUDFImpl::state_fields`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-5451e9f265ed39056933ce16)
for more details.

This is used to support multi-phase aggregations

<a id="op-7ebf88939605c495a4bd21bb"></a>
## supports_null_handling_clause

`function` · `datafusion_expr::udaf::AggregateUDF::supports_null_handling_clause` · datafusion-expr 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::supports_null_handling_clause`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-20c2d24527663a5d2f4968cb) for more details.

<a id="op-73dd58d5c2b6c734a0cbd245"></a>
## supports_within_group_clause

`function` · `datafusion_expr::udaf::AggregateUDF::supports_within_group_clause` · datafusion-expr 55.1.0

```rust
fn supports_within_group_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:349`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::supports_within_group_clause`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-2bf4f090182fa02ec5a2da5a) for more details.

<a id="op-cc9f0797bf3b99e0cdcc2726"></a>
## value_from_stats

`function` · `datafusion_expr::udaf::AggregateUDF::value_from_stats` · datafusion-expr 55.1.0

```rust
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return the value of this aggregate function if it can be determined
entirely from statistics and arguments.

See [`AggregateUDFImpl::value_from_stats`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-20017ef1e72ea9537862313e) for more details.

<a id="op-a50b6f4089a662c2e80d8894"></a>
## window_function_display_name

`function` · `datafusion_expr::udaf::AggregateUDF::window_function_display_name` · datafusion-expr 55.1.0

```rust
fn window_function_display_name(&self, params: &WindowFunctionParams) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33a41c71fa5566fd61e72513"></a>
## window_function_schema_name

`function` · `datafusion_expr::udaf::AggregateUDF::window_function_schema_name` · datafusion-expr 55.1.0

```rust
fn window_function_schema_name(&self, params: &WindowFunctionParams) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1cd6dab9cd8b201f718a491"></a>
## with_aliases

`function` · `datafusion_expr::udaf::AggregateUDF::with_aliases` · datafusion-expr 55.1.0

```rust
fn with_aliases(self, aliases: impl IntoIterator<Item = &'static str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Adds additional names that can be used to invoke this function, in
addition to `name`

If you implement [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) directly you should return aliases directly.

<a id="op-cbcdada130e7bf8988c81067"></a>
## with_beneficial_ordering

`function` · `datafusion_expr::udaf::AggregateUDF::with_beneficial_ordering` · datafusion-expr 55.1.0

```rust
fn with_beneficial_ordering(self, beneficial_ordering: bool) -> Result<Option<AggregateUDF>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [360, 2], "filename": "src/udaf.rs"}, "trait": null, "trait_path": null}`

Source: `src/udaf.rs:276`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

See [`AggregateUDFImpl::with_beneficial_ordering`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-7be7de13eb811e7ea01e8f98) for more details.
