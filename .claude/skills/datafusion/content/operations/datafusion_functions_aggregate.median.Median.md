# `datafusion_functions_aggregate::median::Median`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.median.Median.json).

<a id="op-eaeb73dcd397d8bb72beb246"></a>
## Median

`struct` · `datafusion_functions_aggregate::median::Median` · datafusion-functions-aggregate 55.1.0

```rust
struct Median
```

Source: `src/median.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

MEDIAN aggregate expression. If using the non-distinct variation, then this uses a
lot of memory because all values need to be stored in memory before a result can be
computed. If an approximation is sufficient then APPROX_MEDIAN provides a much more
efficient solution.

If using the distinct variation, the memory usage will be similarly high if the
cardinality is high as it stores all distinct values in memory before computing the
result, but if cardinality is low then memory usage will also be lower.

<a id="op-e2a11af33ed0c9f3b998d2a2"></a>
## accumulator

`function` · `datafusion_functions_aggregate::median::Median::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:171`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c8648336647217b8a6e1213"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::median::Median::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ff0b090ee041ab161bb67a9"></a>
## default

`function` · `datafusion_functions_aggregate::median::Median::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [100, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/median.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-622cc733817bd339dd7b2e76"></a>
## documentation

`function` · `datafusion_functions_aggregate::median::Median::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e42f03dabd3fec8706622481"></a>
## eq

`function` · `datafusion_functions_aggregate::median::Median::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Median) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 10], "end": [91, 19], "filename": "src/median.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/median.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28aa785af574f0b6b73418e2"></a>
## fmt

`function` · `datafusion_functions_aggregate::median::Median::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 31], "end": [91, 36], "filename": "src/median.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/median.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcfecf3e5748b0b17e6e82d5"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::median::Median::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:210`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95aba1d6ff8852332013b34a"></a>
## hash

`function` · `datafusion_functions_aggregate::median::Median::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [91, 25], "end": [91, 29], "filename": "src/median.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/median.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb3d7cac0002e2908db325f2"></a>
## name

`function` · `datafusion_functions_aggregate::median::Median::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0f79b3f28d96b1675b995a7"></a>
## new

`function` · `datafusion_functions_aggregate::median::Median::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [126, 2], "filename": "src/median.rs"}, "trait": null, "trait_path": null}`

Source: `src/median.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5da8a685da2728670d38e278"></a>
## return_type

`function` · `datafusion_functions_aggregate::median::Median::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc68cbb67f169c616d095518"></a>
## signature

`function` · `datafusion_functions_aggregate::median::Median::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-123e5e4cf8dbe03ef4a8c464"></a>
## state_fields

`function` · `datafusion_functions_aggregate::median::Median::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::median::Median", "path": "Median"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [128, 1], "end": [254, 2], "filename": "src/median.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/median.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
