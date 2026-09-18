# `datafusion_functions_aggregate::sum::Sum`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.sum.Sum.json).

<a id="op-9ee5b9b04b47c18c1a922241"></a>
## Sum

`struct` · `datafusion_functions_aggregate::sum::Sum` · datafusion-functions-aggregate 55.1.0

```rust
struct Sum
```

Source: `src/sum.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03491d6c5b212494ca8ae5a6"></a>
## accumulator

`function` · `datafusion_functions_aggregate::sum::Sum::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7194907275ed78f341aab6de"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::sum::Sum::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:308`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ece74c87172be3436efe178c"></a>
## create_sliding_accumulator

`function` · `datafusion_functions_aggregate::sum::Sum::create_sliding_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:323`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4247296df241cf5d12202029"></a>
## default

`function` · `datafusion_functions_aggregate::sum::Sum::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [217, 1], "end": [221, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/sum.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-afb2620cdc7c0b1593896bcb"></a>
## documentation

`function` · `datafusion_functions_aggregate::sum::Sum::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:359`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ccb7f28f8930ecf57d62f58"></a>
## eq

`function` · `datafusion_functions_aggregate::sum::Sum::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Sum) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 17], "end": [163, 26], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/sum.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f694d4761bbd9dd9f1cb2eca"></a>
## fmt

`function` · `datafusion_functions_aggregate::sum::Sum::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 10], "end": [163, 15], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/sum.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e314edf7cb489501c6b7920"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::sum::Sum::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:304`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebd15ab8723866d2ad187372"></a>
## hash

`function` · `datafusion_functions_aggregate::sum::Sum::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [163, 32], "end": [163, 36], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/sum.rs:163`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e11ad346701796a65e3c712"></a>
## name

`function` · `datafusion_functions_aggregate::sum::Sum::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29144848a4078333958b02ad"></a>
## new

`function` · `datafusion_functions_aggregate::sum::Sum::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [215, 2], "filename": "src/sum.rs"}, "trait": null, "trait_path": null}`

Source: `src/sum.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b1d7e0327ff7c1e567fe60f"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::sum::Sum::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:355`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bce1de6f81592e03f4c5c71b"></a>
## return_type

`function` · `datafusion_functions_aggregate::sum::Sum::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:232`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3df467ef8d6d5a5866cac18a"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::sum::Sum::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:351`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14ab5c4b33f27203191d49c4"></a>
## set_monotonicity

`function` · `datafusion_functions_aggregate::sum::Sum::set_monotonicity` · datafusion-functions-aggregate 55.1.0

```rust
fn set_monotonicity(&self, data_type: &DataType) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:363`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06fbd4de683146a3831c5dc2"></a>
## signature

`function` · `datafusion_functions_aggregate::sum::Sum::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:228`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64ddde6959570b7a4e240ce6"></a>
## simplify_expr_op_literal

`function` · `datafusion_functions_aggregate::sum::Sum::simplify_expr_op_literal` · datafusion-functions-aggregate 55.1.0

```rust
fn simplify_expr_op_literal(&self, agg_function: &AggregateFunction, arg: &Expr, op: Operator, lit: &Expr, _arg_is_left: bool) -> Result<Option<Expr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:379`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Implement ClickBench Q29 specific optimization:
`SUM(arg + constant)` --> `SUM(arg) + constant * COUNT(arg)`

See background on [`AggregateUDFImpl::simplify_expr_op_literal`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-491c7aa80585642342237af6)

<a id="op-41a02fec5167d58502c125ad"></a>
## state_fields

`function` · `datafusion_functions_aggregate::sum::Sum::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0470935681a2e2b37f063c23"></a>
## value_from_stats

`function` · `datafusion_functions_aggregate::sum::Sum::value_from_stats` · datafusion-functions-aggregate 55.1.0

```rust
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::sum::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [223, 1], "end": [467, 2], "filename": "src/sum.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/sum.rs:416`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
