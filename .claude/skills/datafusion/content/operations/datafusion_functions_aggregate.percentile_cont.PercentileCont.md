# `datafusion_functions_aggregate::percentile_cont::PercentileCont`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.percentile_cont.PercentileCont.json).

<a id="op-fdfc3e38e1fc56e24a3ebc9f"></a>
## PercentileCont

`struct` · `datafusion_functions_aggregate::percentile_cont::PercentileCont` · datafusion-functions-aggregate 55.1.0

```rust
struct PercentileCont
```

Source: `src/percentile_cont.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

PERCENTILE_CONT aggregate expression. This uses an exact calculation and stores all values
in memory before computing the result. If an approximation is sufficient then
APPROX_PERCENTILE_CONT provides a much more efficient solution.

If using the distinct variation, the memory usage will be similarly high if the
cardinality is high as it stores all distinct values in memory before computing the
result, but if cardinality is low then memory usage will also be lower.

<a id="op-7912eb40cc8a59c5f2b1108f"></a>
## accumulator

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0e0e2cd33d62824fd2d218b"></a>
## aliases

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b76aca316541db458db3eb0"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:259`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a97dadc5926ff11a5cf3be87"></a>
## default

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [137, 1], "end": [141, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/percentile_cont.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64937443f1292de41b8b6ece"></a>
## documentation

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:290`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3728a53c854fd93828ed69f2"></a>
## eq

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &PercentileCont) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 10], "end": [131, 19], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/percentile_cont.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fb8f1871fcc763aaf56f9aa"></a>
## fmt

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 31], "end": [131, 36], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/percentile_cont.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a084fe417699b9882ef2b24e"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:255`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb98d0f423ce1b42538dfc95"></a>
## hash

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [131, 25], "end": [131, 29], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/percentile_cont.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a45ee8cce732468a505dd0fb"></a>
## name

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6aa0353b5d705918b61977"></a>
## new

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 1], "end": [166, 2], "filename": "src/percentile_cont.rs"}, "trait": null, "trait_path": null}`

Source: `src/percentile_cont.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72064f2606b1c104f88004a5"></a>
## return_type

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-620e734119f7dbcd2bb0a46d"></a>
## signature

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:177`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ffb0a97a5b3f54d1d98a4275"></a>
## simplify

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::simplify` · datafusion-functions-aggregate 55.1.0

```rust
fn simplify(&self) -> Option<AggregateFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e25a7d4a72524709fe8f17a5"></a>
## state_fields

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:188`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0a2492b7f409243a3f9f0ff"></a>
## supports_within_group_clause

`function` · `datafusion_functions_aggregate::percentile_cont::PercentileCont::supports_within_group_clause` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_within_group_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::percentile_cont::PercentileCont", "path": "PercentileCont"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 1], "end": [293, 2], "filename": "src/percentile_cont.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/percentile_cont.rs:286`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
