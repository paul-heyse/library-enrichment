# `datafusion_functions_aggregate::string_agg::StringAgg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.string_agg.StringAgg.json).

<a id="op-70824892be80de37d65aeea4"></a>
## StringAgg

`struct` · `datafusion_functions_aggregate::string_agg::StringAgg` · datafusion-functions-aggregate 55.1.0

```rust
struct StringAgg
```

Source: `src/string_agg.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

STRING_AGG aggregate expression

<a id="op-043503c4742e168e5c9c93b9"></a>
## accumulator

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5911be90e469a4de0bcb82e7"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:222`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcee98ffba834988e333d280"></a>
## default

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [147, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string_agg.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c367de5c013fc31ca90b97c5"></a>
## documentation

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:230`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f8305cb863692d8c4c2350d"></a>
## eq

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &StringAgg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 17], "end": [90, 26], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string_agg.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b6cebecbc09e50e78dd0b37"></a>
## fmt

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 10], "end": [90, 15], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string_agg.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57567d0739b159a69866cc04"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20e4409be6944b4f828d7fe8"></a>
## hash

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [90, 32], "end": [90, 36], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string_agg.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91dc52a7d8fcca72ed2d2075"></a>
## name

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:154`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9c1f4bca941f9907efd6ff50"></a>
## new

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [141, 2], "filename": "src/string_agg.rs"}, "trait": null, "trait_path": null}`

Source: `src/string_agg.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new StringAgg aggregate function

<a id="op-92695d0b03343128998eca2c"></a>
## return_type

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2b6ee3ddf683bcddeed4301"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> datafusion_expr::ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9c456b2ea1d7509ed8e555d"></a>
## signature

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05c5328237b556f905f9c891"></a>
## state_fields

`function` · `datafusion_functions_aggregate::string_agg::StringAgg::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::string_agg::StringAgg", "path": "StringAgg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [153, 1], "end": [233, 2], "filename": "src/string_agg.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/string_agg.rs:166`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
