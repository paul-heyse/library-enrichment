# `datafusion_functions_aggregate::array_agg::ArrayAgg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.array_agg.ArrayAgg.json).

<a id="op-051f6a479dc9cc7aa73e47ab"></a>
## ArrayAgg

`struct` · `datafusion_functions_aggregate::array_agg::ArrayAgg` · datafusion-functions-aggregate 55.1.0

```rust
struct ArrayAgg
```

Source: `src/array_agg.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

ARRAY_AGG aggregate expression

<a id="op-c9c3a179cee3e8355232b54e"></a>
## accumulator

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ae057dd994c30e62664b27"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7b82d917afe678d8a58f89e"></a>
## default

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [102, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/array_agg.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c90281d79b1c8e1e24a5dad9"></a>
## documentation

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e6d9586d8781a28998a1361"></a>
## eq

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &ArrayAgg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 17], "end": [88, 26], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/array_agg.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1ac26b8d595c3c4b23abb5d"></a>
## fmt

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 10], "end": [88, 15], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/array_agg.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-833badf194c8c00cacca8038"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-189c4aecae707da3c9e8e7fd"></a>
## hash

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 32], "end": [88, 36], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/array_agg.rs:88`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b1a767177de673693d011c3"></a>
## name

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-456a39ab2e88941f4669c9fa"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:160`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea98c325d6720a99bf32f443"></a>
## return_type

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-773c88390d11458b6b3dad45"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f1ce98419c8673d1de7fe18"></a>
## signature

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b5503229739d10643389c98"></a>
## state_fields

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f2a4b487bc5c6a456965f8c"></a>
## supports_null_handling_clause

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::supports_null_handling_clause` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ff9cb075f2f1ea6b6a1f935"></a>
## with_beneficial_ordering

`function` · `datafusion_functions_aggregate::array_agg::ArrayAgg::with_beneficial_ordering` · datafusion-functions-aggregate 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::array_agg::ArrayAgg", "path": "ArrayAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [104, 1], "end": [261, 2], "filename": "src/array_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/array_agg.rs:164`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
