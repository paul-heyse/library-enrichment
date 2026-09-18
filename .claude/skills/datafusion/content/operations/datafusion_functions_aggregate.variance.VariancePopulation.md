# `datafusion_functions_aggregate::variance::VariancePopulation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.VariancePopulation.json).

<a id="op-2aba8181d0116c756b60f60f"></a>
## VariancePopulation

`struct` · `datafusion_functions_aggregate::variance::VariancePopulation` · datafusion-functions-aggregate 55.1.0

```rust
struct VariancePopulation
```

Source: `src/variance.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d84bd2f1560edc14b3767528"></a>
## accumulator

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02daebb55d2a37e0536f20e9"></a>
## aliases

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bbe415028f6a7f9a016fe32"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0288d1abe5c515994ba32cea"></a>
## default

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [167, 1], "end": [171, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/variance.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-439c999612b84d21323065a7"></a>
## documentation

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af81b4628cab250109f1d988"></a>
## eq

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &VariancePopulation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 10], "end": [161, 19], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variance.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-65249ab072b5566d9fd4a865"></a>
## fmt

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 31], "end": [161, 36], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variance.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5897e54028f6a60950ba9251"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64b2b77b549d8271e7cf1b33"></a>
## hash

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [161, 25], "end": [161, 29], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/variance.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a166274efb74821038ccbfb8"></a>
## name

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a2c79e0acfa94173a045613"></a>
## new

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [180, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cbbbd2c9d0e2ebd146c767e"></a>
## return_type

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c34f77a574abcf9e09fd47eb"></a>
## signature

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-41236089a7d3745a1d6a71d8"></a>
## state_fields

`function` · `datafusion_functions_aggregate::variance::VariancePopulation::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VariancePopulation", "path": "VariancePopulation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [182, 1], "end": [255, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
