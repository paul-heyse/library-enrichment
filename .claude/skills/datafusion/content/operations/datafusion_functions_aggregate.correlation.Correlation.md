# `datafusion_functions_aggregate::correlation::Correlation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.correlation.Correlation.json).

<a id="op-3fd6ac9849edb93926f71627"></a>
## Correlation

`struct` · `datafusion_functions_aggregate::correlation::Correlation` · datafusion-functions-aggregate 55.1.0

```rust
struct Correlation
```

Source: `src/correlation.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb04d1eb08b48a0a08d91514"></a>
## accumulator

`function` · `datafusion_functions_aggregate::correlation::Correlation::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba9d3afb1ba365ecad35ff7a"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::correlation::Correlation::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9797f2226f1c27b8b833279"></a>
## default

`function` · `datafusion_functions_aggregate::correlation::Correlation::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 1], "end": [81, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/correlation.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63a79c35847de548032218d5"></a>
## documentation

`function` · `datafusion_functions_aggregate::correlation::Correlation::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-409130716584254bf70a0177"></a>
## eq

`function` · `datafusion_functions_aggregate::correlation::Correlation::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Correlation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 17], "end": [72, 26], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/correlation.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2b2603c5aea29602e34108c"></a>
## fmt

`function` · `datafusion_functions_aggregate::correlation::Correlation::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 10], "end": [72, 15], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/correlation.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a188e8d723afc8d03d46cb87"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::correlation::Correlation::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:137`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dd7ba987ac85a2b3db521c9"></a>
## hash

`function` · `datafusion_functions_aggregate::correlation::Correlation::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 32], "end": [72, 36], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/correlation.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-288426d182f66a6fe97efc9b"></a>
## name

`function` · `datafusion_functions_aggregate::correlation::Correlation::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59972efe8badedd9efc14644"></a>
## new

`function` · `datafusion_functions_aggregate::correlation::Correlation::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [83, 1], "end": [95, 2], "filename": "src/correlation.rs"}, "trait": null, "trait_path": null}`

Source: `src/correlation.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new CORR aggregate function

<a id="op-3f07648c00c77768b6e86db9"></a>
## return_type

`function` · `datafusion_functions_aggregate::correlation::Correlation::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-888970349cc8e35f72cb26dd"></a>
## signature

`function` · `datafusion_functions_aggregate::correlation::Correlation::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc4253ba321f8f8cdfb39941"></a>
## state_fields

`function` · `datafusion_functions_aggregate::correlation::Correlation::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::correlation::Correlation", "path": "Correlation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [148, 2], "filename": "src/correlation.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/correlation.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
