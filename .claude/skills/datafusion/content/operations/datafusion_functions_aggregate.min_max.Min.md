# `datafusion_functions_aggregate::min_max::Min`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.min_max.Min.json).

<a id="op-4d2ab2fda80f84caf706a914"></a>
## Min

`struct` · `datafusion_functions_aggregate::min_max::Min` · datafusion-functions-aggregate 55.1.0

```rust
struct Min
```

Source: `src/min_max.rs:467`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e20f04cae6fb0adaf4257a4"></a>
## accumulator

`function` · `datafusion_functions_aggregate::min_max::Min::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:512`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f072609febd932343ae23eb"></a>
## coerce_types

`function` · `datafusion_functions_aggregate::min_max::Min::coerce_types` · datafusion-functions-aggregate 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:658`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa1a00581a9b48df29b712fb"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::min_max::Min::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31d94fb69a9f8bf182eeb4d1"></a>
## create_sliding_accumulator

`function` · `datafusion_functions_aggregate::min_max::Min::create_sliding_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:638`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-304cff6f0f586423fdc263bc"></a>
## default

`function` · `datafusion_functions_aggregate::min_max::Min::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [479, 1], "end": [483, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/min_max.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df3cc5b89af8a5fd93576102"></a>
## documentation

`function` · `datafusion_functions_aggregate::min_max::Min::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:666`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7f0ce1c21cc3298fad05df7"></a>
## eq

`function` · `datafusion_functions_aggregate::min_max::Min::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Min) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 17], "end": [466, 26], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/min_max.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da3201f2b40f9027053be98e"></a>
## fmt

`function` · `datafusion_functions_aggregate::min_max::Min::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 10], "end": [466, 15], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/min_max.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f00c8138c9015e65c36ad3cc"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::min_max::Min::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:518`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3fd0b0fbef38cac8bd84b2c"></a>
## hash

`function` · `datafusion_functions_aggregate::min_max::Min::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [466, 32], "end": [466, 36], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/min_max.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae6b88174ed102a189dbca49"></a>
## is_descending

`function` · `datafusion_functions_aggregate::min_max::Min::is_descending` · datafusion-functions-aggregate 55.1.0

```rust
fn is_descending(&self) -> Option<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:647`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0a8e675f6de09d061e560cb"></a>
## name

`function` · `datafusion_functions_aggregate::min_max::Min::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:500`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dbd794b7a02bf962795ae28"></a>
## new

`function` · `datafusion_functions_aggregate::min_max::Min::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [477, 2], "filename": "src/min_max.rs"}, "trait": null, "trait_path": null}`

Source: `src/min_max.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05cc9810061bcbb882568977"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::min_max::Min::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> datafusion_expr::utils::AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef594a5bd640f3638e80d8be"></a>
## return_type

`function` · `datafusion_functions_aggregate::min_max::Min::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:508`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-144016c601f18f2f598822b8"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::min_max::Min::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:662`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c54a0afe882264a6b345bb8"></a>
## set_monotonicity

`function` · `datafusion_functions_aggregate::min_max::Min::set_monotonicity` · datafusion-functions-aggregate 55.1.0

```rust
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:670`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95cfbdd4c0863e4f9ec2899a"></a>
## signature

`function` · `datafusion_functions_aggregate::min_max::Min::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:504`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63d184c7c5a4d3dffb303d44"></a>
## value_from_stats

`function` · `datafusion_functions_aggregate::min_max::Min::value_from_stats` · datafusion-functions-aggregate 55.1.0

```rust
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::min_max::Min", "path": "Min"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [499, 1], "end": [675, 2], "filename": "src/min_max.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/min_max.rs:651`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
