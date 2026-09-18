# `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.approx_percentile_cont_with_weight.ApproxPercentileContWithWeight.json).

<a id="op-08e8828c396c8730dcaae552"></a>
## ApproxPercentileContWithWeight

`struct` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight` · datafusion-functions-aggregate 55.1.0

```rust
struct ApproxPercentileContWithWeight
```

Source: `src/approx_percentile_cont_with_weight.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

APPROX_PERCENTILE_CONT_WITH_WEIGHT aggregate expression

<a id="op-54357b125e10153c334265e8"></a>
## accumulator

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cc821d632a78ef8bc449e9d"></a>
## default

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [123, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/approx_percentile_cont_with_weight.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-818e8c997f7666ecf929aa53"></a>
## documentation

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:281`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-be28828f82e69d204c36a08d"></a>
## eq

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &ApproxPercentileContWithWeight) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 10], "end": [113, 19], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/approx_percentile_cont_with_weight.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ae789d4ddef134d3882c725"></a>
## fmt

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 31], "end": [113, 36], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/approx_percentile_cont_with_weight.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9134a3372be564188c0fee4b"></a>
## hash

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 25], "end": [113, 29], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/approx_percentile_cont_with_weight.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9dd748d2642d5acd5f06932"></a>
## name

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e378d2901faefbcbd4cdcfea"></a>
## new

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [179, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": null, "trait_path": null}`

Source: `src/approx_percentile_cont_with_weight.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new [`ApproxPercentileContWithWeight`](../operations/datafusion_functions_aggregate.approx_percentile_cont_with_weight.ApproxPercentileContWithWeight.md#op-08e8828c396c8730dcaae552) aggregate function.

<a id="op-9e3b50630944d2207ef1fa11"></a>
## return_type

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:190`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-100a10102f250a856f4c7336"></a>
## signature

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-61ed20d8488d86e6808df644"></a>
## state_fields

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

See [`TDigest::to_scalar_state()`] for a description of the serialized
state.

Unresolved upstream links (retained, not inferred): ``TDigest::to_scalar_state()``.

<a id="op-b2c2ca5da4d6a30417cc1019"></a>
## supports_within_group_clause

`function` · `datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight::supports_within_group_clause` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_within_group_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::approx_percentile_cont_with_weight::ApproxPercentileContWithWeight", "path": "ApproxPercentileContWithWeight"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [181, 1], "end": [284, 2], "filename": "src/approx_percentile_cont_with_weight.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/approx_percentile_cont_with_weight.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
