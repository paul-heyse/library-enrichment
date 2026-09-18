# `datafusion_functions_aggregate::approx_distinct::ApproxDistinct`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_distinct.ApproxDistinct.json).

<a id="op-c9af29cd8e14fdb320d3e537"></a>
## ApproxDistinct

`struct` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct` · datafusion-functions-aggregate 55.1.0

```rust
struct ApproxDistinct
```

Source: `src/approx_distinct.rs:655`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5efab4eff3464a8a833a230a"></a>
## accumulator

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c6afdc779513e2ab69e9dc0"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:860`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-727fb3dfe0bf59acbcd26357"></a>
## default

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [634, 1], "end": [638, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/approx_distinct.rs:635`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99bc3bccf979b9f96d40b38f"></a>
## default_value

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::default_value` · datafusion-functions-aggregate 55.1.0

```rust
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:719`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae55a552641c321138ca3808"></a>
## documentation

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:874`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd93e0e2ef4f31d7325de88e"></a>
## eq

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &ApproxDistinct) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 10], "end": [654, 19], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/approx_distinct.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc77d9fd86a3eda4ac2f09d6"></a>
## fmt

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [625, 1], "end": [632, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/approx_distinct.rs:626`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0d51c58211b13853f3cd982"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:856`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91573d738b09245d4d3b7bcf"></a>
## hash

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [654, 25], "end": [654, 29], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/approx_distinct.rs:654`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46380b1214be1767e5e92895"></a>
## is_nullable

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::is_nullable` · datafusion-functions-aggregate 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:723`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a7603275107acfbd109f2b8"></a>
## name

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:707`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4f95ac3b62565b0d002fd83"></a>
## new

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [665, 2], "filename": "src/approx_distinct.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_distinct.rs:660`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea74dae240b2fa37873e77a3"></a>
## return_type

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:715`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-85cce3dd9337fae6e142cbd9"></a>
## signature

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f41dfc8f023868049d4c7683"></a>
## state_fields

`function` · `datafusion_functions_aggregate::approx_distinct::ApproxDistinct::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_distinct::ApproxDistinct", "path": "ApproxDistinct"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [706, 1], "end": [877, 2], "filename": "src/approx_distinct.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_distinct.rs:727`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
