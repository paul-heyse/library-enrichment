# `datafusion_functions_aggregate::variance::VarianceSample`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.variance.VarianceSample.json).

<a id="op-a430035d7c3eefc128e96f68"></a>
## VarianceSample

`struct` · `datafusion_functions_aggregate::variance::VarianceSample` · datafusion-functions-aggregate 55.1.0

```rust
struct VarianceSample
```

Source: `src/variance.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-549620bd3c563cef92fdedb8"></a>
## accumulator

`function` · `datafusion_functions_aggregate::variance::VarianceSample::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e00dc854f6e14dafb3e766c2"></a>
## aliases

`function` · `datafusion_functions_aggregate::variance::VarianceSample::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:135`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beca592dde06e577e559db79"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::variance::VarianceSample::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:143`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de9ad8d833f44801abcf3025"></a>
## default

`function` · `datafusion_functions_aggregate::variance::VarianceSample::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [75, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/variance.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f6c685b57b3d72505da6f8e"></a>
## documentation

`function` · `datafusion_functions_aggregate::variance::VarianceSample::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:150`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b89afb9e2dfc9ea3fc018f4e"></a>
## eq

`function` · `datafusion_functions_aggregate::variance::VarianceSample::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &VarianceSample) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 10], "end": [65, 19], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/variance.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eca7eef05ed7123264b2866a"></a>
## fmt

`function` · `datafusion_functions_aggregate::variance::VarianceSample::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 31], "end": [65, 36], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/variance.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6df79d955854fc2dd30721d"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::variance::VarianceSample::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:139`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f39f6762842eae84fbbf11f"></a>
## hash

`function` · `datafusion_functions_aggregate::variance::VarianceSample::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 25], "end": [65, 29], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/variance.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e7e936661c33194683b0d97"></a>
## name

`function` · `datafusion_functions_aggregate::variance::VarianceSample::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31b09f6260ea62d5b6a12560"></a>
## new

`function` · `datafusion_functions_aggregate::variance::VarianceSample::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [84, 2], "filename": "src/variance.rs"}, "trait": null, "trait_path": null}`

Source: `src/variance.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b13a8424329283e7fa257e77"></a>
## return_type

`function` · `datafusion_functions_aggregate::variance::VarianceSample::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5373eb4a9716788703b9f60a"></a>
## signature

`function` · `datafusion_functions_aggregate::variance::VarianceSample::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beb6a9a174bbff387d10b206"></a>
## state_fields

`function` · `datafusion_functions_aggregate::variance::VarianceSample::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::variance::VarianceSample", "path": "VarianceSample"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [153, 2], "filename": "src/variance.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/variance.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
