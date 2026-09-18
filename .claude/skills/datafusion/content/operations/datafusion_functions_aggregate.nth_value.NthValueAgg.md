# `datafusion_functions_aggregate::nth_value::NthValueAgg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.nth_value.NthValueAgg.json).

<a id="op-34b49a6dadde0213adc1dfa2"></a>
## NthValueAgg

`struct` · `datafusion_functions_aggregate::nth_value::NthValueAgg` · datafusion-functions-aggregate 55.1.0

```rust
struct NthValueAgg
```

Source: `src/nth_value.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Expression for a `NTH_VALUE(..., ... ORDER BY ...)` aggregation. In a multi
partition setting, partial aggregations are computed for every partition,
and then their results are merged.

<a id="op-7d7f6cf9ea3eafa8d6a779b9"></a>
## accumulator

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89b4ca866806308a270621c7"></a>
## default

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [107, 1], "end": [111, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/nth_value.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9898dfd36e4d3050921a4964"></a>
## documentation

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:186`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c6b1b7bab7407bdf1d7e127"></a>
## eq

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &NthValueAgg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 17], "end": [93, 26], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/nth_value.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d406ecaf219dc56243cf664f"></a>
## fmt

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/nth_value.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e031ba8475d7e8effc9a65be"></a>
## hash

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 32], "end": [93, 36], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/nth_value.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8ce7ca5ad1c03a2317d351f"></a>
## name

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb5a7275bcd9ab7acc09b565"></a>
## new

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [98, 1], "end": [105, 2], "filename": "src/nth_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/nth_value.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new `NthValueAgg` aggregate function

<a id="op-94425edbf22de97f6a3b14ad"></a>
## return_type

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdbf56ba1ccb118cbacae6ee"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:182`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e507825f6c926a2b7a329634"></a>
## signature

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3f52413eb78f9f17e798749c"></a>
## state_fields

`function` · `datafusion_functions_aggregate::nth_value::NthValueAgg::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::nth_value::NthValueAgg", "path": "NthValueAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [113, 1], "end": [189, 2], "filename": "src/nth_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/nth_value.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
