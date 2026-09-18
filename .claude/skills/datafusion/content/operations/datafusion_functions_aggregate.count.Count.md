# `datafusion_functions_aggregate::count::Count`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.count.Count.json).

<a id="op-0e27ccb52355cdb161db5334"></a>
## Count

`struct` · `datafusion_functions_aggregate::count::Count` · datafusion-functions-aggregate 55.1.0

```rust
struct Count
```

Source: `src/count.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b3ff517694db844003c249e"></a>
## accumulator

`function` · `datafusion_functions_aggregate::count::Count::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ca1a70beeb9371e0048721"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::count::Count::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:368`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02f1ea2f35068c064692874a"></a>
## create_sliding_accumulator

`function` · `datafusion_functions_aggregate::count::Count::create_sliding_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:433`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422e156d102c3d92af22f159"></a>
## default

`function` · `datafusion_functions_aggregate::count::Count::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 1], "end": [165, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/count.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20ba10bc00e2e8c9d0187d35"></a>
## default_value

`function` · `datafusion_functions_aggregate::count::Count::default_value` · datafusion-functions-aggregate 55.1.0

```rust
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:382`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db2a57dc9894d09157d6b568"></a>
## documentation

`function` · `datafusion_functions_aggregate::count::Count::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:423`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf6c1ee7ad4e0c30378e8129"></a>
## eq

`function` · `datafusion_functions_aggregate::count::Count::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Count) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 10], "end": [156, 19], "filename": "src/count.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/count.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92c270d111a88877cf546d50"></a>
## fmt

`function` · `datafusion_functions_aggregate::count::Count::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 31], "end": [156, 36], "filename": "src/count.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/count.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af3400c4d20cf707ce1afc0f"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::count::Count::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:348`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-631d55b00d31fb7293a2341b"></a>
## hash

`function` · `datafusion_functions_aggregate::count::Count::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 25], "end": [156, 29], "filename": "src/count.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/count.rs:156`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea13fb835732877d4551ab8c"></a>
## is_nullable

`function` · `datafusion_functions_aggregate::count::Count::is_nullable` · datafusion-functions-aggregate 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:296`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55db726d9ccf8fbd47b07581"></a>
## name

`function` · `datafusion_functions_aggregate::count::Count::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad60c0bdf7a4a4f056b5b5fb"></a>
## new

`function` · `datafusion_functions_aggregate::count::Count::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [176, 2], "filename": "src/count.rs"}, "trait": null, "trait_path": null}`

Source: `src/count.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c9843aeb20a5523afe69bcf"></a>
## return_type

`function` · `datafusion_functions_aggregate::count::Count::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:292`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44e2e7d913cdf3966ab55820"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::count::Count::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:378`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-400bcaca676afaac1f3e3520"></a>
## set_monotonicity

`function` · `datafusion_functions_aggregate::count::Count::set_monotonicity` · datafusion-functions-aggregate 55.1.0

```rust
fn set_monotonicity(&self, _data_type: &DataType) -> SetMonotonicity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-adddecac984a490164662fd7"></a>
## signature

`function` · `datafusion_functions_aggregate::count::Count::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad7de25a1520dd9ffd274e07"></a>
## state_fields

`function` · `datafusion_functions_aggregate::count::Count::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c5c4003d78fbaa6c3838e6f"></a>
## value_from_stats

`function` · `datafusion_functions_aggregate::count::Count::value_from_stats` · datafusion-functions-aggregate 55.1.0

```rust
fn value_from_stats(&self, statistics_args: &StatisticsArgs<'_>) -> Option<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::count::Count", "path": "Count"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [446, 2], "filename": "src/count.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/count.rs:386`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
