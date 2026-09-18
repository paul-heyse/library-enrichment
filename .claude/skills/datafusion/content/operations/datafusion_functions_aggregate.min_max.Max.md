# `datafusion_functions_aggregate::min_max::Max`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.min_max.Max.json).

<a id="op-dcc23b905cdaab3229e7ee16"></a>
## Max

`struct` · `datafusion_functions_aggregate::min_max::Max` · datafusion-functions-aggregate 55.1.0

```rust
struct Max
```

Source: `src/min_max.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f423315fcdcba63a2159b94b"></a>
## accumulator

`function` · `datafusion_functions_aggregate::min_max::Max::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3100edb176748daf09c37122"></a>
## coerce_types

`function` · `datafusion_functions_aggregate::min_max::Max::coerce_types` · datafusion-functions-aggregate 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fce8035f79e9c41579834d4"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::min_max::Max::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46f9d3872933e1cd64520dae"></a>
## create_sliding_accumulator

`function` · `datafusion_functions_aggregate::min_max::Max::create_sliding_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd72bd8a95b173097da336b0"></a>
## default

`function` · `datafusion_functions_aggregate::min_max::Max::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [112, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/min_max.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be5fa89a0be41194d1ba49b8"></a>
## documentation

`function` · `datafusion_functions_aggregate::min_max::Max::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:371`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce9612180446d427341db199"></a>
## eq

`function` · `datafusion_functions_aggregate::min_max::Max::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Max) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 26], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/min_max.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a99f4b571c0d07219521d814"></a>
## fmt

`function` · `datafusion_functions_aggregate::min_max::Max::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b5cd867c312f9e9f30ab6c2"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::min_max::Max::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:224`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e22dffeb839d4a16a8c276e9"></a>
## hash

`function` · `datafusion_functions_aggregate::min_max::Max::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 32], "end": [95, 36], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/min_max.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17b1cd65af3f8fc6c906214f"></a>
## is_descending

`function` · `datafusion_functions_aggregate::min_max::Max::is_descending` · datafusion-functions-aggregate 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:353`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7455507282802cddda012b9f"></a>
## name

`function` · `datafusion_functions_aggregate::min_max::Max::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1baed7b510270d919a6f9280"></a>
## new

`function` · `datafusion_functions_aggregate::min_max::Max::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [106, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-898c44808e8195b39b1b9b6d"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::min_max::Max::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> datafusion_expr::utils::AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:357`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c7f246bda1c7615f4a5124e"></a>
## return_type

`function` · `datafusion_functions_aggregate::min_max::Max::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-129d2110363dbd8fed0f1f35"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::min_max::Max::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:364`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab328878da4bd00e8a0c7374"></a>
## set_monotonicity

`function` · `datafusion_functions_aggregate::min_max::Max::set_monotonicity` · datafusion-functions-aggregate 55.1.0

```rust
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:375`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b78795aeb8f473e0fb048479"></a>
## signature

`function` · `datafusion_functions_aggregate::min_max::Max::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8171e218fef5697d29086f32"></a>
## value_from_stats

`function` · `datafusion_functions_aggregate::min_max::Max::value_from_stats` · datafusion-functions-aggregate 55.1.0

```rust
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Max", "path": "Max"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [205, 1], "end": [380, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
